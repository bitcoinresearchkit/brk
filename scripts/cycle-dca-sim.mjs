#!/usr/bin/env node

const DEFAULT_BUY_LEVELS = new Map([
  [100, 10.875],
  [95, 11.7395833],
  [90, 12.6041667],
  [85, 13.46875],
  [80, 14.3333333],
  [75, 15.1979167],
  [70, 16.0625],
  [65, 16.9270833],
  [60, 17.7916667],
  [55, 18.65625],
  [50, 19.5208333],
  [45, 20.3854167],
  [40, 21.25],
]);

const DAYS_PER_MONTH = 365.2425 / 12;

function parseArgs(argv) {
  const opts = {
    baseUrl: "https://bitview.space",
    dataStart: "2014-01-01",
    start: "2014-01-01",
    end: null,
    starts: null,
    startSet: "cycle-extremes",
    initialCash: 10_000,
    monthlyTopup: 1_000,
    dailyBuy: null,
    buyRunwayMonths: 0,
    minCashReserveMonths: 0,
    initialDeployDays: 365,
    buyTriggerPct: 50,
    buyLevels: DEFAULT_BUY_LEVELS,
    sellRule: "percentile-band",
    sellArmPct: 100,
    sellBandLowerPct: 95,
    sellBandUpperPct: 100,
    sellBandMultiple: 2.75,
    sellAthMultiple: 3,
    sellMap: null,
    maxDailySellFraction: 0.005,
    mode: "both",
    output: "table",
  };

  for (let i = 0; i < argv.length; i += 1) {
    const arg = argv[i];
    const next = () => {
      i += 1;
      if (i >= argv.length) throw new Error(`Missing value for ${arg}`);
      return argv[i];
    };

    if (arg === "--help" || arg === "-h") {
      opts.help = true;
    } else if (arg === "--base-url") {
      opts.baseUrl = next();
    } else if (arg === "--data-start") {
      opts.dataStart = next();
    } else if (arg === "--start") {
      opts.start = next();
    } else if (arg === "--end") {
      opts.end = next();
    } else if (arg === "--starts") {
      opts.starts = next()
        .split(",")
        .map((s) => s.trim())
        .filter(Boolean);
    } else if (arg === "--start-set") {
      opts.startSet = next();
    } else if (arg === "--initial-cash") {
      opts.initialCash = parseNumber(next(), arg);
    } else if (arg === "--monthly-topup") {
      opts.monthlyTopup = parseNumber(next(), arg);
    } else if (arg === "--daily-buy") {
      opts.dailyBuy = parseNumber(next(), arg);
    } else if (arg === "--buy-runway-months") {
      opts.buyRunwayMonths = parseNumber(next(), arg);
    } else if (arg === "--min-cash-reserve-months") {
      opts.minCashReserveMonths = parseNumber(next(), arg);
    } else if (arg === "--initial-deploy-days") {
      opts.initialDeployDays = parseInteger(next(), arg);
    } else if (arg === "--buy-trigger-pct") {
      opts.buyTriggerPct = parseInteger(next(), arg);
    } else if (arg === "--buy-levels") {
      opts.buyLevels = parsePctMap(next(), arg);
    } else if (arg === "--sell-map") {
      opts.sellMap = parsePctMap(next(), arg);
      opts.sellRule = "percentile-map";
    } else if (arg === "--sell-band") {
      const [lower, upper] = next().split(":");
      if (!lower || !upper) {
        throw new Error("--sell-band must look like lowerPct:upperPct");
      }
      opts.sellBandLowerPct = parseInteger(lower, arg);
      opts.sellBandUpperPct = parseInteger(upper, arg);
      opts.sellRule = "percentile-band";
    } else if (arg === "--sell-arm-pct") {
      opts.sellArmPct = parseInteger(next(), arg);
    } else if (arg === "--sell-band-multiple") {
      opts.sellBandMultiple = parseNumber(next(), arg);
      opts.sellRule = "percentile-band";
    } else if (arg === "--sell-ath-multiple") {
      opts.sellAthMultiple = parseNumber(next(), arg);
      opts.sellRule = "ath";
    } else if (arg === "--max-daily-sell-fraction") {
      opts.maxDailySellFraction = parseNumber(next(), arg);
    } else if (arg === "--mode") {
      opts.mode = next();
    } else if (arg === "--csv") {
      opts.output = "csv";
    } else {
      throw new Error(`Unknown argument: ${arg}`);
    }
  }

  if (!["both", "hold", "sell"].includes(opts.mode)) {
    throw new Error("--mode must be one of: both, hold, sell");
  }

  if (!["cycle-extremes", "single", "custom"].includes(opts.startSet)) {
    throw new Error("--start-set must be one of: cycle-extremes, single, custom");
  }

  if (opts.starts?.length) {
    opts.startSet = "custom";
  }

  return opts;
}

