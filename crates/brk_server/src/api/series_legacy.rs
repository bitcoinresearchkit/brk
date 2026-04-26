//! Deprecated series-format infrastructure. Sunset date: 2027-01-01.
//!
//! Two responsibilities, deletable as a unit when the sunset arrives:
//! - `handler` / `SUNSET`: the shared legacy series handler used by `/api/series`
//!   in legacy mode (registered by metrics endpoints that emit the old format).
//! - `add_series_legacy_routes`: the deprecated `/api/series/cost-basis/*` URLs.

use std::{collections::BTreeMap, net::SocketAddr};

use aide::axum::{ApiRouter, routing::get_with};
use axum::{
    Extension,
    body::Bytes,
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode, Uri},
    response::Response,
};
use brk_error::{Error, Result as BrkResult};
use brk_query::Query as BrkQuery;
use brk_types::{
    Bitcoin, Cents, Cohort, Date, Day1, Dollars, OutputLegacy, Sats, SeriesSelection,
    UrpdAggregation, Version,
};
use rustc_hash::FxHashMap;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vecdb::ReadableOptionVec;

use crate::{
    AppState, CacheStrategy, Result,
    extended::{HeaderMapExtended, TransformResponseExtended},
};

pub const SUNSET: &str = "2027-01-01T00:00:00Z";

/// Legacy series handler. Emits the pre-2027 `OutputLegacy` format and tags
/// the response with `Deprecation` / `Sunset` headers. Reused by `metrics/*`
/// for endpoints that must stay on the old format until sunset.
pub async fn handler(
    uri: Uri,
    headers: HeaderMap,
    Extension(addr): Extension<SocketAddr>,
    Query(params): Query<SeriesSelection>,
    State(state): State<AppState>,
) -> Result<Response> {
    let mut response = super::series::serve(state, uri, headers, addr, params, legacy_bytes).await?;
    if response.status() == StatusCode::OK {
        response.headers_mut().insert_deprecation(SUNSET);
    }
    Ok(response)
}

fn legacy_bytes(q: &BrkQuery, r: brk_query::ResolvedQuery) -> BrkResult<Bytes> {
    Ok(match q.format_legacy(r)?.output {
        OutputLegacy::CSV(s) => Bytes::from(s),
        OutputLegacy::Json(v) => Bytes::from(v.to_vec()),
    })
}

#[derive(Deserialize, JsonSchema)]
struct CostBasisParams {
    cohort: Cohort,
    #[schemars(with = "String", example = &"2024-01-01")]
    date: Date,
}

#[derive(Deserialize, JsonSchema)]
struct CostBasisCohortParam {
    cohort: Cohort,
}

#[derive(Deserialize, JsonSchema)]
struct CostBasisQuery {
    #[serde(default)]
    bucket: UrpdAggregation,
    #[serde(default)]
    value: CostBasisValue,
}

/// Value type for the deprecated cost-basis distribution output.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
enum CostBasisValue {
    #[default]
    Supply,
    Realized,
    Unrealized,
}

/// Formatted cost basis output.
/// Key: price floor in USD. Value: BTC (for supply) or USD (for realized/unrealized).
type CostBasisFormatted = BTreeMap<Dollars, f64>;

