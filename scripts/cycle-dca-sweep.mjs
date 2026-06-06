#!/usr/bin/env node

const PCTS = [100, 95, 90, 85, 80, 75, 70, 65, 60, 55, 50, 45, 40];
const DAYS_PER_MONTH = 365.2425 / 12;

function parseArgs(argv) {
  const opts = {
    baseUrl: "https://bitview.space",
    dataStart: "2014-01-01",
    end: "2026-06-06",
    startFrom: "2017-12-01",
    startTo: "2025-08-01",
    starts: null,
    initialCash: 10_000,
    monthlyTopup: 1_000,
    initialDeployDays: 365,
    topMin: 0.25,
    topMax: 8,
    bottomMin: 0.25,
    bottomMax: 16,
    step: 0.25,
    sellMin: 0,
    sellMax: 10,
    sellStep: 1,
    topN: 12,
  };

  for (let i = 0; i < argv.length; i += 1) {
    const arg = argv[i];
    const next = () => {
      i += 1;
      if (i >= argv.length) throw new Error(`Missing value for ${arg}`);
      return argv[i];
    };

    if (arg === "--base-url") opts.baseUrl = next();
    else if (arg === "--data-start") opts.dataStart = next();
    else if (arg === "--end") opts.end = next();
    else if (arg === "--start-from") opts.startFrom = next();
    else if (arg === "--start-to") opts.startTo = next();
    else if (arg === "--starts") opts.starts = next().split(",").filter(Boolean);
    else if (arg === "--top-min") opts.topMin = Number(next());
    else if (arg === "--top-max") opts.topMax = Number(next());
    else if (arg === "--bottom-min") opts.bottomMin = Number(next());
    else if (arg === "--bottom-max") opts.bottomMax = Number(next());
    else if (arg === "--step") opts.step = Number(next());
    else if (arg === "--sell-min") opts.sellMin = Number(next());
    else if (arg === "--sell-max") opts.sellMax = Number(next());
    else if (arg === "--sell-step") opts.sellStep = Number(next());
    else if (arg === "--top-n") opts.topN = Number(next());
    else throw new Error(`Unknown argument: ${arg}`);
  }

  return opts;
}

async function main() {
  const opts = parseArgs(process.argv.slice(2));
  const rows = await loadRows(opts);
  const starts = resolveStarts(rows, opts);
  const startStates = starts.map((start) => ({
    ...start,
    phase: inferPhase(rows, start.index),
    dca: simulateSimpleDailyDca(rows, start.index, opts),
    lump: simulateLumpAndTopup(rows, start.index, opts),
  }));
  const baselineWeights = linearWeights(1, 7);
  const baseline = scoreVariant(rows, startStates, baselineWeights, 1, opts);

  const candidates = [];
  for (const top of range(opts.topMin, opts.topMax, opts.step)) {
    const bottomStart = Math.max(opts.bottomMin, top);
    for (const bottom of range(bottomStart, opts.bottomMax, opts.step)) {
      const weights = linearWeights(top, bottom);
      for (const sellMultiple of range(opts.sellMin, opts.sellMax, opts.sellStep)) {
        const score = scoreVariant(rows, startStates, weights, sellMultiple, opts);
        candidates.push({
          top,
          bottom,
          sellMultiple,
          ...score,
        });
      }
    }
  }

  candidates.sort((a, b) => b.avgVsDca - a.avgVsDca);

  console.log(
    [
      `Starts: ${starts[0].date} to ${starts.at(-1).date} (${starts.length})`,
      `Data end: ${rows.at(-1).date}`,
      `Baseline p100=1 -> p40=7, sell x1: avg vs DCA ${fmtPct(baseline.avgVsDca)}, median vs DCA ${fmtPct(baseline.medianVsDca)}, worst vs DCA ${fmtPct(baseline.worstVsDca)}, avg final ${usd(baseline.avgFinal)}`,
      "",
      "rank,p100,p40,sell_x,avg_vs_dca,median_vs_dca,worst_vs_dca,avg_vs_lump,avg_final,wins_vs_baseline",
    ].join("\n"),
  );

  for (const [index, candidate] of candidates.slice(0, opts.topN).entries()) {
    console.log(
      [
        index + 1,
        fmt(candidate.top),
        fmt(candidate.bottom),
        fmt(candidate.sellMultiple),
        fmtPct(candidate.avgVsDca),
        fmtPct(candidate.medianVsDca),
        fmtPct(candidate.worstVsDca),
        fmtPct(candidate.avgVsLump),
        usd(candidate.avgFinal),
        candidate.finals.filter((value, i) => value > baseline.finals[i]).length,
      ].join(","),
    );
  }
}

