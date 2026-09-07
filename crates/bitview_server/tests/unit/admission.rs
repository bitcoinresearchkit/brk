use std::{sync::mpsc, time::Duration};

use axum::{
    body::Bytes,
    http::{HeaderMap, StatusCode, header::IF_NONE_MATCH},
};
use bitview_query::RepresentationId;
use brk_types::Version;
use tokio::{spawn, sync::oneshot, time::timeout};

use super::chain_fixture::run;
use crate::{CacheParams, CacheStrategy};

#[test]
fn bound_responses_preserve_body_identity_and_validate_before_revalidation() {
    run(|state, _| async move {
        for identity in [
            RepresentationId::content(b"[]"),
            RepresentationId::Block(state.sync(|query| query.tip_blockhash())),
        ] {
            let params = CacheParams::resolve(
                &crate::AppState::representation_strategy(Version::ONE, identity),
                state.cdn_cache_mode,
            );
            for conditional in [false, true] {
                let mut headers = HeaderMap::new();
                if conditional {
                    headers.insert(IF_NONE_MATCH, "*".parse().unwrap());
                }
                let response = state
                    .respond_json_bound(&headers, Version::ONE, move |_| {
                        Ok((b"[]".to_vec(), identity))
                    })
                    .await;
                assert_eq!(
                    response.status(),
                    if conditional {
                        StatusCode::NOT_MODIFIED
                    } else {
                        StatusCode::OK
                    }
                );
                let mut expected = HeaderMap::new();
                params.apply_to(&mut expected);
                for (name, value) in &expected {
                    assert_eq!(response.headers().get(name), Some(value));
                }
                assert_eq!(
                    response.headers().contains_key("content-type"),
                    !conditional
                );
                let bytes = axum::body::to_bytes(response.into_body(), 1024)
                    .await
                    .unwrap();
                assert_eq!(
                    &bytes[..],
                    if conditional {
                        b"".as_slice()
                    } else {
                        b"[]".as_slice()
                    }
                );

                let error = state
                    .respond_json_bound(&headers, Version::ONE, |_| {
                        Err(brk_error::Error::StateUpdating)
                    })
                    .await;
                assert_eq!(error.status(), StatusCode::SERVICE_UNAVAILABLE);
                assert!(!error.headers().contains_key("etag"));
                assert_eq!(error.headers()["cache-control"], "no-store");
            }
        }
    });
}

#[test]
fn generic_body_jobs_remain_admitted_after_cancellation() {
    run(|state, _| async move {
        for bound in [false, true] {
            let (started, ready) = oneshot::channel();
            let (release, blocked) = mpsc::channel();
            let first_state = state.clone();
            let first = spawn(async move {
                if bound {
                    first_state
                        .respond_json_bound(&HeaderMap::new(), Version::ONE, move |_| {
                            started.send(()).unwrap();
                            blocked.recv().unwrap();
                            Ok((b"[]".to_vec(), RepresentationId::content(b"[]")))
                        })
                        .await
                } else {
                    first_state
                        .respond_json_bytes(
                            &HeaderMap::new(),
                            CacheStrategy::Immutable(Version::ONE),
                            move |_| {
                                started.send(()).unwrap();
                                blocked.recv().unwrap();
                                Ok(b"[]".to_vec())
                            },
                        )
                        .await
                }
            });
            timeout(Duration::from_secs(2), ready)
                .await
                .unwrap()
                .unwrap();
            first.abort();
            let _ = first.await;
            assert_eq!(state.sync_query.available_permits(), 0);

            // Already-known validators must not wait for a body permit.
            let mut headers = HeaderMap::new();
            headers.insert(IF_NONE_MATCH, "*".parse().unwrap());
            let cached = timeout(
                Duration::from_millis(100),
                state.respond_json_bytes(&headers, CacheStrategy::Immutable(Version::ONE), |_| {
                    panic!("304 built a body")
                }),
            )
            .await
            .unwrap();
            assert_eq!(cached.status(), StatusCode::NOT_MODIFIED);

            let next_state = state.clone();
            let mut next = spawn(async move {
                next_state
                    .respond_with_params(
                        &HeaderMap::new(),
                        CacheParams::deploy(),
                        |_| {},
                        |_| Ok(Bytes::from_static(b"[]")),
                    )
                    .await
            });
            assert!(timeout(Duration::from_millis(50), &mut next).await.is_err());
            release.send(()).unwrap();
            assert_eq!(
                timeout(Duration::from_secs(2), next)
                    .await
                    .unwrap()
                    .unwrap()
                    .status(),
                StatusCode::OK
            );
            assert_eq!(state.sync_query.available_permits(), 1);
        }
    });
}
