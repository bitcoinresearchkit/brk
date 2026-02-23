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
    } else {
      return false;
    }
  },
};

/**
 * @typedef {"timestamp" | "date" | "week" | "month" | "month3" | "month6" | "year" | "year10"} ChartableIndexName
 */

export const serdeChartableIndex = {
  /**
   * @param {IndexName} v
   * @returns {ChartableIndexName | null}
   */
  serialize(v) {
    switch (v) {
      case "day1":
        return "date";
      case "year10":
        return "year10";
      case "height":
        return "timestamp";
      case "month1":
        return "month";
      case "month3":
        return "month3";
      case "month6":
        return "month6";
      case "week1":
        return "week";
      case "year1":
        return "year";
      default:
        return null;
    }
  },
  /**
   * @param {ChartableIndexName} v
   * @returns {ChartableIndex}
   */
  deserialize(v) {
    switch (v) {
      case "timestamp":
        return "height";
      case "date":
        return "day1";
      case "week":
        return "week1";
      case "month":
        return "month1";
      case "month3":
        return "month3";
      case "month6":
        return "month6";
      case "year":
        return "year1";
      case "year10":
        return "year10";
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
