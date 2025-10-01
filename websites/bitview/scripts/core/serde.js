const localhost = window.location.hostname === "localhost";

export const serdeString = {
  /**
   * @param {string} v
   */
  serialize(v) {
    return v;
  },
  /**
   * @param {string} v
   */
  deserialize(v) {
    return v;
  },
};

export const serdeMetrics = {
  /**
   * @param {Metric[]} v
   */
  serialize(v) {
    return v.join(",");
  },
  /**
   * @param {string} v
   */
  deserialize(v) {
    return /** @type {Metric[]} */ (v.split(","));
  },
};

export const serdeNumber = {
  /**
   * @param {number} v
   */
  serialize(v) {
    return String(v);
  },
  /**
   * @param {string} v
   */
  deserialize(v) {
    return Number(v);
  },
};

export const serdeOptNumber = {
  /**
   * @param {number | null} v
   */
  serialize(v) {
    return v !== null ? String(v) : "";
  },
  /**
   * @param {string} v
   */
  deserialize(v) {
    return v ? Number(v) : null;
  },
};

export const serdeDate = {
  /**
   * @param {Date} date
   */
  serialize(date) {
    return date.toString();
  },
  /**
   * @param {string} v
   */
  deserialize(v) {
    return new Date(v);
  },
};

export const serdeOptDate = {
  /**
   * @param {Date | null} date
   */
  serialize(date) {
    return date !== null ? date.toString() : "";
  },
  /**
   * @param {string} v
   */
  deserialize(v) {
    return new Date(v);
  },
};

export const serdeBool = {
  /**
   * @param {boolean} v
   */
  serialize(v) {
    return String(v);
  },
  /**
   * @param {string} v
   */
  deserialize(v) {
    if (v === "true" || v === "1") {
      return true;
    } else if (v === "false" || v === "0") {
      return false;
    } else {
      throw "deser bool err";
    }
  },
};

export const serdeChartableIndex = {
  /**
   * @param {IndexName} v
   * @returns {ChartableIndexName | null}
   */
  serialize(v) {
    switch (v) {
      case "dateindex":
        return "date";
      case "decadeindex":
        return "decade";
      case "difficultyepoch":
        return "epoch";
      case "height":
        return "timestamp";
      case "monthindex":
        return "month";
      case "quarterindex":
        return "quarter";
      case "semesterindex":
        return "semester";
      case "weekindex":
        return "week";
      case "yearindex":
        return "year";
      default:
        return null;
    }
  },
  /**
   * @param {ChartableIndexName} v
   * @returns {IndexName}
   */
  deserialize(v) {
    switch (v) {
      case "timestamp":
        return "height";
      case "date":
        return "dateindex";
      case "week":
        return "weekindex";
      case "epoch":
        return "difficultyepoch";
      case "month":
        return "monthindex";
      case "quarter":
        return "quarterindex";
      case "semester":
        return "semesterindex";
      case "year":
        return "yearindex";
      case "decade":
        return "decadeindex";
      default:
        throw Error("todo");
    }
  },
};

/**
 * @typedef {"" |
 *   "%all" |
 *   "%cmcap" |
 *   "%cp+l" |
 *   "%mcap" |
 *   "%pnl" |
 *   "%rcap" |
 *   "%self" |
 *   "/sec" |
 *   "address data" |
 *   "block" |
 *   "blocks" |
 *   "bool" |
 *   "btc" |
 *   "bytes" |
 *   "cents" |
 *   "coinblocks" |
 *   "coindays" |
 *   "constant" |
 *   "count" |
 *   "date" |
 *   "days" |
 *   "difficulty" |
 *   "epoch" |
 *   "gigabytes" |
 *   "h/s" |
 *   "hash" |
 *   "height" |
 *   "id" |
 *   "index" |
 *   "len" |
 *   "locktime" |
 *   "percentage" |
 *   "position" |
 *   "ratio" |
 *   "sat/vb" |
 *   "satblocks" |
 *   "satdays" |
 *   "sats" |
 *   "sats/(ph/s)/day" |
 *   "sats/(th/s)/day" |
 *   "sd" |
 *   "secs" |
 *   "timestamp" |
 *   "tx" |
 *   "type" |
 *   "usd" |
 *   "usd/(ph/s)/day" |
 *   "usd/(th/s)/day" |
 *   "vb" |
 *   "version" |
 *   "wu" |
 *   "years" |
 * "" } Unit
 */