function parseNumber(value, label) {
  const parsed = Number(value);
  if (!Number.isFinite(parsed)) throw new Error(`${label} must be a number`);
  return parsed;
}

function parseInteger(value, label) {
  const parsed = Number.parseInt(value, 10);
  if (!Number.isInteger(parsed)) throw new Error(`${label} must be an integer`);
  return parsed;
}

function parsePctMap(value, label) {
  const map = new Map();
  for (const part of value.split(",")) {
    const [pct, weight] = part.split(":");
    if (!pct || !weight) {
      throw new Error(`${label} entries must look like pct:value,pct:value`);
    }
    map.set(parseInteger(pct, label), parseNumber(weight, label));
  }
  return map;
}

function printHelp() {
  console.log(`Usage: node scripts/cycle-dca-sim.mjs [options]

Fetches Bitview daily price + BTC-weighted cost-basis percentile data and
simulates the image rule:
  - p50 touch starts daily DCA-in
  - p100 increase stops DCA-in and arms DCA-out
  - optional DCA-out sells inside the selected percentile band after arming
  - start with cash, then add a monthly top-up

Defaults:
  --start 2014-01-01
  --data-start 2014-01-01
  --initial-cash 10000
  --monthly-topup 1000
  --buy-levels 100:10.875,95:11.7395833,90:12.6041667,85:13.46875,80:14.3333333,75:15.1979167,70:16.0625,65:16.9270833,60:17.7916667,55:18.65625,50:19.5208333,45:20.3854167,40:21.25
  --sell-band 95:100
  --sell-band-multiple 2.75
  --max-daily-sell-fraction 0.005
  --mode both
  --start-set cycle-extremes

Options:
  --start YYYY-MM-DD
  --data-start YYYY-MM-DD            Warmup data start for cycle phase inference
  --end YYYY-MM-DD
  --starts YYYY-MM-DD,YYYY-MM-DD  Explicit start dates
  --start-set cycle-extremes|single|custom
  --mode both|hold|sell
  --initial-cash USD
  --monthly-topup USD
  --daily-buy USD                    Default: monthly top-up / average month
  --buy-runway-months N              Cap daily buys to preserve N months of spend runway
  --min-cash-reserve-months N        Never spend below N months of top-up cash
  --initial-deploy-days N            Adds initial cash / N to buy budget on active buy days
  --buy-trigger-pct N
  --buy-levels pct:weight,...
  --sell-arm-pct N                  Arms sell phase once p100 increases, or price touches other pct
  --sell-band lowerPct:upperPct      Sell only inside percentile band after arming
  --sell-band-multiple N             Multiplies daily sell size while inside the band
  --sell-ath-multiple N             Sell when price >= previous ATH * N
  --sell-map pct:multiplier,...      Alternative: sell on cost-basis percentile thresholds
  --max-daily-sell-fraction N        Max BTC fraction sold per day when all sell thresholds fire
  --csv
`);
}

async function main() {
  const opts = parseArgs(process.argv.slice(2));
  if (opts.help) {
    printHelp();
    return;
  }

  const data = await loadData(opts);
  const starts = resolveStartPoints(data.rows, opts);
  const results = [];

  for (const startPoint of starts) {
    const startIndex = startPoint.index;
    const benchmarkLump = simulateLumpAndTopup(data.rows, startIndex, opts);
    const benchmarkDca = simulateSimpleDailyDca(data.rows, startIndex, opts);

    const modes =
      opts.mode === "both" ? [false, true] : opts.mode === "sell" ? [true] : [false];

    for (const sellEnabled of modes) {
      const signal = simulateSignal(data.rows, startIndex, opts, sellEnabled);
      results.push({
        start_label: startPoint.label,
        start_date: data.rows[startIndex].date,
        start_kind: startPoint.kind,
        start_epoch: startPoint.epoch,
        mode: sellEnabled ? "sell" : "hold",
        final_date: signal.finalDate,
        contributed: signal.contributed,
        final_value: signal.finalValue,
        return_pct: pct(signal.finalValue / signal.contributed - 1),
        cash: signal.cash,
        min_cash: signal.minCash,
        btc: signal.btc,
        buys: signal.buys,
        capped_buy_days: signal.cappedBuyDays,
        sells: signal.sells,
        bought_usd: signal.boughtUsd,
        sold_usd: signal.soldUsd,
        max_drawdown_pct: pct(signal.maxDrawdown),
        lump_value: benchmarkLump.finalValue,
        lump_delta_pct: pct(signal.finalValue / benchmarkLump.finalValue - 1),
        daily_dca_value: benchmarkDca.finalValue,
        daily_dca_delta_pct: pct(signal.finalValue / benchmarkDca.finalValue - 1),
      });
    }
  }

  if (opts.output === "csv") {
    printCsv(results);
  } else {
    printTable(results, opts);
  }
}