async function loadRows(opts) {
  const names = [
    "date",
    "price",
    "price_ath",
    ...PCTS.map(costBasisSeriesName),
  ];
  const loaded = new Map(
    await Promise.all(names.map(async (name) => [name, await fetchSeries(opts, name)])),
  );
  const len = Math.min(...names.map((name) => loaded.get(name).data.length));
  const rows = [];

  for (let i = 0; i < len; i += 1) {
    const row = {
      date: loaded.get("date").data[i],
      price: loaded.get("price").data[i],
      ath: loaded.get("price_ath").data[i],
      previousAth: i > 0 ? loaded.get("price_ath").data[i - 1] : loaded.get("price_ath").data[i],
      levels: {},
    };
    for (const pct of PCTS) {
      row.levels[pct] = loaded.get(costBasisSeriesName(pct)).data[i];
    }
    rows.push(row);
  }

  return rows;
}

async function fetchSeries(opts, series) {
  const url = new URL(`/api/series/${series}/day1`, normalizeBaseUrl(opts.baseUrl));
  url.searchParams.set("start", opts.dataStart);
  if (opts.end) url.searchParams.set("end", opts.end);

  const response = await fetch(url);
  if (!response.ok) throw new Error(`Failed to fetch ${series}: ${response.status}`);
  const json = await response.json();
  if (!Array.isArray(json.data)) throw new Error(`${series} returned no data array`);
  return json;
}

function resolveStarts(rows, opts) {
  const dates = opts.starts ?? monthStarts(opts.startFrom, opts.startTo);
  return dates.map((date) => ({
    date,
    index: findDateIndex(rows, date),
  }));
}

function monthStarts(from, to) {
  const dates = [];
  for (
    const cursor = new Date(`${from}T00:00:00Z`), end = new Date(`${to}T00:00:00Z`);
    cursor <= end;
    cursor.setUTCMonth(cursor.getUTCMonth() + 1)
  ) {
    dates.push(cursor.toISOString().slice(0, 10));
  }
  return dates;
}

function findDateIndex(rows, date) {
  const index = rows.findIndex((row) => row.date >= date);
  if (index === -1) throw new Error(`Date ${date} is outside loaded data`);
  return index;
}

function inferPhase(rows, startIndex) {
  let buyActive = false;
  let sellArmed = false;

  for (let i = 0; i < startIndex; i += 1) {
    const row = rows[i];
    const p50 = row.levels[50];

    if (Number.isFinite(p50) && p50 > 0 && row.price <= p50) {
      buyActive = true;
      sellArmed = false;
    }

    if (buyActive && isP100Increase(row, rows[i - 1])) {
      buyActive = false;
      sellArmed = true;
    }
  }

  return { buyActive, sellArmed };
}

function scoreVariant(rows, starts, weights, sellMultiple, opts) {
  const finals = [];
  const vsDca = [];
  const vsLump = [];
  const drawdowns = [];

  for (const start of starts) {
    const result = simulateSignal(rows, start.index, start.phase, weights, sellMultiple, opts);
    finals.push(result.finalValue);
    vsDca.push(result.finalValue / start.dca.finalValue - 1);
    vsLump.push(result.finalValue / start.lump.finalValue - 1);
    drawdowns.push(result.maxDrawdown);
  }

  return {
    finals,
    avgFinal: avg(finals),
    avgVsDca: avg(vsDca),
    medianVsDca: median(vsDca),
    worstVsDca: Math.min(...vsDca),
    avgVsLump: avg(vsLump),
    avgDrawdown: avg(drawdowns),
  };
}