export const serdeUnit = {
  /**
   * @param {string} v
   */
  deserialize(v) {
    /** @type {Unit | undefined} */
    let unit;

    /**
     * @param {Unit} u
     */
    function setUnit(u) {
      if (unit)
        throw Error(
          `Can't assign "${u}" to unit, "${unit}" is already assigned to "${v}"`,
        );
      unit = u;
    }

    if (
      (!unit || localhost) &&
      (v.includes("in_sats") ||
        (v.endsWith("supply") &&
          !(v.endsWith("circulating_supply") || v.endsWith("_own_supply"))) ||
        v === "sent" ||
        v === "annualized_volume" ||
        v.endsWith("supply_half") ||
        v.endsWith("supply_in_profit") ||
        v.endsWith("supply_in_loss") ||
        v.endsWith("stack") ||
        (v.endsWith("value") && !v.includes("realized")) ||
        ((v.includes("coinbase") ||
          v.includes("fee") ||
          v.includes("subsidy") ||
          v.includes("rewards")) &&
          !(
            v.startsWith("is_") ||
            v.includes("_btc") ||
            v.includes("_usd") ||
            v.includes("fee_rate") ||
            v.endsWith("dominance")
          )))
    ) {
      setUnit("sats");
    }
    if (
      (!unit || localhost) &&
      !v.endsWith("velocity") &&
      ((v.includes("_btc") &&
        !(v.includes("0k_btc") || v.includes("1k_btc"))) ||
        v.endsWith("_btc"))
    ) {
      setUnit("btc");
    }
    if ((!unit || localhost) && v === "chain") {
      setUnit("block");
    }
    if ((!unit || localhost) && v.startsWith("blocks_before")) {
      setUnit("blocks");
    }
    if (
      (!unit || localhost) &&
      (v === "emptyaddressdata" || v === "loadedaddressdata")
    ) {
      setUnit("address data");
    }
    if (
      (!unit || localhost) &&
      (v === "price_high" ||
        v === "price_ohlc" ||
        v === "price_low" ||
        v === "price_close" ||
        v === "price_open" ||
        v === "price_ath" ||
        v === "market_cap" ||
        v.startsWith("price_true_range") ||
        (v.includes("_usd") && !v.endsWith("velocity")) ||
        v.includes("cointime_value") ||
        v.endsWith("_ago") ||
        v.endsWith("price_paid") ||
        v.endsWith("_price") ||
        (v.startsWith("price") && (v.endsWith("min") || v.endsWith("max"))) ||
        (v.endsWith("_cap") && !v.includes("rel_to")) ||
        v.endsWith("value_created") ||
        v.endsWith("value_destroyed") ||
        ((v.includes("realized") || v.includes("true_market_mean")) &&
          !v.includes("ratio") &&
          !v.includes("rel_to")) ||
        ((v.endsWith("sma") || v.includes("sma_x") || v.endsWith("ema")) &&
          !v.includes("ratio") &&
          !v.includes("sopr") &&
          !v.includes("hash_rate")) ||
        v === "ath")
    ) {
      setUnit("usd");
    }
    if ((!unit || localhost) && v.endsWith("cents")) {
      setUnit("cents");
    }
    if (
      ((!unit || localhost) &&
        (v.endsWith("ratio") ||
          (v.includes("ratio") &&
            (v.endsWith("sma") || v.endsWith("ema") || v.endsWith("zscore"))) ||
          v.includes("sopr") ||
          v.endsWith("_5sd") ||
          v.endsWith("1sd") ||
          v.endsWith("2sd") ||
          v.endsWith("3sd") ||
          v.endsWith("pct1") ||
          v.endsWith("pct2") ||
          v.endsWith("pct5") ||
          v.endsWith("pct95") ||
          v.endsWith("pct98") ||
          v.endsWith("pct99"))) ||
      v.includes("liveliness") ||
      v.includes("vaultedness") ||
      v == "puell_multiple" ||
      v.endsWith("velocity")
    ) {
      setUnit("ratio");
    }
    if (
      (!unit || localhost) &&
      (v === "price_drawdown" ||
        v === "difficulty_adjustment" ||
        v.endsWith("inflation_rate") ||
        v.endsWith("_oscillator") ||
        v.endsWith("_dominance") ||
        v.endsWith("_returns") ||
        v.endsWith("_rebound") ||
        v.endsWith("_volatility") ||
        v.endsWith("_cagr"))
    ) {
      setUnit("percentage");
    }
    if (
      (!unit || localhost) &&
      (v.endsWith("count") ||
        v.includes("_count_") ||
        v.startsWith("block_count") ||
        v.includes("blocks_mined") ||
        (v.includes("tx_v") && !v.includes("vsize")))
    ) {
      setUnit("count");
    }
    if (
      (!unit || localhost) &&
      (v.startsWith("hash_rate") || v.endsWith("as_hash"))
    ) {
      setUnit("h/s");
    }
    if ((!unit || localhost) && v === "pool") {
      setUnit("id");
    }
    if ((!unit || localhost) && v.includes("fee_rate")) {
      setUnit("sat/vb");
    }
    if ((!unit || localhost) && v.startsWith("is_")) {
      setUnit("bool");
    }
    if ((!unit || localhost) && v.endsWith("type")) {
      setUnit("type");
    }
    if (
      (!unit || localhost) &&
      (v === "interval" || v.startsWith("block_interval"))
    ) {
      setUnit("secs");
    }
    if ((!unit || localhost) && v.endsWith("_per_sec")) {
      setUnit("/sec");
    }
    if ((!unit || localhost) && v.endsWith("locktime")) {
      setUnit("locktime");
    }

    if ((!unit || localhost) && v.endsWith("version")) {
      setUnit("version");
    }
    if (
      (!unit || localhost) &&
      (v === "txid" ||
        (v.endsWith("bytes") && !v.endsWith("vbytes")) ||
        v.endsWith("base_size") ||
        v.endsWith("total_size") ||
        v.includes("block_size"))
    ) {
      setUnit("bytes");
    }
    if ((!unit || localhost) && v.endsWith("_sd")) {
      setUnit("sd");
    }
    if ((!unit || localhost) && (v.includes("vsize") || v.includes("vbytes"))) {
      setUnit("vb");
    }
    if ((!unit || localhost) && v.includes("weight")) {
      setUnit("wu");
    }
    if ((!unit || localhost) && v.endsWith("index")) {
      setUnit("index");
    }
    if ((!unit || localhost) && (v === "date" || v === "date_fixed")) {
      setUnit("date");
    }
    if (
      (!unit || localhost) &&
      (v === "timestamp" || v === "timestamp_fixed")
    ) {
      setUnit("timestamp");
    }
    if ((!unit || localhost) && v.includes("coinblocks")) {
      setUnit("coinblocks");
    }
    if ((!unit || localhost) && v.includes("coindays")) {
      setUnit("coindays");
    }
    if ((!unit || localhost) && v.includes("satblocks")) {
      setUnit("satblocks");
    }
    if ((!unit || localhost) && v.includes("satdays")) {
      setUnit("satdays");
    }
    if ((!unit || localhost) && v.endsWith("height")) {
      setUnit("height");
    }
    if ((!unit || localhost) && v.endsWith("rel_to_market_cap")) {
      setUnit("%mcap");
    }
    if ((!unit || localhost) && v.endsWith("rel_to_own_market_cap")) {
      setUnit("%cmcap");
    }
    if ((!unit || localhost) && v.endsWith("rel_to_own_total_unrealized_pnl")) {
      setUnit("%cp+l");
    }
    if ((!unit || localhost) && v.endsWith("rel_to_realized_cap")) {
      setUnit("%rcap");
    }
    if ((!unit || localhost) && v.endsWith("rel_to_circulating_supply")) {
      setUnit("%all");
    }
    if (
      (!unit || localhost) &&
      (v.includes("rel_to_realized_profit") ||
        v.includes("rel_to_realized_loss"))
    ) {
      setUnit("%pnl");
    }
    if ((!unit || localhost) && v.endsWith("rel_to_own_supply")) {
      setUnit("%self");
    }
    if ((!unit || localhost) && v.endsWith("epoch")) {
      setUnit("epoch");
    }
    if ((!unit || localhost) && v === "difficulty") {
      setUnit("difficulty");
    }
    if ((!unit || localhost) && v === "blockhash") {
      setUnit("hash");
    }
    if ((!unit || localhost) && v.startsWith("hash_price_phs")) {
      setUnit("usd/(ph/s)/day");
    }
    if ((!unit || localhost) && v.startsWith("hash_price_ths")) {
      setUnit("usd/(th/s)/day");
    }
    if ((!unit || localhost) && v.startsWith("hash_value_phs")) {
      setUnit("sats/(ph/s)/day");
    }
    if ((!unit || localhost) && v.startsWith("hash_value_ths")) {
      setUnit("sats/(th/s)/day");
    }

    if (
      (!unit || localhost) &&
      (v.includes("days_between") ||
        v.includes("days_since") ||
        v.startsWith("days_before"))
    ) {
      setUnit("days");
    }
    if ((!unit || localhost) && v.includes("years_between")) {
      setUnit("years");
    }
    if ((!unit || localhost) && v == "len") {
      setUnit("len");
    }
    if ((!unit || localhost) && v == "position") {
      setUnit("position");
    }
    if ((!unit || localhost) && v.startsWith("constant")) {
      setUnit("constant");
    }

    if (!unit) {
      console.log();
      throw Error(`Unit not set for "${v}"`);
    }

    return /** @type {Unit} */ (unit);
  },
};