async function loadData(opts) {
  const pctSet = new Set([
    opts.buyTriggerPct,
    ...opts.buyLevels.keys(),
    opts.sellArmPct,
    ...(opts.sellRule === "percentile-band"
      ? [opts.sellBandLowerPct, opts.sellBandUpperPct]
      : []),
    ...(opts.sellMap?.keys() ?? []),
  ]);

  const seriesNames = [
    "date",
    "price",
    "price_ath",
    "halving_epoch",
    ...[...pctSet]
      .sort((a, b) => a - b)
      .map((pct) => costBasisSeriesName(pct)),
  ];

  const loaded = new Map(
    await Promise.all(
      seriesNames.map(async (name) => [name, await fetchSeries(opts, name)]),
    ),
  );

  const dates = loaded.get("date").data;
  const price = loaded.get("price").data;
  const ath = loaded.get("price_ath").data;
  const epoch = loaded.get("halving_epoch").data;
  const len = Math.min(dates.length, price.length, ath.length, epoch.length);
  const rows = [];

  for (let i = 0; i < len; i += 1) {
    const percentiles = new Map();
    for (const pct of pctSet) {
      const series = loaded.get(costBasisSeriesName(pct)).data;
      percentiles.set(pct, series[i]);
    }

    rows.push({
      date: dates[i],
      price: price[i],
      ath: ath[i],
      previousAth: i > 0 ? ath[i - 1] : ath[i],
      epoch: epoch[i],
      percentiles,
    });
  }

  return { rows };
}

async function fetchSeries(opts, series) {
  const url = new URL(`/api/series/${series}/day1`, normalizeBaseUrl(opts.baseUrl));
  url.searchParams.set("start", opts.dataStart);
  if (opts.end) url.searchParams.set("end", opts.end);

  const response = await fetch(url);
  if (!response.ok) {
    const text = await response.text();
    throw new Error(`Failed to fetch ${series}: ${response.status} ${text}`);
  }

  const json = await response.json();
  if (!Array.isArray(json.data)) {
    throw new Error(`Series ${series} did not return a data array`);
  }
  return json;
}

function normalizeBaseUrl(baseUrl) {
  return baseUrl.endsWith("/") ? baseUrl.slice(0, -1) : baseUrl;
}

function costBasisSeriesName(pct) {
  if (pct === 100) return "cost_basis_max";
  return `cost_basis_per_coin_pct${String(pct).padStart(2, "0")}`;
}

function resolveStartPoints(rows, opts) {
  if (opts.startSet === "custom") {
    return opts.starts.map((date) => ({
      label: date,
      date,
      kind: "custom",
      epoch: null,
      index: findDateIndex(rows, date),
    }));
  }

  if (opts.startSet === "single") {
    return [
      {
        label: opts.start,
        date: opts.start,
        kind: "single",
        epoch: null,
        index: findDateIndex(rows, opts.start),
      },
    ];
  }

  const firstIndex = findDateIndex(rows, opts.start);
  const byEpoch = new Map();
  rows.forEach((row, index) => {
    if (index < firstIndex) return;
    if (!Number.isFinite(row.price) || row.price <= 0) return;
    if (!byEpoch.has(row.epoch)) byEpoch.set(row.epoch, []);
    byEpoch.get(row.epoch).push({ row, index });
  });

  const points = [];
  for (const [epoch, items] of [...byEpoch.entries()].sort(([a], [b]) => a - b)) {
    if (!items.length) continue;

    const top = items.reduce((best, item) =>
      item.row.price > best.row.price ? item : best,
    );
    const bottom = items.reduce((best, item) =>
      item.row.price < best.row.price ? item : best,
    );

    points.push({
      label: `epoch-${epoch}-bottom`,
      date: bottom.row.date,
      kind: "bottom",
      epoch,
      index: bottom.index,
    });
    points.push({
      label: `epoch-${epoch}-top`,
      date: top.row.date,
      kind: "top",
      epoch,
      index: top.index,
    });
  }

  return uniqueByIndex(points).sort((a, b) => a.index - b.index);
}