function simulateSignal(rows, startIndex, phase, weights, sellMultiple, opts) {
  let cash = opts.initialCash;
  let btc = 0;
  let buyActive = phase.buyActive;
  let sellArmed = phase.sellArmed;
  let initialDeployActiveDays = 0;
  let peakValue = cash;
  let maxDrawdown = 0;
  const baseDailyBuy = opts.monthlyTopup / DAYS_PER_MONTH;

  for (let i = startIndex; i < rows.length; i += 1) {
    const row = rows[i];
    if (i > startIndex && isMonthStart(row.date)) cash += opts.monthlyTopup;

    const p50 = row.levels[50];
    if (Number.isFinite(p50) && p50 > 0 && row.price <= p50) {
      buyActive = true;
      sellArmed = false;
    }

    if (buyActive && isP100Increase(row, rows[i - 1])) {
      buyActive = false;
      sellArmed = true;
    }

    if (sellMultiple > 0 && sellArmed && isInsideSellBand(row) && btc > 0) {
      const btcToSell = btc * Math.min(1, 0.005 * sellMultiple);
      btc -= btcToSell;
      cash += btcToSell * row.price;
    }

    if (buyActive && cash > 0) {
      const initialBudget =
        initialDeployActiveDays < opts.initialDeployDays
          ? opts.initialCash / opts.initialDeployDays
          : 0;
      const usd = Math.min(
        cash,
        (baseDailyBuy + initialBudget) * buyWeight(row, weights),
      );
      if (usd > 0) {
        btc += usd / row.price;
        cash -= usd;
        initialDeployActiveDays += 1;
      }
    }

    const value = cash + btc * row.price;
    peakValue = Math.max(peakValue, value);
    maxDrawdown = Math.max(maxDrawdown, peakValue === 0 ? 0 : 1 - value / peakValue);
  }

  return {
    finalValue: cash + btc * rows.at(-1).price,
    maxDrawdown,
  };
}

function simulateLumpAndTopup(rows, startIndex, opts) {
  let btc = opts.initialCash / rows[startIndex].price;
  for (let i = startIndex + 1; i < rows.length; i += 1) {
    if (isMonthStart(rows[i].date)) btc += opts.monthlyTopup / rows[i].price;
  }
  return { finalValue: btc * rows.at(-1).price };
}

function simulateSimpleDailyDca(rows, startIndex, opts) {
  let cash = opts.initialCash;
  let btc = 0;
  const baseDailyBuy = opts.monthlyTopup / DAYS_PER_MONTH;

  for (let i = startIndex; i < rows.length; i += 1) {
    const row = rows[i];
    if (i > startIndex && isMonthStart(row.date)) cash += opts.monthlyTopup;

    const elapsedDays = i - startIndex;
    const initialBudget =
      elapsedDays < opts.initialDeployDays ? opts.initialCash / opts.initialDeployDays : 0;
    const usd = Math.min(cash, baseDailyBuy + initialBudget);
    if (usd > 0) {
      btc += usd / row.price;
      cash -= usd;
    }
  }

  return { finalValue: cash + btc * rows.at(-1).price };
}

function linearWeights(top, bottom) {
  const weights = {};
  for (let i = 0; i < PCTS.length; i += 1) {
    weights[PCTS[i]] = top + ((bottom - top) * i) / (PCTS.length - 1);
  }
  return weights;
}

function buyWeight(row, weights) {
  let weight = 0;
  for (const pct of PCTS) {
    const level = row.levels[pct];
    if (Number.isFinite(level) && level > 0 && row.price <= level) {
      weight = Math.max(weight, weights[pct]);
    }
  }
  return weight;
}

function isP100Increase(row, previousRow) {
  const level = row.levels[100];
  const previousLevel = previousRow?.levels[100];
  return Number.isFinite(level) && Number.isFinite(previousLevel) && level > previousLevel;
}

function isInsideSellBand(row) {
  const p95 = row.levels[95];
  const p100 = row.levels[100];
  if (!Number.isFinite(p95) || !Number.isFinite(p100) || p95 <= 0 || p100 <= 0) {
    return false;
  }
  const lower = Math.min(p95, p100);
  const upper = Math.max(p95, p100);
  return row.price >= lower && row.price <= upper;
}

function costBasisSeriesName(pct) {
  if (pct === 100) return "cost_basis_max";
  return `cost_basis_per_coin_pct${String(pct).padStart(2, "0")}`;
}

function normalizeBaseUrl(baseUrl) {
  return baseUrl.endsWith("/") ? baseUrl.slice(0, -1) : baseUrl;
}

function isMonthStart(date) {
  return date.endsWith("-01");
}

function range(min, max, step) {
  const values = [];
  const scale = 1 / step;
  for (let value = min; value <= max + step / 10; value += step) {
    values.push(Math.round(value * scale) / scale);
  }
  return values;
}

function avg(values) {
  return values.reduce((sum, value) => sum + value, 0) / values.length;
}

function median(values) {
  const sorted = [...values].sort((a, b) => a - b);
  return sorted[Math.floor(sorted.length / 2)];
}

function fmt(value) {
  return String(Number(value.toFixed(2)));
}

function fmtPct(value) {
  return `${(value * 100).toFixed(2)}%`;
}

function usd(value) {
  return `$${(value / 1000).toFixed(1)}k`;
}

main().catch((error) => {
  console.error(error.message);
  process.exitCode = 1;
});
