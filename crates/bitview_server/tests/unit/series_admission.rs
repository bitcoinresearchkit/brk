use std::time::Duration;

use aide::axum::ApiRouter;
use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::from_fn,
};
use tokio::{spawn, time::sleep};
use tower::ServiceExt;

use crate::{AppState, api::ApiRoutes, read_availability};

pub async fn check(state: &AppState) {
    let router = ApiRouter::new().add_api_routes().with_state(state.clone());
    let request = |method, path: &str, tag: &str| {
        Request::builder()
            .method(method)
            .uri(path)
            .header("if-none-match", tag)
            .body(Body::empty())
            .unwrap()
    };
    let budget = state.series_bodies.response_bodies();
    for endpoint in ["timestamp/height", "timestamp/height/data", "bulk"] {
        for format in ["json", "csv"] {
            let selection = if endpoint == "bulk" {
                "series=timestamp&index=height&"
            } else {
                ""
            };
            let path = format!("/api/series/{endpoint}?{selection}limit=1&format={format}");
            let first = router
                .clone()
                .oneshot(request("GET", &path, "\"old\""))
                .await
                .unwrap();
            assert_eq!(first.status(), StatusCode::OK, "{path}");
            if format == "csv" {
                assert!(first.headers().contains_key("content-disposition"));
            }
            let tag = first.headers()["etag"].to_str().unwrap().to_owned();
            let second = router
                .clone()
                .oneshot(request("GET", &path, "\"old\""))
                .await
                .unwrap();
            assert_eq!(second.status(), StatusCode::OK);
            assert_eq!(budget.available_permits(), 0);
            let busy = router
                .clone()
                .oneshot(request("GET", &path, "\"old\""))
                .await
                .unwrap();
            assert_eq!(busy.status(), StatusCode::SERVICE_UNAVAILABLE);
            assert!(!busy.headers().contains_key("etag"));
            for method in ["GET", "HEAD"] {
                for condition in [tag.as_str(), "*"] {
                    let response = router
                        .clone()
                        .oneshot(request(method, &path, condition))
                        .await
                        .unwrap();
                    assert_eq!(response.status(), StatusCode::NOT_MODIFIED);
                }
            }
            let waiting_router = router.clone().layer(from_fn(read_availability::wait));
            let pending = spawn(waiting_router.oneshot(request("GET", &path, "\"old\"")));
            sleep(Duration::from_millis(50)).await;
            assert!(
                !pending.is_finished(),
                "capacity must wait inside the server"
            );
            drop(first);
            let admitted = pending.await.unwrap().unwrap();
            assert_eq!(admitted.status(), StatusCode::OK);
            assert_eq!(budget.available_permits(), 0);
            drop(admitted);
            assert_eq!(budget.available_permits(), 1);
            drop(second);
            assert_eq!(budget.available_permits(), 2);
        }
    }
}