function uniqueByIndex(points) {
  const seen = new Set();
  return points.filter((point) => {
    if (seen.has(point.index)) return false;
    seen.add(point.index);
    return true;
  });
}

function findDateIndex(rows, date) {
  const index = rows.findIndex((row) => row.date >= date);
  if (index === -1) throw new Error(`Start date ${date} is outside loaded data`);
  return index;
}

function simulateSignal(rows, startIndex, opts, sellEnabled) {
  let cash = opts.initialCash;
  let btc = 0;
  let { buyActive, sellArmed } = inferPhase(rows, startIndex, opts);
  let initialDeployActiveDays = 0;
  let contributed = opts.initialCash;
  let buys = 0;
  let sells = 0;
  let boughtUsd = 0;
  let soldUsd = 0;
  let minCash = cash;
  let cappedBuyDays = 0;
  let peakValue = cash;
  let maxDrawdown = 0;
  const baseDailyBuy = opts.dailyBuy ?? opts.monthlyTopup / DAYS_PER_MONTH;

  for (let i = startIndex; i < rows.length; i += 1) {
    const row = rows[i];
    if (i > startIndex && isMonthStart(row.date)) {
      cash += opts.monthlyTopup;
      contributed += opts.monthlyTopup;
    }

    const p50 = row.percentiles.get(opts.buyTriggerPct);
    if (Number.isFinite(p50) && p50 > 0 && row.price <= p50) {
      buyActive = true;
      sellArmed = false;
    }

    if (buyActive && isSellArmTouch(row, rows[i - 1], opts)) {
      buyActive = false;
      sellArmed = true;
    }

    if (sellEnabled && sellArmed) {
      const sellFraction = dailySellFraction(row, opts);
      if (sellFraction > 0 && btc > 0) {
        const btcToSell = btc * sellFraction;
        const usd = btcToSell * row.price;
        btc -= btcToSell;
        cash += usd;
        soldUsd += usd;
        sells += 1;
      }
    }

    if (buyActive && cash > 0) {
      const initialBudget =
        initialDeployActiveDays < opts.initialDeployDays && opts.initialDeployDays > 0
          ? opts.initialCash / opts.initialDeployDays
          : 0;
      const buyBudget = (baseDailyBuy + initialBudget) * buyWeight(row, opts);
      const usd = cappedBuyUsd(cash, buyBudget, opts);
      if (usd + 1e-9 < Math.min(cash, buyBudget)) {
        cappedBuyDays += 1;
      }
      if (usd > 0) {
        btc += usd / row.price;
        cash -= usd;
        boughtUsd += usd;
        buys += 1;
        initialDeployActiveDays += 1;
      }
    }

    const value = cash + btc * row.price;
    minCash = Math.min(minCash, cash);
    peakValue = Math.max(peakValue, value);
    maxDrawdown = Math.max(maxDrawdown, peakValue === 0 ? 0 : 1 - value / peakValue);
  }

  const finalRow = rows.at(-1);
  return {
    finalDate: finalRow.date,
    finalValue: cash + btc * finalRow.price,
    cash,
    minCash,
    btc,
    buys,
    cappedBuyDays,
    sells,
    boughtUsd,
    soldUsd,
    contributed,
    maxDrawdown,
  };
}

function cappedBuyUsd(cash, buyBudget, opts) {
  let cap = buyBudget;

  if (opts.buyRunwayMonths > 0) {
    cap = Math.min(cap, cash / (DAYS_PER_MONTH * opts.buyRunwayMonths));
  }

  if (opts.minCashReserveMonths > 0) {
    const reserve = opts.monthlyTopup * opts.minCashReserveMonths;
    cap = Math.min(cap, Math.max(0, cash - reserve));
  }

  return Math.min(cash, cap);
}

