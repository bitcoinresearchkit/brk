use aide::axum::{ApiRouter, routing::get_with};
use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, Uri},
};
use brk_types::{Cohort, Date, Urpd, Version};

use crate::{
    CacheStrategy,
    extended::TransformResponseExtended,
    params::{Empty, UrpdCohortParam, UrpdParams, UrpdQuery},
};

use super::AppState;

pub trait ApiUrpdRoutes {
    fn add_urpd_routes(self) -> Self;
}

impl ApiUrpdRoutes for ApiRouter<AppState> {
    fn add_urpd_routes(self) -> Self {
        self.api_route(
            "/api/urpd",
            get_with(
                async |uri: Uri, headers: HeaderMap, _: Empty, State(state): State<AppState>| {
                    state
                        .cached_json(&headers, CacheStrategy::Deploy, &uri, |q| q.urpd_cohorts())
                        .await
                },
                |op| {
                    op.id("list_urpd_cohorts")
                        .urpd_tag()
                        .summary("Available URPD cohorts")
                        .description(
                            "Cohorts for which URPD data is available. Returns names like \
                            `all`, `sth`, `lth`, `utxos_under_1h_old`.",
                        )
                        .json_response::<Vec<Cohort>>()
                        .not_modified()
                        .server_error()
                },
            ),
        )
        .api_route(
            "/api/urpd/{cohort}/dates",
            get_with(
                async |uri: Uri,
                       headers: HeaderMap,
                       Path(params): Path<UrpdCohortParam>,
                       _: Empty,
                       State(state): State<AppState>| {
                    state
                        .cached_json(&headers, CacheStrategy::Tip, &uri, move |q| {
                            q.urpd_dates(&params.cohort)
                        })
                        .await
                },
                |op| {
                    op.id("list_urpd_dates")
                        .urpd_tag()
                        .summary("Available URPD dates")
                        .description(
                            "Dates for which a URPD snapshot is available for the cohort. \
                            One entry per UTC day, sorted ascending.",
                        )
                        .json_response::<Vec<Date>>()
                        .not_modified()
                        .not_found()
                        .server_error()
                },
            ),
        )
        .api_route(
            "/api/urpd/{cohort}",
            get_with(
                async |uri: Uri,
                       headers: HeaderMap,
                       Path(params): Path<UrpdCohortParam>,
                       Query(query): Query<UrpdQuery>,
                       State(state): State<AppState>| {
                    state
                        .cached_json(&headers, CacheStrategy::Tip, &uri, move |q| {
                            q.urpd_latest(&params.cohort, query.aggregation)
                        })
                        .await
                },
                |op| {
                    op.id("get_urpd")
                        .urpd_tag()
                        .summary("Latest URPD")
                        .description(
                            "URPD for the most recent available date in the cohort. \
                            The response's `date` field echoes which date was served.\n\n\
                            See the URPD tag description for the response shape and `agg` options.",
                        )
                        .json_response::<Urpd>()
                        .not_modified()
                        .not_found()
                        .server_error()
                },
            ),
        )
        .api_route(
            "/api/urpd/{cohort}/{date}",
            get_with(
                async |uri: Uri,
                       headers: HeaderMap,
                       Path(params): Path<UrpdParams>,
                       Query(query): Query<UrpdQuery>,
                       State(state): State<AppState>| {
                    let strategy = state.date_cache(Version::ONE, params.date);
                    state
                        .cached_json(&headers, strategy, &uri, move |q| {
                            q.urpd_at(&params.cohort, params.date, query.aggregation)
                        })
                        .await
                },
                |op| {
                    op.id("get_urpd_at")
                        .urpd_tag()
                        .summary("URPD at date")
                        .description(
                            "URPD for a (cohort, date) pair. Returns \
                            `{ cohort, date, aggregation, close, total_supply, buckets }` where \
                            each bucket is `{ price_floor, supply, realized_cap, unrealized_pnl }`.\n\n\
                            See the URPD tag description for unit conventions and `agg` options.",
                        )
                        .json_response::<Urpd>()
                        .not_modified()
                        .not_found()
                        .server_error()
                },
            ),
        )
    }
}