fn cost_basis_formatted(
    q: &BrkQuery,
    cohort: &Cohort,
    date: Date,
    agg: UrpdAggregation,
    value: CostBasisValue,
) -> BrkResult<CostBasisFormatted> {
    let raw = q.urpd_raw(cohort, date)?;
    let day1 = Day1::try_from(date).map_err(|e| Error::Parse(e.to_string()))?;
    let spot_cents = q
        .computer()
        .prices
        .split
        .close
        .cents
        .day1
        .collect_one_flat(day1)
        .ok_or_else(|| Error::NotFound(format!("No price data for {date}")))?;
    let spot = Dollars::from(spot_cents);
    let needs_realized = value == CostBasisValue::Realized;

    let mut bucketed: FxHashMap<Cents, (Sats, Dollars)> =
        FxHashMap::with_capacity_and_hasher(raw.map.len(), Default::default());
    for (&price_cents, &sats) in &raw.map {
        let price = Cents::from(price_cents);
        let key = match agg {
            UrpdAggregation::Raw => price,
            _ => agg.bucket_floor(price).unwrap_or(price),
        };
        let entry = bucketed.entry(key).or_insert((Sats::ZERO, Dollars::ZERO));
        entry.0 += sats;
        if needs_realized {
            entry.1 += Dollars::from(price) * sats;
        }
    }

    Ok(bucketed
        .into_iter()
        .map(|(cents, (sats, realized))| {
            let k = Dollars::from(cents);
            let v = match value {
                CostBasisValue::Supply => f64::from(Bitcoin::from(sats)),
                CostBasisValue::Realized => f64::from(realized),
                CostBasisValue::Unrealized => f64::from((spot - k) * sats),
            };
            (k, v)
        })
        .collect())
}

pub trait ApiSeriesLegacyRoutes {
    fn add_series_legacy_routes(self) -> Self;
}

impl ApiSeriesLegacyRoutes for ApiRouter<AppState> {
    fn add_series_legacy_routes(self) -> Self {
        self.api_route(
            "/api/series/cost-basis",
            get_with(
                async |uri: Uri, headers: HeaderMap, State(state): State<AppState>| {
                    state
                        .cached_json(&headers, CacheStrategy::Deploy, &uri, |q| q.urpd_cohorts())
                        .await
                },
                |op| {
                    op.id("get_cost_basis_cohorts")
                        .series_tag()
                        .deprecated()
                        .summary("Available cost basis cohorts (deprecated)")
                        .description(
                            "**DEPRECATED** - Use `GET /api/urpd` instead.\n\n\
                            Sunset date: 2027-01-01.",
                        )
                        .json_response::<Vec<Cohort>>()
                        .not_modified()
                        .server_error()
                },
            ),
        )
        .api_route(
            "/api/series/cost-basis/{cohort}/dates",
            get_with(
                async |uri: Uri,
                       headers: HeaderMap,
                       Path(params): Path<CostBasisCohortParam>,
                       State(state): State<AppState>| {
                    state
                        .cached_json(&headers, CacheStrategy::Tip, &uri, move |q| {
                            q.urpd_dates(&params.cohort)
                        })
                        .await
                },
                |op| {
                    op.id("get_cost_basis_dates")
                        .series_tag()
                        .deprecated()
                        .summary("Available cost basis dates (deprecated)")
                        .description(
                            "**DEPRECATED** - Use `GET /api/urpd/{cohort}/dates` instead.\n\n\
                            Sunset date: 2027-01-01.",
                        )
                        .json_response::<Vec<Date>>()
                        .not_modified()
                        .not_found()
                        .server_error()
                },
            ),
        )
        .api_route(
            "/api/series/cost-basis/{cohort}/{date}",
            get_with(
                async |uri: Uri,
                       headers: HeaderMap,
                       Path(params): Path<CostBasisParams>,
                       Query(query): Query<CostBasisQuery>,
                       State(state): State<AppState>| {
                    let strategy = state.date_cache(Version::ONE, params.date);
                    state
                        .cached_json(&headers, strategy, &uri, move |q| {
                            cost_basis_formatted(
                                q,
                                &params.cohort,
                                params.date,
                                query.bucket,
                                query.value,
                            )
                        })
                        .await
                },
                |op| {
                    op.id("get_cost_basis")
                        .series_tag()
                        .deprecated()
                        .summary("Cost basis distribution (deprecated)")
                        .description(
                            "**DEPRECATED** - Use `GET /api/urpd/{cohort}/{date}` instead. \
                            The new endpoint returns supply, realized cap, and unrealized P&L \
                            per bucket in one response.\n\n\
                            Sunset date: 2027-01-01.",
                        )
                        .json_response::<CostBasisFormatted>()
                        .not_modified()
                        .not_found()
                        .server_error()
                },
            ),
        )
    }
}
