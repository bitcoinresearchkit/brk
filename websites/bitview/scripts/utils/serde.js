const localhost = window.location.hostname === "localhost";
console.log({ localhost });

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
   * @returns {ChartableIndex}
   */
  deserialize(v) {
    switch (v) {
      case "timestamp":
        return "height";
      case "date":
        return "dateindex";
      case "week":
        return "weekindex";
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