function inferPhase(rows, startIndex, opts) {
  let buyActive = false;
  let sellArmed = false;

  for (let i = 0; i < startIndex; i += 1) {
    const row = rows[i];
    const p50 = row.percentiles.get(opts.buyTriggerPct);

    if (Number.isFinite(p50) && p50 > 0 && row.price <= p50) {
      buyActive = true;
      sellArmed = false;
    }

    if (buyActive && isSellArmTouch(row, rows[i - 1], opts)) {
      buyActive = false;
      sellArmed = true;
    }
  }

  return { buyActive, sellArmed };
}

function simulateLumpAndTopup(rows, startIndex, opts) {
  let cash = opts.initialCash;
  let btc = cash / rows[startIndex].price;
  let contributed = cash;
  cash = 0;

  for (let i = startIndex + 1; i < rows.length; i += 1) {
    const row = rows[i];
    if (isMonthStart(row.date)) {
      contributed += opts.monthlyTopup;
      btc += opts.monthlyTopup / row.price;
    }
  }

  const finalRow = rows.at(-1);
  return {
    finalValue: btc * finalRow.price,
    contributed,
  };
}

function simulateSimpleDailyDca(rows, startIndex, opts) {
  let cash = opts.initialCash;
  let btc = 0;
  let contributed = cash;
  const baseDailyBuy = opts.dailyBuy ?? opts.monthlyTopup / DAYS_PER_MONTH;

  for (let i = startIndex; i < rows.length; i += 1) {
    const row = rows[i];
    if (i > startIndex && isMonthStart(row.date)) {
      cash += opts.monthlyTopup;
      contributed += opts.monthlyTopup;
    }

    const elapsedDays = i - startIndex;
    const initialBudget =
      elapsedDays < opts.initialDeployDays && opts.initialDeployDays > 0
        ? opts.initialCash / opts.initialDeployDays
        : 0;
    const usd = Math.min(cash, baseDailyBuy + initialBudget);
    if (usd > 0) {
      btc += usd / row.price;
      cash -= usd;
    }
  }

  const finalRow = rows.at(-1);
  return {
    finalValue: cash + btc * finalRow.price,
    contributed,
  };
}

function buyWeight(row, opts) {
  let weight = 0;
  for (const [pct, pctWeight] of [...opts.buyLevels.entries()].sort(
    ([a], [b]) => b - a,
  )) {
    const level = row.percentiles.get(pct);
    if (Number.isFinite(level) && level > 0 && row.price <= level) {
      weight = Math.max(weight, pctWeight);
    }
  }
  return weight;
}

function isSellArmTouch(row, previousRow, opts) {
  const level = row.percentiles.get(opts.sellArmPct);
  if (!Number.isFinite(level) || level <= 0) return false;

  if (opts.sellArmPct === 100) {
    const previousLevel = previousRow?.percentiles.get(opts.sellArmPct);
    return Number.isFinite(previousLevel) && level > previousLevel;
  }

  return row.price >= level;
}

function dailySellFraction(row, opts) {
  if (opts.sellRule === "percentile-band") {
    const lowerLevel = row.percentiles.get(opts.sellBandLowerPct);
    const upperLevel = row.percentiles.get(opts.sellBandUpperPct);
    if (
      !Number.isFinite(lowerLevel) ||
      !Number.isFinite(upperLevel) ||
      lowerLevel <= 0 ||
      upperLevel <= 0
    ) {
      return 0;
    }

    const lower = Math.min(lowerLevel, upperLevel);
    const upper = Math.max(lowerLevel, upperLevel);
    return row.price >= lower && row.price <= upper
      ? Math.min(1, opts.maxDailySellFraction * opts.sellBandMultiple)
      : 0;
  }

  if (opts.sellRule === "ath") {
    const threshold = row.previousAth * opts.sellAthMultiple;
    return Number.isFinite(threshold) && threshold > 0 && row.price >= threshold
      ? opts.maxDailySellFraction
      : 0;
  }

  const totalWeight = [...opts.sellMap.values()].reduce((sum, weight) => sum + weight, 0);
  if (totalWeight <= 0) return 0;

  let triggeredWeight = 0;
  for (const [pct, multiplier] of opts.sellMap) {
    const level = row.percentiles.get(pct);
    if (Number.isFinite(level) && level > 0 && row.price >= level * multiplier) {
      triggeredWeight += multiplier;
    }
  }

  if (triggeredWeight <= 0) return 0;
  return opts.maxDailySellFraction * (triggeredWeight / totalWeight);
}

function isMonthStart(date) {
  return date.endsWith("-01");
}

