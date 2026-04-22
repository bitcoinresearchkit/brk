//! Deprecated `/api/series/cost-basis/*` routes.
//! Sunset date: 2027-01-01. Delete this file and its registration in `mod.rs` together.

use std::collections::BTreeMap;

use aide::axum::{ApiRouter, routing::get_with};
use axum::{
    extract::{Path, Query as AxumQuery, State},
    http::{HeaderMap, Uri},
};
use brk_error::{Error, Result};
use brk_query::Query;
use brk_types::{Bitcoin, Cents, Cohort, Date, Day1, Dollars, Sats, UrpdAggregation, Version};
use rustc_hash::FxHashMap;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vecdb::ReadableOptionVec;

use crate::{AppState, CacheStrategy, extended::TransformResponseExtended};

#[derive(Deserialize, JsonSchema)]
pub(super) struct CostBasisParams {
    pub cohort: Cohort,
    #[schemars(with = "String", example = &"2024-01-01")]
    pub date: Date,
}

#[derive(Deserialize, JsonSchema)]
pub(super) struct CostBasisCohortParam {
    pub cohort: Cohort,
}

#[derive(Deserialize, JsonSchema)]
pub(super) struct CostBasisQuery {
    #[serde(default)]
    pub bucket: UrpdAggregation,
    #[serde(default)]
    pub value: CostBasisValue,
}

/// Value type for the deprecated cost-basis distribution output.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub(super) enum CostBasisValue {
    #[default]
    Supply,
    Realized,
    Unrealized,
}

/// Formatted cost basis output.
/// Key: price floor in USD. Value: BTC (for supply) or USD (for realized/unrealized).
type CostBasisFormatted = BTreeMap<Dollars, f64>;

fn cost_basis_formatted(
    q: &Query,
    cohort: &Cohort,
    date: Date,
    agg: UrpdAggregation,
    value: CostBasisValue,
) -> Result<CostBasisFormatted> {
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

pub(super) trait ApiCostBasisLegacyRoutes {
    fn add_cost_basis_legacy_routes(self) -> Self;
}

impl ApiCostBasisLegacyRoutes for ApiRouter<AppState> {
    fn add_cost_basis_legacy_routes(self) -> Self {
        self.api_route(
            "/api/series/cost-basis",
            get_with(
                async |uri: Uri, headers: HeaderMap, State(state): State<AppState>| {
                    state
                        .cached_json(&headers, CacheStrategy::Static, &uri, |q| q.urpd_cohorts())
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
                       AxumQuery(query): AxumQuery<CostBasisQuery>,
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
