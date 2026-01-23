/** Shared helpers for options */

import { Unit } from "../utils/units.js";
import { line } from "./series.js";

/**
 * Create sats/btc/usd line series from a pattern with .sats/.bitcoin/.dollars
 * @param {{ sats: AnyMetricPattern, bitcoin: AnyMetricPattern, dollars: AnyMetricPattern }} pattern
 * @param {string} name
 * @param {Color} [color]
 * @param {{ defaultActive?: boolean }} [options]
 * @returns {FetchedLineSeriesBlueprint[]}
 */
export function satsBtcUsd(pattern, name, color, options) {
  const { defaultActive } = options || {};
  return [
    line({ metric: pattern.sats, name, color, unit: Unit.sats, defaultActive }),
    line({
      metric: pattern.bitcoin,
      name,
      color,
      unit: Unit.btc,
      defaultActive,
    }),
    line({
      metric: pattern.dollars,
      name,
      color,
      unit: Unit.usd,
      defaultActive,
    }),
  ];
}

/**
 * Build percentile USD mappings from a ratio pattern
 * @param {Colors} colors
 * @param {ActivePriceRatioPattern} ratio
 */
export function percentileUsdMap(colors, ratio) {
  return /** @type {const} */ ([
    { name: "pct95", prop: ratio.ratioPct95Usd, color: colors.fuchsia },
    { name: "pct5", prop: ratio.ratioPct5Usd, color: colors.cyan },
    { name: "pct98", prop: ratio.ratioPct98Usd, color: colors.pink },
    { name: "pct2", prop: ratio.ratioPct2Usd, color: colors.sky },
    { name: "pct99", prop: ratio.ratioPct99Usd, color: colors.rose },
    { name: "pct1", prop: ratio.ratioPct1Usd, color: colors.blue },
  ]);
}

/**
 * Build percentile ratio mappings from a ratio pattern
 * @param {Colors} colors
 * @param {ActivePriceRatioPattern} ratio
 */
export function percentileMap(colors, ratio) {
  return /** @type {const} */ ([
    { name: "pct95", prop: ratio.ratioPct95, color: colors.fuchsia },
    { name: "pct5", prop: ratio.ratioPct5, color: colors.cyan },
    { name: "pct98", prop: ratio.ratioPct98, color: colors.pink },
    { name: "pct2", prop: ratio.ratioPct2, color: colors.sky },
    { name: "pct99", prop: ratio.ratioPct99, color: colors.rose },
    { name: "pct1", prop: ratio.ratioPct1, color: colors.blue },
  ]);
}

/**
 * Build SD patterns from a ratio pattern
 * @param {ActivePriceRatioPattern} ratio
 */
export function sdPatterns(ratio) {
  return /** @type {const} */ ([
    { nameAddon: "all", titleAddon: "", sd: ratio.ratioSd },
    { nameAddon: "4y", titleAddon: "4y", sd: ratio.ratio4ySd },
    { nameAddon: "2y", titleAddon: "2y", sd: ratio.ratio2ySd },
    { nameAddon: "1y", titleAddon: "1y", sd: ratio.ratio1ySd },
  ]);
}

/**
 * Build SD band mappings from an SD pattern
 * @param {Colors} colors
 * @param {Ratio1ySdPattern} sd
 */
export function sdBands(colors, sd) {
  return /** @type {const} */ ([
    { name: "0σ", prop: sd._0sdUsd, color: colors.lime },
    { name: "+0.5σ", prop: sd.p05sdUsd, color: colors.yellow },
    { name: "−0.5σ", prop: sd.m05sdUsd, color: colors.teal },
    { name: "+1σ", prop: sd.p1sdUsd, color: colors.amber },
    { name: "−1σ", prop: sd.m1sdUsd, color: colors.cyan },
    { name: "+1.5σ", prop: sd.p15sdUsd, color: colors.orange },
    { name: "−1.5σ", prop: sd.m15sdUsd, color: colors.sky },
    { name: "+2σ", prop: sd.p2sdUsd, color: colors.red },
    { name: "−2σ", prop: sd.m2sdUsd, color: colors.blue },
    { name: "+2.5σ", prop: sd.p25sdUsd, color: colors.rose },
    { name: "−2.5σ", prop: sd.m25sdUsd, color: colors.indigo },
    { name: "+3σ", prop: sd.p3sd, color: colors.pink },
    { name: "−3σ", prop: sd.m3sd, color: colors.violet },
  ]);
}