function pct(value) {
  return value * 100;
}

function printTable(results, opts) {
  console.log(
    [
      `Data: ${opts.start}${opts.end ? ` to ${opts.end}` : " to latest"}`,
      `Cash: ${usd(opts.initialCash)} initial + ${usd(opts.monthlyTopup)} monthly`,
      `Buy runway: ${formatBuyRunway(opts)}`,
      `Buy trigger: p${opts.buyTriggerPct} touch starts DCA-in; p${opts.sellArmPct} increase stops it`,
      `Sell: ${
      opts.mode === "hold"
          ? "disabled"
          : opts.sellRule === "ath"
            ? `optional, previous ATH x${formatNumber(opts.sellAthMultiple, 2)}, max ${formatNumber(opts.maxDailySellFraction * 100, 3)}% BTC/day`
            : opts.sellRule === "percentile-band"
              ? `optional, armed by p${opts.sellArmPct} touch, p${opts.sellBandLowerPct}-p${opts.sellBandUpperPct}, sell size x${formatNumber(opts.sellBandMultiple, 2)}, base ${formatNumber(opts.maxDailySellFraction * 100, 3)}% BTC/day`
            : `optional, percentile map, max ${formatNumber(opts.maxDailySellFraction * 100, 3)}% BTC/day`
      }`,
      "",
    ].join("\n"),
  );

  const rows = results.map((result) => ({
    start: `${result.start_label} ${result.start_date}`,
    mode: result.mode,
    contributed: usd(result.contributed),
    final: usd(result.final_value),
    ret: `${formatNumber(result.return_pct, 2)}%`,
    cash: usd(result.cash),
    min_cash: usd(result.min_cash),
    btc: formatNumber(result.btc, 6),
    buys: String(result.buys),
    capped_buys: String(result.capped_buy_days),
    sells: String(result.sells),
    dd: `${formatNumber(result.max_drawdown_pct, 2)}%`,
    vs_lump: `${formatNumber(result.lump_delta_pct, 2)}%`,
    vs_dca: `${formatNumber(result.daily_dca_delta_pct, 2)}%`,
  }));

  printFixedWidthTable(rows, [
    ["start", "start"],
    ["mode", "mode"],
    ["contributed", "contributed"],
    ["final", "final"],
    ["ret", "return"],
    ["cash", "cash"],
    ["min_cash", "min cash"],
    ["btc", "btc"],
    ["buys", "buys"],
    ["capped_buys", "capped buys"],
    ["sells", "sells"],
    ["dd", "max dd"],
    ["vs_lump", "vs lump"],
    ["vs_dca", "vs dca"],
  ]);
}

function formatBuyRunway(opts) {
  const parts = [];
  if (opts.buyRunwayMonths > 0) {
    parts.push(
      `daily cap preserves ${formatNumber(opts.buyRunwayMonths, 2)} month(s)`,
    );
  }
  if (opts.minCashReserveMonths > 0) {
    parts.push(
      `cash floor ${formatNumber(opts.minCashReserveMonths, 2)} month(s) top-up`,
    );
  }
  return parts.length ? parts.join("; ") : "none";
}

function printCsv(results) {
  const keys = Object.keys(results[0] ?? {});
  console.log(keys.join(","));
  for (const result of results) {
    console.log(
      keys
        .map((key) => {
          const value = result[key];
          if (typeof value === "string") return `"${value.replaceAll('"', '""')}"`;
          return value;
        })
        .join(","),
    );
  }
}

function printFixedWidthTable(rows, columns) {
  const widths = new Map();
  for (const [key, label] of columns) {
    widths.set(
      key,
      Math.max(label.length, ...rows.map((row) => String(row[key]).length)),
    );
  }

  const formatRow = (row) =>
    columns
      .map(([key]) => String(row[key]).padEnd(widths.get(key)))
      .join("  ");

  console.log(formatRow(Object.fromEntries(columns.map(([key, label]) => [key, label]))));
  console.log(
    columns.map(([key]) => "-".repeat(widths.get(key))).join("  "),
  );
  for (const row of rows) console.log(formatRow(row));
}

function usd(value) {
  return `$${formatNumber(value, 2)}`;
}

function formatNumber(value, digits) {
  return new Intl.NumberFormat("en-US", {
    minimumFractionDigits: digits,
    maximumFractionDigits: digits,
  }).format(value);
}

main().catch((error) => {
  console.error(error.message);
  process.exitCode = 1;
});
