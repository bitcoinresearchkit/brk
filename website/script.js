// @ts-check

/**
 * @import { FilePath, PartialPreset, PartialPresetFolder, PartialPresetTree, Preset, PresetFolder, Series, PriceSeriesType, ResourceDataset, Scale, SerializedPresetsHistory, TimeRange, Unit, Marker, Weighted, DatasetPath, OHLC, FetchedJSON, DatasetValue, FetchedResult, AnyDatasetPath, SeriesBlueprint, BaselineSpecificSeriesBlueprint, CandlestickSpecificSeriesBlueprint, LineSpecificSeriesBlueprint, SpecificSeriesBlueprintWithChart, Signal, Color, SettingsTheme } from "./types/self"
 * @import * as _ from "./libraries/ufuzzy/types"
 * @import { DeepPartial, ChartOptions, IChartApi, IHorzScaleBehavior, WhitespaceData, SingleValueData, ISeriesApi, Time, LogicalRange, SeriesMarker, CandlestickData, SeriesType, BaselineStyleOptions, SeriesOptionsCommon } from "./libraries/lightweight-charts/types"
 * @import { DatePath, HeightPath } from "./types/paths";
 * @import { SignalOptions, untrack as Untrack } from "./libraries/solid-signals/types/core.js"
 * @import { getOwner as GetOwner, onCleanup as OnCleanup, Owner } from "./libraries/solid-signals/types/owner.js"
 * @import { createSignal as CreateSignal, createEffect as CreateEffect, Accessor, Setter, createMemo as CreateMemo, createRoot as CreateRoot, runWithOwner as RunWithOwner } from "./libraries/solid-signals/types/signals.js";
 */

import {
  createSignal as _createSignal,
  createEffect as _createEffect,
  createMemo as _createMemo,
  createRoot as _createRoot,
  untrack as _untrack,
  getOwner as _getOwner,
  runWithOwner as _runWithOwner,
  onCleanup as _onCleanup,
} from "./libraries/solid-signals/script.js";

const createSolidSignal = /** @type {CreateSignal} */ (_createSignal);
const createEffect = /** @type {CreateEffect} */ (_createEffect);
const createMemo = /** @type {CreateMemo} */ (_createMemo);
const createRoot = /** @type {CreateRoot} */ (_createRoot);
const untrack = /** @type {Untrack} */ (_untrack);
const getOwner = /** @type {GetOwner} */ (_getOwner);
const runWithOwner = /** @type {RunWithOwner} */ (_runWithOwner);
const onCleanup = /** @type {OnCleanup} */ (_onCleanup);
/**
 * @template T
 * @param {T} initialValue
 * @param {SignalOptions<T>} [options]
 * @returns {Signal<T>}
 */
function createSignal(initialValue, options) {
  const [get, set] = createSolidSignal(initialValue, options);
  // @ts-ignore
  get.set = set;
  // @ts-ignore
  return get;
}

/**
 * @param {string} id
 * @returns {HTMLElement}
 */
function getElementById(id) {
  const element = window.document.getElementById(id);
  if (!element) throw `Element with id = "${id}" should exist`;
  return element;
}

/**
 * @param {HTMLElement} parent
 * @param {HTMLElement} child
 * @param {number} index
 */
function insertElementAtIndex(parent, child, index) {
  if (!index) index = 0;
  if (index >= parent.children.length) {
    parent.appendChild(child);
  } else {
    parent.insertBefore(child, parent.children[index]);
  }
}

/**
 * @param {string} s
 * @returns {string}
 */
function stringToId(s) {
  return s.replace(/\W/g, " ").trim().replace(/ +/g, "-").toLowerCase();
}

/**
 * @param {VoidFunction} callback
 * @param {number} [timeout = 1]
 */
function runWhenIdle(callback, timeout = 1) {
  if ("requestIdleCallback" in window) {
    requestIdleCallback(callback);
  } else {
    setTimeout(callback, timeout);
  }
}

/**
 * @param {Date} date
 * @returns {string}
 */
function dateToString(date) {
  return date.toJSON().split("T")[0];
}

/**
 * @param {Date} oldest
 * @param {Date} youngest
 * @returns {number}
 */
function getNumberOfDaysBetweenTwoDates(oldest, youngest) {
  return Math.round(
    Math.abs((youngest.getTime() - oldest.getTime()) / ONE_DAY_IN_MS)
  );
}

/**
 *
 * @template {(...args: any[]) => any} F
 * @param {F} callback
 * @param {number} [wait=250]
 */
function debounce(callback, wait = 250) {
  /** @type {number | undefined} */
  let timeoutId;
  /** @type {Parameters<F>} */
  let latestArgs;

  return (/** @type {Parameters<F>} */ ...args) => {
    latestArgs = args;

    if (!timeoutId) {
      timeoutId = window.setTimeout(async () => {
        await callback(...latestArgs);

        timeoutId = undefined;
      }, wait);
    }
  };
}

const urlParamsHelpers = {
  whitelist: ["from", "to"],
  /**
   * @param {Object} args
   * @param {URLSearchParams} [args.urlParams]
   * @param {string} [args.pathname]
   */
  replaceHistory({ urlParams, pathname }) {
    urlParams ||= new URLSearchParams(window.location.search);
    pathname ||= window.location.pathname;

    window.history.replaceState(
      null,
      "",
      `${pathname}?${urlParams.toString()}`
    );
  },
  /**
   * @param {string} [pathname]
   */
  reset(pathname) {
    const urlParams = new URLSearchParams();

    [...new URLSearchParams(window.location.search).entries()]
      .filter(([key, _]) => this.whitelist.includes(key))
      .forEach(([key, value]) => {
        urlParams.set(key, value);
      });

    this.replaceHistory({ urlParams, pathname });
  },
  /**
   * @param {string} key
   * @param {string | boolean | undefined} value
   */
  write(key, value) {
    const urlParams = new URLSearchParams(window.location.search);

    if (value !== undefined) {
      urlParams.set(key, String(value));
    } else {
      urlParams.delete(key);
    }

    this.replaceHistory({ urlParams });
  },
  /**
   * @param {string} key
   */
  remove(key) {
    this.write(key, undefined);
  },
  /**
   *
   * @param {string} key
   * @returns {boolean | null}
   */
  readBool(key) {
    const urlParams = new URLSearchParams(window.location.search);

    const parameter = urlParams.get(key);

    if (parameter) {
      return utils.isSerializedBooleanTrue(parameter);
    }

    return null;
  },
};

const localeStorageHelpers = {
  /**
   * @param {string} key
   */
  readBool(key) {
    const saved = localStorage.getItem(key);
    if (saved) {
      return utils.isSerializedBooleanTrue(saved);
    }
    return null;
  },
  /**
   * @param {string} key
   * @param {string | boolean | undefined} value
   */
  write(key, value) {
    value !== undefined && value !== null
      ? localStorage.setItem(key, String(value))
      : localStorage.removeItem(key);
  },
  /**
   * @param {string} key
   */
  remove(key) {
    this.write(key, undefined);
  },
};

const utils = {
  /**
   * @param {string} serialized
   * @returns {boolean}
   */
  isSerializedBooleanTrue(serialized) {
    return serialized === "true" || serialized === "1";
  },
};

const dom = {
  head: window.document.getElementsByTagName("head")[0],
  /**
   * @param {string} name
   */
  queryOrCreateMetaElement(name) {
    let meta = /** @type {HTMLMetaElement | null} */ (
      window.document.querySelector(`meta[name="${name}"]`)
    );

    if (!meta) {
      meta = window.document.createElement("meta");
      meta.name = name;
      this.head.appendChild(meta);
    }
    return meta;
  },
};

const env = (function initEnv() {
  const standalone =
    "standalone" in window.navigator && !!window.navigator.standalone;
  const userAgent = navigator.userAgent.toLowerCase();
  const isChrome = userAgent.includes("chrome");
  const safari = userAgent.includes("safari");
  const safariOnly = safari && !isChrome;
  const macOS = userAgent.includes("mac os");
  const iphone = userAgent.includes("iphone");
  const ipad = userAgent.includes("ipad");

  return {
    standalone,
    userAgent,
    isChrome,
    safari,
    safariOnly,
    macOS,
    iphone,
    ipad,
  };
})();

const ONE_SECOND_IN_MS = 1_000;
const FIVE_SECOND_IN_MS = 5 * ONE_SECOND_IN_MS;
const TEN_SECOND_IN_MS = 2 * FIVE_SECOND_IN_MS;
const ONE_MINUTE_IN_MS = 6 * TEN_SECOND_IN_MS;
const FIVE_MINUTES_IN_MS = 5 * ONE_MINUTE_IN_MS;
const TEN_MINUTES_IN_MS = 2 * FIVE_MINUTES_IN_MS;
const ONE_HOUR_IN_MS = 6 * TEN_MINUTES_IN_MS;
const ONE_DAY_IN_MS = 24 * ONE_HOUR_IN_MS;

const mainFrames = getElementById("frames");
const chartFrameSelectorLabelId = `selected-frame-selector-label`;
const chartLabel = getElementById(chartFrameSelectorLabelId);
const foldersLabel = getElementById(`folders-frame-selector-label`);
const searchLabel = getElementById(`search-frame-selector-label`);
const searchFrame = getElementById("search-frame");
const foldersFrame = getElementById("folders-frame");
const selectedFrame = getElementById("selected-frame");
const historyList = getElementById("history-list");
const searchInput = /** @type {HTMLInputElement} */ (
  getElementById("search-input")
);
const searchSmall = getElementById("search-small");
const searchResults = getElementById("search-results");
const presetTitle = getElementById("preset-title");
const presetDescription = getElementById("preset-description");
const foldersFilterAllCount = getElementById("folders-filter-all-count");
const foldersFilterFavoritesCount = getElementById(
  "folders-filter-favorites-count"
);
const foldersFilterNewCount = getElementById("folders-filter-new-count");
const chartListElement = getElementById("chart-list");
const legendElement = getElementById("legend");
const bodyStyle = getComputedStyle(window.document.documentElement);
const buttonFavorite = getElementById("button-favorite");
const buttonShare = getElementById("button-share");
const timeScaleDateButtons = getElementById("timescale-date-buttons");
const timeScaleHeightButtons = getElementById("timescale-height-buttons");

const dark = createSignal(true);

function initFrames() {
  const localStorageKey = "checked-frame-selector-label";
  let selectedFrameLabel = localStorage.getItem(localStorageKey);

  const fieldset = window.document.getElementById("frame-selectors");
  if (!fieldset) throw "Fieldset should exist";

  const children = Array.from(fieldset.children);

  /** @type {HTMLElement | undefined} */
  let focusedSection = undefined;

  for (let i = 0; i < children.length; i++) {
    const element = children[i];

    switch (element.tagName) {
      case "LABEL": {
        element.addEventListener("click", (event) => {
          const id = element.id;

          event.stopPropagation();
          event.preventDefault();

          const forId = element.getAttribute("for") || "";
          const input = /** @type {HTMLInputElement | undefined} */ (
            window.document.getElementById(forId)
          );

          if (!input) throw "Shouldn't be possible";
          selectedFrameLabel = id;
          localStorage.setItem(localStorageKey, id);
          input.checked = true;

          const sectionId = element.id.split("-").splice(0, 2).join("-");

          const section = window.document.getElementById(sectionId);

          if (!section) {
            console.log(sectionId, section);
            throw "Section should exist";
          }

          if (section === focusedSection) {
            return;
          }

          section.hidden = false;
          if (focusedSection?.parentElement === mainFrames) {
            focusedSection.hidden = true;
          }
          focusedSection = section;
        });
        break;
      }
    }
  }

  if (selectedFrameLabel) {
    const frameLabel = window.document.getElementById(selectedFrameLabel);
    if (!frameLabel) throw "Frame should exist";
    frameLabel.click();
  } else {
    chartLabel.click();
  }

  // When going from mobile view to desktop view, if selected frame was open, go to the folders frame
  new IntersectionObserver((entries) => {
    for (let i = 0; i < entries.length; i++) {
      if (
        !entries[i].isIntersecting &&
        entries[i].target === chartLabel &&
        selectedFrameLabel === chartFrameSelectorLabelId
      ) {
        foldersLabel.click();
      }
    }
  }).observe(chartLabel);

  window.document.addEventListener("keydown", (event) => {
    switch (event.key) {
      case "Escape": {
        event.stopPropagation();
        event.preventDefault();
        foldersLabel.click();
        break;
      }
      case "/": {
        if (window.document.activeElement === searchInput) {
          return;
        }

        event.stopPropagation();
        event.preventDefault();
        searchLabel.click();
        searchInput.focus();
        break;
      }
    }
  });
}
initFrames();

function createColors() {
  function lightRed() {
    const tailwindRed300 = "#fca5a5";
    const tailwindRed800 = "#991b1b";
    return dark() ? tailwindRed300 : tailwindRed800;
  }
  function red() {
    return "#e63636"; // 550
  }
  function darkRed() {
    const tailwindRed900 = "#7f1d1d";
    const tailwindRed100 = "#fee2e2";
    return dark() ? tailwindRed900 : tailwindRed100;
  }
  function orange() {
    return bodyStyle.getPropertyValue("--orange"); // 550
  }
  function darkOrange() {
    const tailwindOrange900 = "#7c2d12";
    const tailwindOrange100 = "#ffedd5";
    return dark() ? tailwindOrange900 : tailwindOrange100;
  }
  function amber() {
    return "#e78a05"; // 550
  }
  function yellow() {
    return "#db9e03"; // 550
  }
  function lime() {
    return "#74b713"; // 550
  }
  function green() {
    return "#1cb454";
  }
  function darkGreen() {
    const tailwindGreen900 = "#14532d";
    const tailwindGreen100 = "#dcfce7";
    return dark() ? tailwindGreen900 : tailwindGreen100;
  }
  function emerald() {
    return "#0ba775";
  }
  function darkEmerald() {
    const tailwindEmerald900 = "#064e3b";
    const tailwindEmerald100 = "#d1fae5";
    return dark() ? tailwindEmerald900 : tailwindEmerald100;
  }
  function teal() {
    return "#10a697"; // 550
  }
  function cyan() {
    return "#06a3c3"; // 550
  }
  function sky() {
    return "#0794d8"; // 550
  }
  function blue() {
    return "#2f73f1"; // 550
  }
  function indigo() {
    return "#5957eb";
  }
  function violet() {
    return "#834cf2";
  }
  function purple() {
    return "#9d45f0";
  }
  function fuchsia() {
    return "#cc37e1";
  }
  function pink() {
    return "#e53882";
  }
  function rose() {
    return "#ea3053";
  }
  function darkRose() {
    const tailwindRose900 = "#881337";
    const tailwindRose100 = "#ffe4e6";
    return dark() ? tailwindRose900 : tailwindRose100;
  }

  function darkWhite() {
    const _ = dark();
    return bodyStyle.getPropertyValue("--off-color");
  }

  function gray() {
    const _ = dark();
    return bodyStyle.getPropertyValue("--border-color");
  }

  function white() {
    const _ = dark();
    return bodyStyle.getPropertyValue("--color");
  }

  function black() {
    const _ = dark();
    return bodyStyle.getPropertyValue("--background-color");
  }

  return {
    white,
    black,
    darkWhite,
    gray,
    lightBitcoin: yellow,
    bitcoin: orange,
    darkBitcoin: darkOrange,
    lightDollars: lime,
    dollars: emerald,
    darkDollars: darkEmerald,

    _1d: lightRed,
    _1w: red,
    _8d: orange,
    _13d: amber,
    _21d: yellow,
    _1m: lime,
    _34d: green,
    _55d: emerald,
    _89d: teal,
    _144d: cyan,
    _6m: sky,
    _1y: blue,
    _2y: indigo,
    _200w: violet,
    _4y: purple,
    _10y: fuchsia,

    p2pk: lime,
    p2pkh: violet,
    p2sh: emerald,
    p2wpkh: cyan,
    p2wsh: pink,
    p2tr: blue,
    crab: red,
    fish: lime,
    humpback: violet,
    plankton: emerald,
    shark: cyan,
    shrimp: pink,
    whale: blue,
    megalodon: purple,
    realizedPrice: orange,
    oneMonthHolders: cyan,
    threeMonthsHolders: lime,
    sth: yellow,
    sixMonthsHolder: red,
    oneYearHolders: pink,
    twoYearsHolders: purple,
    lth: fuchsia,
    balancedPrice: yellow,
    cointimePrice: yellow,
    trueMarketMeanPrice: blue,
    vaultedPrice: green,
    cvdd: lime,
    terminalPrice: red,
    loss: red,
    darkLoss: darkRed,
    profit: green,
    darkProfit: darkGreen,
    thermoCap: green,
    investorCap: rose,
    realizedCap: orange,
    ethereum: indigo,
    usdt: emerald,
    usdc: blue,
    ust: red,
    busd: yellow,
    usdd: emerald,
    frax: gray,
    dai: amber,
    tusd: indigo,
    pyusd: blue,
    darkLiveliness: darkRose,
    liveliness: rose,
    vaultedness: green,
    activityToVaultednessRatio: violet,
    up_to_1d: lightRed,
    up_to_1w: red,
    up_to_1m: orange,
    up_to_2m: orange,
    up_to_3m: orange,
    up_to_4m: orange,
    up_to_5m: orange,
    up_to_6m: orange,
    up_to_1y: orange,
    up_to_2y: orange,
    up_to_3y: orange,
    up_to_4y: orange,
    up_to_5y: orange,
    up_to_7y: orange,
    up_to_10y: orange,
    up_to_15y: orange,
    from_10y_to_15y: purple,
    from_7y_to_10y: violet,
    from_5y_to_7y: indigo,
    from_3y_to_5y: sky,
    from_2y_to_3y: teal,
    from_1y_to_2y: green,
    from_6m_to_1y: lime,
    from_3m_to_6m: yellow,
    from_1m_to_3m: amber,
    from_1w_to_1m: orange,
    from_1d_to_1w: red,
    from_1y: green,
    from_2y: teal,
    from_4y: indigo,
    from_10y: violet,
    from_15y: fuchsia,
    coinblocksCreated: purple,
    coinblocksDestroyed: red,
    coinblocksStored: green,
    momentum: [green, yellow, red],
    momentumGreen: green,
    momentumYellow: yellow,
    momentumRed: red,
    probability0_1p: red,
    probability0_5p: orange,
    probability1p: yellow,
    year_2009: yellow,
    year_2010: yellow,
    year_2011: yellow,
    year_2012: yellow,
    year_2013: yellow,
    year_2014: yellow,
    year_2015: yellow,
    year_2016: yellow,
    year_2017: yellow,
    year_2018: yellow,
    year_2019: yellow,
    year_2020: yellow,
    year_2021: yellow,
    year_2022: yellow,
    year_2023: yellow,
    year_2024: yellow,
  };
}
const colors = createColors();

function initEverythingRelatedToPresets() {
  /** @type {Signal<Preset>} */
  const selected = createSignal(/** @type {any} */ (undefined));
  const selectedLocalStorageKey = `selected-id`;
  const savedSelectedId = localStorage.getItem(selectedLocalStorageKey);
  const firstTime = !savedSelectedId;

  /**
   * @returns {PartialPresetTree}
   */
  function createPartialTree() {
    function initConsts() {
      const xth = /** @type {const} */ ([
        {
          id: "sth",
          key: "sth",
          name: "Short Term Holders",
          legend: "STH",
        },
        {
          id: "lth",
          key: "lth",
          name: "Long Term Holders",
          legend: "LTH",
        },
      ]);

      const upTo = /** @type {const} */ ([
        { id: "up-to-1d", key: "up_to_1d", name: "Up To 1 Day", legend: "1D" },
        { id: "up-to-1w", key: "up_to_1w", name: "Up To 1 Week", legend: "1W" },
        {
          id: "up-to-1m",
          key: "up_to_1m",
          name: "Up To 1 Month",
          legend: "1M",
        },
        {
          id: "up-to-2m",
          key: "up_to_2m",
          name: "Up To 2 Months",
          legend: "2M",
        },
        {
          id: "up-to-3m",
          key: "up_to_3m",
          name: "Up To 3 Months",
          legend: "3M",
        },
        {
          id: "up-to-4m",
          key: "up_to_4m",
          name: "Up To 4 Months",
          legend: "4M",
        },
        {
          id: "up-to-5m",
          key: "up_to_5m",
          name: "Up To 5 Months",
          legend: "5M",
        },
        {
          id: "up-to-6m",
          key: "up_to_6m",
          name: "Up To 6 Months",
          legend: "6M",
        },
        { id: "up-to-1y", key: "up_to_1y", name: "Up To 1 Year", legend: "1Y" },
        {
          id: "up-to-2y",
          key: "up_to_2y",
          name: "Up To 2 Years",
          legend: "2Y",
        },
        {
          id: "up-to-3y",
          key: "up_to_3y",
          name: "Up To 3 Years",
          legend: "3Y",
        },
        {
          id: "up-to-5y",
          key: "up_to_5y",
          name: "Up To 5 Years",
          legend: "5Y",
        },
        {
          id: "up-to-7y",
          key: "up_to_7y",
          name: "Up To 7 Years",
          legend: "7Y",
        },
        {
          id: "up-to-10y",
          key: "up_to_10y",
          name: "Up To 10 Years",
          legend: "10Y",
        },
        {
          id: "up-to-15y",
          key: "up_to_15y",
          name: "Up To 15 Years",
          legend: "15Y",
        },
      ]);

      const fromXToY = /** @type {const} */ ([
        {
          id: "from-1d-to-1w",
          key: "from_1d_to_1w",
          name: "From 1 Day To 1 Week",
          legend: "1D - 1W",
        },
        {
          id: "from-1w-to-1m",
          key: "from_1w_to_1m",
          name: "From 1 Week To 1 Month",
          legend: "1W - 1M",
        },
        {
          id: "from-1m-to-3m",
          key: "from_1m_to_3m",
          name: "From 1 Month To 3 Months",
          legend: "1M - 3M",
        },
        {
          id: "from-3m-to-6m",
          key: "from_3m_to_6m",
          name: "From 3 Months To 6 Months",
          legend: "3M - 6M",
        },
        {
          id: "from-6m-to-1y",
          key: "from_6m_to_1y",
          name: "From 6 Months To 1 Year",
          legend: "6M - 1Y",
        },
        {
          id: "from-1y-to-2y",
          key: "from_1y_to_2y",
          name: "From 1 Year To 2 Years",
          legend: "1Y - 2Y",
        },
        {
          id: "from-2y-to-3y",
          key: "from_2y_to_3y",
          name: "From 2 Years To 3 Years",
          legend: "2Y - 3Y",
        },
        {
          id: "from-3y-to-5y",
          key: "from_3y_to_5y",
          name: "From 3 Years To 5 Years",
          legend: "3Y - 5Y",
        },
        {
          id: "from-5y-to-7y",
          key: "from_5y_to_7y",
          name: "From 5 Years To 7 Years",
          legend: "5Y - 7Y",
        },
        {
          id: "from-7y-to-10y",
          key: "from_7y_to_10y",
          name: "From 7 Years To 10 Years",
          legend: "7Y - 10Y",
        },
        {
          id: "from-10y-to-15y",
          key: "from_10y_to_15y",
          name: "From 10 Years To 15 Years",
          legend: "10Y - 15Y",
        },
      ]);

      const fromX = /** @type {const} */ ([
        {
          id: "from-1y",
          key: "from_1y",
          name: "From 1 Year",
          legend: "1Y+",
        },
        {
          id: "from-2y",
          key: "from_2y",
          name: "From 2 Years",
          legend: "2Y+",
        },
        {
          id: "from-4y",
          key: "from_4y",
          name: "From 4 Years",
          legend: "4Y+",
        },
        {
          id: "from-10y",
          key: "from_10y",
          name: "From 10 Years",
          legend: "10Y+",
        },
        {
          id: "from-15y",
          key: "from_15y",
          name: "From 15 Years",
          legend: "15Y+",
        },
      ]);

      const year = /** @type {const} */ ([
        { id: "year-2009", key: "year_2009", name: "2009" },
        { id: "year-2010", key: "year_2010", name: "2010" },
        { id: "year-2011", key: "year_2011", name: "2011" },
        { id: "year-2012", key: "year_2012", name: "2012" },
        { id: "year-2013", key: "year_2013", name: "2013" },
        { id: "year-2014", key: "year_2014", name: "2014" },
        { id: "year-2015", key: "year_2015", name: "2015" },
        { id: "year-2016", key: "year_2016", name: "2016" },
        { id: "year-2017", key: "year_2017", name: "2017" },
        { id: "year-2018", key: "year_2018", name: "2018" },
        { id: "year-2019", key: "year_2019", name: "2019" },
        { id: "year-2020", key: "year_2020", name: "2020" },
        { id: "year-2021", key: "year_2021", name: "2021" },
        { id: "year-2022", key: "year_2022", name: "2022" },
        { id: "year-2023", key: "year_2023", name: "2023" },
        { id: "year-2024", key: "year_2024", name: "2024" },
      ]);

      const age = /** @type {const} */ ([
        {
          key: "",
          id: "",
          name: "",
        },
        ...xth,
        ...upTo,
        ...fromXToY,
        ...fromX,
        ...year,
      ]);

      const size = /** @type {const} */ ([
        {
          key: "plankton",
          name: "Plankton",
          size: "1 sat to 0.1 BTC",
        },
        {
          key: "shrimp",
          name: "Shrimp",
          size: "0.1 sat to 1 BTC",
        },
        { key: "crab", name: "Crab", size: "1 BTC to 10 BTC" },
        { key: "fish", name: "Fish", size: "10 BTC to 100 BTC" },
        { key: "shark", name: "Shark", size: "100 BTC to 1000 BTC" },
        { key: "whale", name: "Whale", size: "1000 BTC to 10 000 BTC" },
        {
          key: "humpback",
          name: "Humpback",
          size: "10 000 BTC to 100 000 BTC",
        },
        { key: "megalodon", name: "Megalodon", size: "More than 100 000 BTC" },
      ]);

      const type = /** @type {const} */ ([
        { key: "p2pk", name: "P2PK" },
        { key: "p2pkh", name: "P2PKH" },
        { key: "p2sh", name: "P2SH" },
        { key: "p2wpkh", name: "P2WPKH" },
        { key: "p2wsh", name: "P2WSH" },
        { key: "p2tr", name: "P2TR" },
      ]);

      const address = /** @type {const} */ ([...size, ...type]);

      const liquidities = /** @type {const} */ ([
        {
          key: "illiquid",
          id: "illiquid",
          name: "Illiquid",
        },
        { key: "liquid", id: "liquid", name: "Liquid" },
        {
          key: "highly_liquid",
          id: "highly-liquid",
          name: "Highly Liquid",
        },
      ]);

      const averages = /** @type {const} */ ([
        { name: "1 Week", key: "1w", days: 7 },
        { name: "8 Days", key: "8d", days: 8 },
        { name: "13 Days", key: "13d", days: 13 },
        { name: "21 Days", key: "21d", days: 21 },
        { name: "1 Month", key: "1m", days: 30 },
        { name: "34 Days", key: "34d", days: 34 },
        { name: "55 Days", key: "55d", days: 55 },
        { name: "89 Days", key: "89d", days: 89 },
        { name: "144 Days", key: "144d", days: 144 },
        { name: "1 Year", key: "1y", days: 365 },
        { name: "2 Years", key: "2y", days: 2 * 365 },
        { name: "200 Weeks", key: "200w", days: 200 * 7 },
        { name: "4 Years", key: "4y", days: 4 * 365 },
      ]);

      const totalReturns = /** @type {const} */ ([
        { name: "1 Day", key: "1d" },
        { name: "1 Month", key: "1m" },
        { name: "6 Months", key: "6m" },
        { name: "1 Year", key: "1y" },
        { name: "2 Years", key: "2y" },
        { name: "3 Years", key: "3y" },
        { name: "4 Years", key: "4y" },
        { name: "6 Years", key: "6y" },
        { name: "8 Years", key: "8y" },
        { name: "10 Years", key: "10y" },
      ]);

      const compoundReturns = /** @type {const} */ ([
        { name: "4 Years", key: "4y" },
      ]);

      const percentiles = /** @type {const} */ ([
        {
          key: "median_price_paid",
          id: "median-price-paid",
          name: "Median",
          title: "Median Paid",
          value: 50,
        },
        {
          key: "95p_price_paid",
          id: "95p-price-paid",
          name: `95%`,
          title: `95th Percentile Paid`,
          value: 95,
        },
        {
          key: "90p_price_paid",
          id: "90p-price-paid",
          name: `90%`,
          title: `90th Percentile Paid`,
          value: 90,
        },
        {
          key: "85p_price_paid",
          id: "85p-price-paid",
          name: `85%`,
          title: `85th Percentile Paid`,
          value: 85,
        },
        {
          key: "80p_price_paid",
          id: "80p-price-paid",
          name: `80%`,
          title: `80th Percentile Paid`,
          value: 80,
        },
        {
          key: "75p_price_paid",
          id: "75p-price-paid",
          name: `75%`,
          title: `75th Percentile Paid`,
          value: 75,
        },
        {
          key: "70p_price_paid",
          id: "70p-price-paid",
          name: `70%`,
          title: `70th Percentile Paid`,
          value: 70,
        },
        {
          key: "65p_price_paid",
          id: "65p-price-paid",
          name: `65%`,
          title: `65th Percentile Paid`,
          value: 65,
        },
        {
          key: "60p_price_paid",
          id: "60p-price-paid",
          name: `60%`,
          title: `60th Percentile Paid`,
          value: 60,
        },
        {
          key: "55p_price_paid",
          id: "55p-price-paid",
          name: `55%`,
          title: `55th Percentile Paid`,
          value: 55,
        },
        {
          key: "45p_price_paid",
          id: "45p-price-paid",
          name: `45%`,
          title: `45th Percentile Paid`,
          value: 45,
        },
        {
          key: "40p_price_paid",
          id: "40p-price-paid",
          name: `40%`,
          title: `40th Percentile Paid`,
          value: 40,
        },
        {
          key: "35p_price_paid",
          id: "35p-price-paid",
          name: `35%`,
          title: `35th Percentile Paid`,
          value: 35,
        },
        {
          key: "30p_price_paid",
          id: "30p-price-paid",
          name: `30%`,
          title: `30th Percentile Paid`,
          value: 30,
        },
        {
          key: "25p_price_paid",
          id: "25p-price-paid",
          name: `25%`,
          title: `25th Percentile Paid`,
          value: 25,
        },
        {
          key: "20p_price_paid",
          id: "20p-price-paid",
          name: `20%`,
          title: `20th Percentile Paid`,
          value: 20,
        },
        {
          key: "15p_price_paid",
          id: "15p-price-paid",
          name: `15%`,
          title: `15th Percentile Paid`,
          value: 15,
        },
        {
          key: "10p_price_paid",
          id: "10p-price-paid",
          name: `10%`,
          title: `10th Percentile Paid`,
          value: 10,
        },
        {
          key: "05p_price_paid",
          id: "05p-price-paid",
          name: `5%`,
          title: `5th Percentile Paid`,
          value: 5,
        },
      ]);

      return {
        xth,
        upTo,
        fromX,
        fromXToY,
        year,
        age,
        type,
        size,
        address,
        liquidities,
        averages,
        totalReturns,
        compoundReturns,
        percentiles,
      };
    }
    const consts = initConsts();
    /**
     * @typedef {(typeof consts.age)[number]["id"]} AgeCohortId
     * @typedef {Exclude<AgeCohortId, "">} AgeCohortIdSub
     * @typedef {(typeof consts.address)[number]["key"]} AddressCohortId
     * @typedef {(typeof consts.liquidities[number]["id"])} LiquidityId
     * @typedef {`${LiquidityId}-${AddressCohortId}`} AddressCohortIdSplitByLiquidity
     * @typedef {AgeCohortId | AddressCohortId} AnyCohortId
     * @typedef {AnyCohortId | AddressCohortIdSplitByLiquidity | LiquidityId} AnyPossibleCohortId
     * @typedef {'' | `${AgeCohortIdSub | AddressCohortId | AddressCohortIdSplitByLiquidity | LiquidityId}-`} AnyDatasetPrefix
     * @typedef {(typeof consts.averages)[number]["key"]} AverageName
     * @typedef {(typeof consts.totalReturns)[number]["key"]} TotalReturnKey
     * @typedef {(typeof consts.compoundReturns)[number]["key"]} CompoundReturnKey
     * @typedef {(typeof consts.percentiles)[number]["id"]} PercentileId
     */

    /**
     * @param {AnyPossibleCohortId} datasetId
     * @returns {AnyDatasetPrefix}
     */
    function datasetIdToPrefix(datasetId) {
      return datasetId
        ? /** @type {const} */ (`${datasetId}-`)
        : /** @type {const} */ ("");
    }

    /**
     *
     * @param {Object} args
     * @param {Scale} args.scale
     * @param {string} args.title
     * @param {Color} args.color
     * @param {Unit} args.unit
     * @param {AnyDatasetPath} [args.keySum]
     * @param {AnyDatasetPath} [args.keyAverage]
     * @param {AnyDatasetPath} [args.keyMax]
     * @param {AnyDatasetPath} [args.key90p]
     * @param {AnyDatasetPath} [args.key75p]
     * @param {AnyDatasetPath} [args.keyMedian]
     * @param {AnyDatasetPath} [args.key25p]
     * @param {AnyDatasetPath} [args.key10p]
     * @param {AnyDatasetPath} [args.keyMin]
     * @returns {PartialPreset[]}
     */
    function createRecapPresets({
      scale,
      unit,
      title,
      keyAverage,
      color,
      keySum,
      keyMax,
      key90p,
      key75p,
      keyMedian,
      key25p,
      key10p,
      keyMin,
    }) {
      return [
        ...(keySum
          ? [
              {
                scale,
                icon: "➕",
                name: "Daily Sum",
                title: `${title} Daily Sum`,
                description: "",
                unit,
                bottom: [
                  {
                    title: "Sum",
                    color,
                    datasetPath: keySum,
                  },
                ],
              },
            ]
          : []),
        ...(keyAverage
          ? [
              {
                scale,
                icon: "🌊",
                name: "Daily Average",
                title: `${title} Daily Average`,
                description: "",
                unit,
                bottom: [
                  {
                    title: "Average",
                    color,
                    datasetPath: keyAverage,
                  },
                ],
              },
            ]
          : []),
        ...(keyMax ||
        key90p ||
        key75p ||
        keyMedian ||
        key25p ||
        key10p ||
        keyMin
          ? [
              {
                scale,
                icon: "%",
                name: "Daily Percentiles",
                title: `${title} Daily Percentiles`,
                description: "",
                unit,
                bottom: [
                  ...(keyMax
                    ? [
                        {
                          title: "Max",
                          color,
                          datasetPath: keyMax,
                        },
                      ]
                    : []),
                  ...(key90p
                    ? [
                        {
                          title: "90%",
                          color,
                          datasetPath: key90p,
                        },
                      ]
                    : []),
                  ...(key75p
                    ? [
                        {
                          title: "75%",
                          color,
                          datasetPath: key75p,
                        },
                      ]
                    : []),
                  ...(keyMedian
                    ? [
                        {
                          title: "Median",
                          color,
                          datasetPath: keyMedian,
                        },
                      ]
                    : []),
                  ...(key25p
                    ? [
                        {
                          title: "25%",
                          color,
                          datasetPath: key25p,
                        },
                      ]
                    : []),
                  ...(key10p
                    ? [
                        {
                          title: "10%",
                          color,
                          datasetPath: key10p,
                        },
                      ]
                    : []),
                  ...(keyMin
                    ? [
                        {
                          title: "Min",
                          color,
                          datasetPath: keyMin,
                        },
                      ]
                    : []),
                ],
              },
            ]
          : []),
        ...(keyMax
          ? [
              {
                scale,
                icon: "⬆️",
                name: "Daily Max",
                title: `${title} Daily Max`,
                description: "",
                unit,
                bottom: [
                  {
                    title: "Max",
                    color,
                    datasetPath: keyMax,
                  },
                ],
              },
            ]
          : []),
        ...(key90p
          ? [
              {
                scale,
                icon: "9️⃣",
                name: "Daily 90th Percentile",
                title: `${title} Daily 90th Percentile`,
                description: "",
                unit,
                bottom: [
                  {
                    title: "90%",
                    color,
                    datasetPath: key90p,
                  },
                ],
              },
            ]
          : []),
        ...(key75p
          ? [
              {
                scale,
                icon: "7️⃣",
                name: "Daily 75th Percentile",
                title: `${title} Size 75th Percentile`,
                description: "",
                unit,
                bottom: [
                  {
                    title: "75%",
                    color,
                    datasetPath: key75p,
                  },
                ],
              },
            ]
          : []),
        ...(keyMedian
          ? [
              {
                scale,
                icon: "5️⃣",
                name: "Daily Median",
                title: `${title} Daily Median`,
                description: "",
                unit,
                bottom: [
                  {
                    title: "Median",
                    color,
                    datasetPath: keyMedian,
                  },
                ],
              },
            ]
          : []),
        ...(key25p
          ? [
              {
                scale,
                icon: "2️⃣",
                name: "Daily 25th Percentile",
                title: `${title} Daily 25th Percentile`,
                description: "",
                unit,
                bottom: [
                  {
                    title: "25%",
                    color,
                    datasetPath: key25p,
                  },
                ],
              },
            ]
          : []),
        ...(key10p
          ? [
              {
                scale,
                icon: "1️⃣",
                name: "Daily 10th Percentile",
                title: `${title} Daily 10th Percentile`,
                description: "",
                unit,
                bottom: [
                  {
                    title: "10%",
                    color,
                    datasetPath: key10p,
                  },
                ],
              },
            ]
          : []),
        ...(keyMin
          ? [
              {
                scale,
                icon: "⬇️",
                name: "Daily Min",
                title: `${title} Daily Min`,
                description: "",
                unit,
                bottom: [
                  {
                    title: "Min",
                    color,
                    datasetPath: keyMin,
                  },
                ],
              },
            ]
          : []),
      ];
    }

    /**
     * @param {Object} args
     * @param {Scale} args.scale
     * @param {Color} args.color
     * @param {AnyDatasetPath} args.valueDatasetPath
     * @param {AnyDatasetPath} args.ratioDatasetPath
     * @param {string} args.title
     * @returns {PartialPresetFolder}
     */
    function createRatioFolder({
      scale,
      color,
      valueDatasetPath,
      ratioDatasetPath,
      title,
    }) {
      return {
        name: "Ratio",
        tree: [
          {
            scale,
            name: "Basic",
            icon: "➗",
            title: `Market Price To ${title} Ratio`,
            unit: "Ratio",
            description: "",
            top: [
              {
                title: `SMA`,
                color,
                datasetPath: valueDatasetPath,
              },
            ],
            bottom: [
              {
                title: `Ratio`,
                type: "Baseline",
                datasetPath: ratioDatasetPath,
                options: {
                  baseValue: {
                    price: 1,
                  },
                },
              },
              {
                title: `Even`,
                color: colors.white,
                datasetPath: `${scale}-to-1`,
                options: {
                  lineStyle: 3,
                  lastValueVisible: false,
                },
              },
            ],
          },
          {
            scale,
            name: "Averages",
            description: "",
            icon: "〰️",
            unit: "Ratio",
            title: `Market Price To ${title} Ratio Averages`,
            top: [
              {
                title: `SMA`,
                color,
                datasetPath: valueDatasetPath,
              },
            ],
            bottom: [
              {
                title: `1Y`,
                color: colors._1y,
                datasetPath: /** @type {any} */ (`${ratioDatasetPath}-1y-sma`),
              },
              {
                title: `1M`,
                color: colors._1m,
                datasetPath: `${ratioDatasetPath}-1m-sma`,
              },
              {
                title: `1W`,
                color: colors._1w,
                datasetPath: `${ratioDatasetPath}-1w-sma`,
              },
              {
                title: `Raw`,
                color: colors.white,
                datasetPath: ratioDatasetPath,
              },
              {
                title: `Even`,
                color: colors.gray,
                datasetPath: `${scale}-to-1`,
                options: {
                  lineStyle: 3,
                  lastValueVisible: false,
                },
              },
            ],
          },
          {
            scale,
            name: "Momentum Oscillator",
            title: `Market Price To ${title} Ratio 1Y SMA Momentum Oscillator`,
            description: "",
            unit: "Ratio",
            icon: "🔀",
            top: [
              {
                title: `SMA`,
                color,
                datasetPath: valueDatasetPath,
              },
            ],
            bottom: [
              {
                title: `Momentum`,
                type: "Baseline",
                datasetPath: /** @type {any} */ (
                  `${ratioDatasetPath}-1y-sma-momentum-oscillator`
                ),
              },
              {
                title: `Base`,
                color: colors.white,
                datasetPath: `${scale}-to-0`,
                options: {
                  lineStyle: 3,
                  lastValueVisible: false,
                },
              },
            ],
          },
          {
            scale,
            name: "Top Percentiles",
            icon: "✈️",
            title: `Market Price To ${title} Ratio Top Percentiles`,
            description: "",
            unit: "Ratio",
            top: [
              {
                title: `SMA`,
                color,
                datasetPath: valueDatasetPath,
              },
            ],
            bottom: [
              {
                title: `99.9%`,
                color: colors.probability0_1p,
                datasetPath: /** @type {any} */ (`${ratioDatasetPath}-99-9p`),
              },
              {
                title: `99.5%`,
                color: colors.probability0_5p,
                datasetPath: `${ratioDatasetPath}-99-5p`,
              },
              {
                title: `99%`,
                color: colors.probability1p,
                datasetPath: `${ratioDatasetPath}-99p`,
              },
              {
                title: `Raw`,
                color: colors.white,
                datasetPath: ratioDatasetPath,
              },
            ],
          },
          {
            scale,
            name: "Bottom Percentiles",
            icon: "🤿",
            title: `Market Price To ${title} Ratio Bottom Percentiles`,
            description: "",
            unit: "Ratio",
            top: [
              {
                title: `SMA`,
                color,
                datasetPath: valueDatasetPath,
              },
            ],
            bottom: [
              {
                title: `1%`,
                color: colors.probability1p,
                datasetPath: /** @type {any} */ (`${ratioDatasetPath}-1p`),
              },
              {
                title: `0.5%`,
                color: colors.probability0_5p,
                datasetPath: `${ratioDatasetPath}-0-5p`,
              },
              {
                title: `0.1%`,
                color: colors.probability0_1p,
                datasetPath: `${ratioDatasetPath}-0-1p`,
              },
              {
                title: `Raw`,
                color: colors.white,
                datasetPath: ratioDatasetPath,
              },
            ],
          },
          {
            scale,
            name: "Top Probabilities",
            icon: "🚀",
            title: `${title} Top Probabilities`,
            description: "",
            unit: "US Dollars",
            top: [
              {
                title: `0.1%`,
                color: colors.probability0_1p,
                datasetPath: /** @type {any} */ (`${valueDatasetPath}-99-9p`),
              },
              {
                title: `0.5%`,
                color: colors.probability0_5p,
                datasetPath: `${valueDatasetPath}-99-5p`,
              },
              {
                title: `1%`,
                color: colors.probability1p,
                datasetPath: `${valueDatasetPath}-99p`,
              },
            ],
          },
          {
            scale,
            name: "Bottom Probabilities",
            icon: "🚇",
            title: `${title} Bottom Probabilities`,
            description: "",
            unit: "US Dollars",
            top: [
              {
                title: `0.1%`,
                color: colors.probability0_1p,
                datasetPath: `${valueDatasetPath}-0-1p`,
              },
              {
                title: `0.5%`,
                color: colors.probability0_5p,
                datasetPath: `${valueDatasetPath}-0-5p`,
              },
              {
                title: `1%`,
                color: colors.probability1p,
                datasetPath: `${valueDatasetPath}-1p`,
              },
            ],
          },
        ],
      };
    }

    /**
     * @param {Scale} scale
     * @returns {PartialPresetFolder}
     */
    function createMarketPresets(scale) {
      /**
       * @param {Scale} scale
       * @returns {PartialPresetFolder}
       */
      function createAveragesPresets(scale) {
        return {
          name: "Averages",
          tree: [
            {
              scale,
              icon: "~",
              name: "All",
              title: "All Moving Averages",
              description: "",
              unit: "US Dollars",
              top: consts.averages.map((average) => ({
                title: average.key.toUpperCase(),
                color: colors[`_${average.key}`],
                datasetPath: `${scale}-to-price-${average.key}-sma`,
              })),
            },
            ...consts.averages.map(({ name, key }) =>
              createAveragePresetFolder({
                scale,
                color: colors[`_${key}`],
                name,
                title: `${name} Market Price Moving Average`,
                key,
              })
            ),
          ],
        };
      }

      /**
       *
       * @param {Object} args
       * @param {Scale} args.scale
       * @param {Color} args.color
       * @param {string} args.name
       * @param {string} args.title
       * @param {AverageName} args.key
       * @returns {PartialPresetFolder}
       */
      function createAveragePresetFolder({ scale, color, name, title, key }) {
        return {
          name,
          tree: [
            {
              scale,
              name: "Average",
              title,
              description: "",
              unit: "US Dollars",
              icon: "~",
              top: [
                {
                  title: `SMA`,
                  color,
                  datasetPath: `${scale}-to-price-${key}-sma`,
                },
              ],
            },
            createRatioFolder({
              scale,
              color,
              ratioDatasetPath: `${scale}-to-market-price-to-price-${key}-sma-ratio`,
              valueDatasetPath: `${scale}-to-price-${key}-sma`,
              title,
            }),
          ],
        };
      }

      /**
       * @returns {PartialPresetFolder}
       */
      function createReturnsPresets() {
        return {
          name: "Returns",
          tree: [
            {
              name: "Total",
              tree: [
                ...consts.totalReturns.map(({ name, key }) =>
                  createReturnsPreset({
                    scale: "date",
                    name,
                    title: `${name} Total`,
                    key: `${key}-total`,
                  })
                ),
              ],
            },
            {
              name: "Compound",
              tree: [
                ...consts.compoundReturns.map(({ name, key }) =>
                  createReturnsPreset({
                    scale: "date",
                    name,
                    title: `${name} Compound`,
                    key: `${key}-compound`,
                  })
                ),
              ],
            },
          ],
        };
      }

      /**
       * @param {Object} args
       * @param {Scale} args.scale
       * @param {string} args.name
       * @param {string} args.title
       * @param {`${TotalReturnKey}-total` | `${CompoundReturnKey}-compound`} args.key
       * @returns {PartialPreset}
       */
      function createReturnsPreset({ scale, name, title, key }) {
        return {
          scale,
          name,
          description: "",
          icon: "🧾",
          title: `${title} Return`,
          unit: "Percentage",
          bottom: [
            {
              title: `Return`,
              type: "Baseline",
              datasetPath: `date-to-price-${key}-return`,
            },
          ],
        };
      }

      /**
       * @returns {PartialPresetFolder}
       */
      function createIndicatorsPresets() {
        return {
          name: "Indicators",
          tree: [],
        };
      }

      return {
        name: "Market",
        tree: [
          {
            scale,
            icon: "💵",
            name: "Price",
            title: "Market Price",
            description: "",
            unit: "US Dollars",
          },
          {
            scale,
            icon: "♾️",
            name: "Capitalization",
            title: "Market Capitalization",
            description: "",
            unit: "US Dollars",
            bottom: [
              {
                title: "Capitalization",
                datasetPath: `${scale}-to-market-cap`,
                color: colors.bitcoin,
              },
            ],
          },
          createAveragesPresets(scale),
          ...(scale === "date"
            ? [createReturnsPresets(), createIndicatorsPresets()]
            : []),
        ],
      };
    }

    /**
     * @param {Scale} scale
     * @returns {PartialPresetFolder}
     */
    function createBlocksPresets(scale) {
      return {
        name: "Blocks",
        tree: [
          ...(scale === "date"
            ? /** @type {PartialPresetTree} */ ([
                {
                  scale,
                  icon: "🧱",
                  name: "Height",
                  title: "Block Height",
                  description: "",
                  unit: "Height",
                  bottom: [
                    {
                      title: "Height",
                      color: colors.bitcoin,
                      datasetPath: `date-to-last-height`,
                    },
                  ],
                },
                {
                  scale,
                  name: "Mined",
                  tree: [
                    {
                      scale,
                      icon: "D",
                      name: "Daily Sum",
                      title: "Daily Sum Of Blocks Mined",
                      description: "",
                      unit: "Count",
                      bottom: [
                        {
                          title: "Target",
                          color: colors.white,
                          datasetPath: `date-to-blocks-mined-1d-target`,
                          options: {
                            lineStyle: 3,
                          },
                        },
                        {
                          title: "1W Avg.",
                          color: colors.momentumYellow,
                          datasetPath: `date-to-blocks-mined-1w-sma`,
                          defaultActive: false,
                        },
                        {
                          title: "1M Avg.",
                          color: colors.bitcoin,
                          datasetPath: `date-to-blocks-mined-1m-sma`,
                        },
                        {
                          title: "Mined",
                          color: colors.darkBitcoin,
                          datasetPath: `date-to-blocks-mined`,
                        },
                      ],
                    },
                    {
                      scale,
                      icon: "W",
                      name: "Weekly Sum",
                      title: "Weekly Sum Of Blocks Mined",
                      description: "",
                      unit: "Count",
                      bottom: [
                        {
                          title: "Target",
                          color: colors.white,
                          datasetPath: `date-to-blocks-mined-1w-target`,
                          options: {
                            lineStyle: 3,
                          },
                        },
                        {
                          title: "Sum Mined",
                          color: colors.bitcoin,
                          datasetPath: `date-to-blocks-mined-1w-sum`,
                        },
                      ],
                    },
                    {
                      scale,
                      icon: "M",
                      name: "Monthly Sum",
                      title: "Monthly Sum Of Blocks Mined",
                      description: "",
                      unit: "Count",
                      bottom: [
                        {
                          title: "Target",
                          color: colors.white,
                          datasetPath: `date-to-blocks-mined-1m-target`,
                          options: {
                            lineStyle: 3,
                          },
                        },
                        {
                          title: "Sum Mined",
                          color: colors.bitcoin,
                          datasetPath: `date-to-blocks-mined-1m-sum`,
                        },
                      ],
                    },
                    {
                      scale,
                      icon: "Y",
                      name: "Yearly Sum",
                      title: "Yearly Sum Of Blocks Mined",
                      description: "",
                      unit: "Count",
                      bottom: [
                        {
                          title: "Target",
                          color: colors.white,
                          datasetPath: `date-to-blocks-mined-1y-target`,
                          options: {
                            lineStyle: 3,
                          },
                        },
                        {
                          title: "Sum Mined",
                          color: colors.bitcoin,
                          datasetPath: `date-to-blocks-mined-1y-sum`,
                        },
                      ],
                    },
                    {
                      scale,
                      icon: "🧱",
                      name: "Total",
                      title: "Total Blocks Mined",
                      description: "",
                      unit: "Count",
                      bottom: [
                        {
                          title: "Mined",
                          color: colors.bitcoin,
                          datasetPath: `date-to-total-blocks-mined`,
                        },
                      ],
                    },
                  ],
                },
                {
                  scale,
                  name: "Size",
                  tree: createRecapPresets({
                    scale,
                    title: "Block Size",
                    color: colors.darkWhite,
                    unit: "Megabytes",
                    keySum: "date-to-block-size-1d-sum",
                    keyAverage: "date-to-block-size-1d-average",
                    keyMax: "date-to-block-size-1d-max",
                    key90p: "date-to-block-size-1d-90p",
                    key75p: "date-to-block-size-1d-75p",
                    keyMedian: "date-to-block-size-1d-median",
                    key25p: "date-to-block-size-1d-25p",
                    key10p: "date-to-block-size-1d-10p",
                    keyMin: "date-to-block-size-1d-min",
                  }),
                },
                {
                  scale,
                  name: "Weight",
                  tree: createRecapPresets({
                    scale,
                    title: "Block Weight",
                    color: colors.darkWhite,
                    unit: "Weight",
                    keyAverage: "date-to-block-weight-1d-average",
                    keyMax: "date-to-block-weight-1d-max",
                    key90p: "date-to-block-weight-1d-90p",
                    key75p: "date-to-block-weight-1d-75p",
                    keyMedian: "date-to-block-weight-1d-median",
                    key25p: "date-to-block-weight-1d-25p",
                    key10p: "date-to-block-weight-1d-10p",
                    keyMin: "date-to-block-weight-1d-min",
                  }),
                },
                {
                  scale,
                  name: "VBytes",
                  tree: createRecapPresets({
                    scale,
                    title: "Block VBytes",
                    color: colors.darkWhite,
                    unit: "Virtual Bytes",
                    keyAverage: "date-to-block-vbytes-1d-average",
                    keyMax: "date-to-block-vbytes-1d-max",
                    key90p: "date-to-block-vbytes-1d-90p",
                    key75p: "date-to-block-vbytes-1d-75p",
                    keyMedian: "date-to-block-vbytes-1d-median",
                    key25p: "date-to-block-vbytes-1d-25p",
                    key10p: "date-to-block-vbytes-1d-10p",
                    keyMin: "date-to-block-vbytes-1d-min",
                  }),
                },
                {
                  scale,
                  name: "Interval",
                  tree: createRecapPresets({
                    scale,
                    title: "Block Interval",
                    color: colors.darkWhite,
                    unit: "Seconds",
                    keyAverage: "date-to-block-interval-1d-average",
                    keyMax: "date-to-block-interval-1d-max",
                    key90p: "date-to-block-interval-1d-90p",
                    key75p: "date-to-block-interval-1d-75p",
                    keyMedian: "date-to-block-interval-1d-median",
                    key25p: "date-to-block-interval-1d-25p",
                    key10p: "date-to-block-interval-1d-10p",
                    keyMin: "date-to-block-interval-1d-min",
                  }),
                },
              ])
            : /** @type {PartialPresetTree} */ ([
                {
                  scale,
                  icon: "📏",
                  name: "Size",
                  title: "Block Size",
                  description: "",
                  unit: "Megabytes",
                  bottom: [
                    {
                      title: "Size",
                      color: colors.darkWhite,
                      datasetPath: `height-to-block-size`,
                    },
                  ],
                },
                {
                  scale,
                  icon: "🏋️",
                  name: "Weight",
                  title: "Block Weight",
                  description: "",
                  unit: "Weight",
                  bottom: [
                    {
                      title: "Weight",
                      color: colors.darkWhite,
                      datasetPath: `height-to-block-weight`,
                    },
                  ],
                },
                {
                  scale,
                  icon: "👾",
                  name: "VBytes",
                  title: "Block VBytes",
                  description: "",
                  unit: "Virtual Bytes",
                  bottom: [
                    {
                      title: "VBytes",
                      color: colors.darkWhite,
                      datasetPath: `height-to-block-vbytes`,
                    },
                  ],
                },
                {
                  scale,
                  icon: "⏰",
                  name: "Interval",
                  title: "Block Interval",
                  description: "",
                  unit: "Seconds",
                  bottom: [
                    {
                      title: "Interval",
                      color: colors.darkWhite,
                      datasetPath: `height-to-block-interval`,
                    },
                  ],
                },
              ])),
          {
            scale,
            icon: "📏",
            name: "Cumulative Size",
            title: "Cumulative Block Size",
            description: "",
            unit: "Megabytes",
            bottom: [
              {
                title: "Size",
                color: colors.darkWhite,
                datasetPath: `${scale}-to-cumulative-block-size`,
              },
            ],
          },
        ],
      };
    }

    /**
     * @param {Scale} scale
     * @returns {PartialPresetFolder}
     */
    function createMinersPresets(scale) {
      return {
        name: "Miners",
        tree: [
          {
            name: "Coinbases",
            tree: [
              ...(scale === "date"
                ? /** @type {PartialPresetTree} */ ([
                    {
                      name: "Last",
                      tree: [
                        {
                          scale,
                          icon: "🍊",
                          name: "In Bitcoin",
                          title: "Last Coinbase In Bitcoin",
                          description: "",
                          unit: "US Dollars",
                          bottom: [
                            {
                              title: "Last",
                              color: colors.bitcoin,
                              datasetPath: `${scale}-to-last-coinbase`,
                            },
                          ],
                        },
                        {
                          scale,
                          icon: "💵",
                          name: "In Dollars",
                          title: "Last Coinbase In Dollars",
                          description: "",
                          unit: "US Dollars",
                          bottom: [
                            {
                              title: "Last",
                              color: colors.dollars,
                              datasetPath: `${scale}-to-last-coinbase-in-dollars`,
                            },
                          ],
                        },
                      ],
                    },
                    {
                      scale,
                      name: "Daily Sum",
                      tree: [
                        {
                          scale,
                          icon: "🍊",
                          name: "In Bitcoin",
                          title: "Daily Sum Of Coinbases In Bitcoin",
                          description: "",
                          unit: "Bitcoin",
                          bottom: [
                            {
                              title: "Sum",
                              color: colors.bitcoin,
                              datasetPath: `${scale}-to-coinbase`,
                            },
                          ],
                        },
                        {
                          scale,
                          icon: "💵",
                          name: "In Dollars",
                          title: "Daily Sum Of Coinbases In Dollars",
                          description: "",
                          unit: "US Dollars",
                          bottom: [
                            {
                              title: "Sum",
                              color: colors.dollars,
                              datasetPath: `${scale}-to-coinbase-in-dollars`,
                            },
                          ],
                        },
                      ],
                    },
                    {
                      scale,
                      name: "Yearly Sum",
                      tree: [
                        {
                          scale,
                          icon: "🍊",
                          name: "In Bitcoin",
                          title: "Yearly Sum Of Coinbases In Bitcoin",
                          description: "",
                          unit: "Bitcoin",
                          bottom: [
                            {
                              title: "Sum",
                              color: colors.bitcoin,
                              datasetPath: `${scale}-to-coinbase-1y-sum`,
                            },
                          ],
                        },
                        {
                          scale,
                          icon: "💵",
                          name: "In Dollars",
                          title: "Yearly Sum Of Coinbases In Dollars",
                          description: "",
                          unit: "US Dollars",
                          bottom: [
                            {
                              title: "Sum",
                              color: colors.dollars,
                              datasetPath: `${scale}-to-coinbase-in-dollars-1y-sum`,
                            },
                          ],
                        },
                      ],
                    },
                    {
                      scale,
                      name: "Cumulative",
                      tree: [
                        {
                          scale,
                          icon: "🍊",
                          name: "In Bitcoin",
                          title: "Cumulative Coinbases In Bitcoin",
                          description: "",
                          unit: "Bitcoin",
                          bottom: [
                            {
                              title: "Coinbases",
                              color: colors.bitcoin,
                              datasetPath: `${scale}-to-cumulative-coinbase`,
                            },
                          ],
                        },
                        {
                          scale,
                          icon: "💵",
                          name: "In Dollars",
                          title: "Cumulative Coinbases In Dollars",
                          description: "",
                          unit: "US Dollars",
                          bottom: [
                            {
                              title: "Coinbases",
                              color: colors.dollars,
                              datasetPath: `${scale}-to-cumulative-coinbase-in-dollars`,
                            },
                          ],
                        },
                      ],
                    },
                  ])
                : []),
            ],
          },

          {
            name: "Subsidies",
            tree: [
              ...(scale === "date"
                ? /** @type {PartialPresetTree} */ ([
                    {
                      name: "Last",
                      tree: [
                        {
                          scale,
                          icon: "🍊",
                          name: "In Bitcoin",
                          title: "Last Subsidy In Bitcoin",
                          description: "",
                          unit: "Bitcoin",
                          bottom: [
                            {
                              title: "Last",
                              color: colors.bitcoin,
                              datasetPath: `${scale}-to-last-subsidy`,
                            },
                          ],
                        },
                        {
                          scale,
                          icon: "💵",
                          name: "In Dollars",
                          title: "Last Subsidy In Dollars",
                          description: "",
                          unit: "US Dollars",
                          bottom: [
                            {
                              title: "Last",
                              color: colors.dollars,
                              datasetPath: `${scale}-to-last-subsidy-in-dollars`,
                            },
                          ],
                        },
                      ],
                    },
                    {
                      scale,
                      name: "Daily Sum",
                      tree: [
                        {
                          scale,
                          icon: "🍊",
                          name: "In Bitcoin",
                          title: "Daily Sum Of Subsidies In Bitcoin",
                          description: "",
                          unit: "Bitcoin",
                          bottom: [
                            {
                              title: "Sum",
                              color: colors.bitcoin,
                              datasetPath: `${scale}-to-subsidy`,
                            },
                          ],
                        },
                        {
                          scale,
                          icon: "💵",
                          name: "In Dollars",
                          title: "Daily Sum Of Subsidies In Dollars",
                          description: "",
                          unit: "US Dollars",
                          bottom: [
                            {
                              title: "Sum",
                              color: colors.dollars,
                              datasetPath: `${scale}-to-subsidy-in-dollars`,
                            },
                          ],
                        },
                      ],
                    },
                    {
                      scale,
                      name: "Yearly Sum",
                      tree: [
                        {
                          scale,
                          icon: "🍊",
                          name: "In Bitcoin",
                          title: "Yearly Sum Of Subsidies In Bitcoin",
                          description: "",
                          unit: "Bitcoin",
                          bottom: [
                            {
                              title: "Sum",
                              color: colors.bitcoin,
                              datasetPath: `${scale}-to-subsidy-1y-sum`,
                            },
                          ],
                        },
                        {
                          scale,
                          icon: "💵",
                          name: "In Dollars",
                          title: "Yearly Sum Of Subsidies In Dollars",
                          description: "",
                          unit: "US Dollars",
                          bottom: [
                            {
                              title: "Sum",
                              color: colors.dollars,
                              datasetPath: `${scale}-to-subsidy-in-dollars-1y-sum`,
                            },
                          ],
                        },
                      ],
                    },
                    {
                      scale,
                      name: "Cumulative",
                      tree: [
                        {
                          scale,
                          icon: "🍊",
                          name: "In Bitcoin",
                          title: "Cumulative Subsidies In Bitcoin",
                          description: "",
                          unit: "Bitcoin",
                          bottom: [
                            {
                              title: "Subsidies",
                              color: colors.bitcoin,
                              datasetPath: `${scale}-to-cumulative-subsidy`,
                            },
                          ],
                        },
                        {
                          scale,
                          icon: "💵",
                          name: "In Dollars",
                          title: "Cumulative Subsidies In Dollars",
                          description: "",
                          unit: "US Dollars",
                          bottom: [
                            {
                              title: "Subsidies",
                              color: colors.dollars,
                              datasetPath: `${scale}-to-cumulative-subsidy-in-dollars`,
                            },
                          ],
                        },
                      ],
                    },
                  ])
                : []),
            ],
          },

          {
            name: "Fees",
            tree: [
              ...(scale === "date"
                ? /** @type {PartialPresetTree} */ ([
                    {
                      name: "Last",
                      tree: [
                        {
                          scale,
                          icon: "🍊",
                          name: "In Bitcoin",
                          title: "Last Fees In Bitcoin",
                          description: "",
                          unit: "Bitcoin",
                          bottom: [
                            {
                              title: "Last",
                              color: colors.bitcoin,
                              datasetPath: `${scale}-to-last-fees`,
                            },
                          ],
                        },
                        {
                          scale,
                          icon: "💵",
                          name: "In Dollars",
                          title: "Last Fees In Dollars",
                          description: "",
                          unit: "US Dollars",
                          bottom: [
                            {
                              title: "Last",
                              color: colors.dollars,
                              datasetPath: `${scale}-to-last-fees-in-dollars`,
                            },
                          ],
                        },
                      ],
                    },
                    {
                      scale,
                      name: "Daily Sum",
                      tree: [
                        {
                          scale,
                          icon: "🍊",
                          name: "In Bitcoin",
                          title: "Daily Sum Of Fees In Bitcoin",
                          description: "",
                          unit: "Bitcoin",
                          bottom: [
                            {
                              title: "Sum",
                              color: colors.bitcoin,
                              datasetPath: `${scale}-to-fees`,
                            },
                          ],
                        },
                        {
                          scale,
                          icon: "💵",
                          name: "In Dollars",
                          title: "Daily Sum Of Fees In Dollars",
                          description: "",
                          unit: "US Dollars",
                          bottom: [
                            {
                              title: "Sum",
                              color: colors.dollars,
                              datasetPath: `${scale}-to-fees-in-dollars`,
                            },
                          ],
                        },
                      ],
                    },
                    {
                      scale,
                      name: "Yearly Sum",
                      tree: [
                        {
                          scale,
                          icon: "🍊",
                          name: "In Bitcoin",
                          title: "Yearly Sum Of Fees In Bitcoin",
                          description: "",
                          unit: "Bitcoin",
                          bottom: [
                            {
                              title: "Sum",
                              color: colors.bitcoin,
                              datasetPath: `${scale}-to-fees-1y-sum`,
                            },
                          ],
                        },
                        {
                          scale,
                          icon: "💵",
                          name: "In Dollars",
                          title: "Yearly Sum Of Fees In Dollars",
                          description: "",
                          unit: "US Dollars",
                          bottom: [
                            {
                              title: "Sum",
                              color: colors.dollars,
                              datasetPath: `${scale}-to-fees-in-dollars-1y-sum`,
                            },
                          ],
                        },
                      ],
                    },
                    {
                      scale,
                      name: "Cumulative",
                      tree: [
                        {
                          scale,
                          icon: "🍊",
                          name: "In Bitcoin",
                          title: "Cumulative Fees In Bitcoin",
                          description: "",
                          unit: "Bitcoin",
                          bottom: [
                            {
                              title: "Fees",
                              color: colors.bitcoin,
                              datasetPath: `${scale}-to-cumulative-fees`,
                            },
                          ],
                        },
                        {
                          scale,
                          icon: "💵",
                          name: "In Dollars",
                          title: "Cumulative Fees In Dollars",
                          description: "",
                          unit: "US Dollars",
                          bottom: [
                            {
                              title: "Fees",
                              color: colors.dollars,
                              datasetPath: `${scale}-to-cumulative-fees-in-dollars`,
                            },
                          ],
                        },
                      ],
                    },
                  ])
                : []),
            ],
          },

          {
            scale,
            icon: "⚔️",
            name: "Subsidy V. Fees",
            title: "Subsidy V. Fees",
            description: "",
            unit: "Percentage",
            bottom: [
              {
                title: "Subsidy",
                color: colors.bitcoin,
                datasetPath: `${scale}-to-subsidy-to-coinbase-ratio`,
              },
              {
                title: "Fees",
                color: colors.darkBitcoin,
                datasetPath: `${scale}-to-fees-to-coinbase-ratio`,
              },
            ],
          },

          ...(scale === "date"
            ? /** @type {PartialPresetTree} */ ([
                {
                  scale,
                  icon: "🧮",
                  name: "Puell Multiple",
                  title: "Puell Multiple",
                  description: "",
                  unit: "",
                  bottom: [
                    {
                      title: "Multiple",
                      color: colors.bitcoin,
                      datasetPath: `date-to-puell-multiple`,
                    },
                  ],
                },

                {
                  scale,
                  icon: "⛏️",
                  name: "Hash Rate",
                  title: "Hash Rate",
                  description: "",
                  unit: "ExaHash / Second",
                  bottom: [
                    {
                      title: "1M SMA",
                      color: colors.momentumYellow,
                      datasetPath: `date-to-hash-rate-1m-sma`,
                    },
                    {
                      title: "1W SMA",
                      color: colors.bitcoin,
                      datasetPath: `date-to-hash-rate-1w-sma`,
                    },
                    {
                      title: "Rate",
                      color: colors.darkBitcoin,
                      datasetPath: `date-to-hash-rate`,
                    },
                  ],
                },
                {
                  scale,
                  icon: "🎗️",
                  name: "Hash Ribbon",
                  title: "Hash Ribbon",
                  description: "",
                  unit: "ExaHash / Second",
                  bottom: [
                    {
                      title: "1M SMA",
                      color: colors.profit,
                      datasetPath: `date-to-hash-rate-1m-sma`,
                    },
                    {
                      title: "2M SMA",
                      color: colors.loss,
                      datasetPath: `date-to-hash-rate-2m-sma`,
                    },
                  ],
                },
                {
                  scale,
                  icon: "🏷️",
                  name: "Hash Price",
                  title: "Hash Price",
                  description: "",
                  unit: "Dollars / (PetaHash / Second)",
                  bottom: [
                    {
                      title: "Price",
                      color: colors.dollars,
                      datasetPath: `date-to-hash-price`,
                    },
                  ],
                },
              ])
            : []),

          {
            scale,
            icon: "🏋️",
            name: "Difficulty",
            title: "Difficulty",
            description: "",
            unit: "",
            bottom: [
              {
                title: "Difficulty",
                color: colors.bitcoin,
                datasetPath: `${scale}-to-difficulty`,
              },
            ],
          },

          ...(scale === "date"
            ? /** @type {PartialPresetTree} */ ([
                {
                  scale,
                  icon: "📊",
                  name: "Difficulty Adjustment",
                  title: "Difficulty Adjustment",
                  description: "",
                  unit: "Percentage",
                  bottom: [
                    {
                      title: "Adjustment",
                      type: "Baseline",
                      datasetPath: `${scale}-to-difficulty-adjustment`,
                    },
                  ],
                },
              ])
            : []),

          {
            scale,
            icon: "🏭",
            name: "Annualized Issuance",
            title: "Annualized Issuance",
            description: "",
            unit: "Bitcoin",
            bottom: [
              {
                title: "Issuance",
                color: colors.bitcoin,
                datasetPath: `${scale}-to-annualized-issuance`,
              },
            ],
          },

          {
            scale,
            icon: "🏗️",
            name: "Yearly Inflation Rate",
            title: "Yearly Inflation Rate",
            description: "",
            unit: "Percentage",
            bottom: [
              {
                title: "Rate",
                color: colors.bitcoin,
                datasetPath: `${scale}-to-yearly-inflation-rate`,
              },
            ],
          },

          // For scale === "height"
          // block_size,
          // block_weight,
          // block_vbytes,
          // block_interval,
        ],
      };
    }

    /**
     * @param {Scale} scale
     * @returns {PartialPresetFolder}
     */
    function createTransactionsPresets(scale) {
      return {
        name: "Transactions",
        tree: [
          {
            scale,
            icon: "🖐️",
            name: "Count",
            title: "Transaction Count",
            description: "",
            unit: "Count",
            bottom: [
              {
                title: "1M SMA",
                color: colors.momentumYellow,
                datasetPath: `${scale}-to-transaction-count-1m-sma`,
              },
              {
                title: "1W SMA",
                color: colors.bitcoin,
                datasetPath: `${scale}-to-transaction-count-1w-sma`,
              },
              {
                title: "Raw",
                color: colors.darkBitcoin,
                datasetPath: `${scale}-to-transaction-count`,
              },
            ],
          },

          {
            name: "Volume",
            tree: [
              {
                scale,
                icon: "🍊",
                name: "In Bitcoin",
                title: "Transaction Volume",
                description: "",
                unit: "Bitcoin",
                bottom: [
                  {
                    title: "1M SMA",
                    color: colors.momentumYellow,
                    datasetPath: `${scale}-to-transaction-volume-1m-sma`,
                  },
                  {
                    title: "1W SMA",
                    color: colors.bitcoin,
                    datasetPath: `${scale}-to-transaction-volume-1w-sma`,
                  },
                  {
                    title: "Raw",
                    color: colors.darkBitcoin,
                    datasetPath: `${scale}-to-transaction-volume`,
                  },
                ],
              },
              {
                scale,
                icon: "💵",
                name: "In Dollars",
                title: "Transaction Volume In Dollars",
                description: "",
                unit: "US Dollars",
                bottom: [
                  {
                    title: "1M SMA",
                    color: colors.lightDollars,
                    datasetPath: `${scale}-to-transaction-volume-in-dollars-1m-sma`,
                  },
                  {
                    title: "1W SMA",
                    color: colors.dollars,
                    datasetPath: `${scale}-to-transaction-volume-in-dollars-1w-sma`,
                  },
                  {
                    title: "Raw",
                    color: colors.darkDollars,
                    datasetPath: `${scale}-to-transaction-volume-in-dollars`,
                  },
                ],
              },
            ],
          },

          {
            name: "Annualized Volume",
            tree: [
              {
                scale,
                icon: "🍊",
                name: "In Bitcoin",
                title: "Annualized Transaction Volume",
                description: "",
                unit: "Bitcoin",
                bottom: [
                  {
                    title: "Volume",
                    color: colors.bitcoin,
                    datasetPath: `${scale}-to-annualized-transaction-volume`,
                  },
                ],
              },
              {
                scale,
                icon: "💵",
                name: "In Dollars",
                title: "Annualized Transaction Volume In Dollars",
                description: "",
                unit: "US Dollars",
                bottom: [
                  {
                    title: "Volume",
                    color: colors.dollars,
                    datasetPath: `${scale}-to-annualized-transaction-volume-in-dollars`,
                  },
                ],
              },
            ],
          },
          {
            scale,
            icon: "💨",
            name: "Velocity",
            title: "Transactions Velocity",
            description: "",
            unit: "",
            bottom: [
              {
                title: "Transactions Velocity",
                color: colors.bitcoin,
                datasetPath: `${scale}-to-transaction-velocity`,
              },
            ],
          },
          {
            scale,
            icon: "⏰",
            name: "Per Second",
            title: "Transactions Per Second",
            description: "",
            unit: "Transactions",
            bottom: [
              {
                title: "1M SMA",
                color: colors.lightBitcoin,
                datasetPath: `${scale}-to-transactions-per-second-1m-sma`,
              },
              {
                title: "1W SMA",
                color: colors.bitcoin,
                datasetPath: `${scale}-to-transactions-per-second-1w-sma`,
              },
              {
                title: "Raw",
                color: colors.darkBitcoin,
                datasetPath: `${scale}-to-transactions-per-second`,
              },
            ],
          },
        ],
      };
    }

    /**
     * @param {Object} args
     * @param {Scale} args.scale
     * @param {AnyPossibleCohortId} args.datasetId
     * @param {string} args.title
     * @param {Color} args.color
     * @returns {PartialPresetFolder}
     */
    function createCohortPresetUTXOFolder({ scale, color, datasetId, title }) {
      const datasetPrefix = datasetIdToPrefix(datasetId);

      return {
        name: "UTXOs",
        tree: [
          {
            scale,
            name: `Count`,
            title: `${title} Unspent Transaction Outputs Count`,
            description: "",
            unit: "Count",
            icon: "🎫",
            bottom: [
              {
                title: "Count",
                color,
                datasetPath: `${scale}-to-${datasetPrefix}utxo-count`,
              },
            ],
          },
        ],
      };
    }

    /**
     * @param {Object} args
     * @param {Scale} args.scale
     * @param {AnyPossibleCohortId} args.datasetId
     * @param {string} args.title
     * @param {Color} args.color
     * @returns {PartialPresetFolder}
     */
    function createCohortPresetRealizedFolder({
      scale,
      color,
      datasetId,
      title,
    }) {
      const datasetPrefix = datasetIdToPrefix(datasetId);

      return {
        name: "Realized",
        tree: [
          {
            scale,
            name: `Price`,
            title: `${title} Realized Price`,
            description: "",
            unit: "US Dollars",
            icon: "🏷️",
            top: [
              {
                title: "Realized Price",
                color,
                datasetPath: `${scale}-to-${datasetPrefix}realized-price`,
              },
            ],
          },
          createRatioFolder({
            scale,
            color,
            ratioDatasetPath: `${scale}-to-market-price-to-${datasetPrefix}realized-price-ratio`,
            valueDatasetPath: `${scale}-to-${datasetPrefix}realized-price`,
            title: `${title} Realized Price`,
          }),
          {
            scale,
            name: `Capitalization`,
            title: `${title} Realized Capitalization`,
            description: "",
            unit: "US Dollars",
            icon: "💰",
            bottom: [
              {
                title: `${name} Realized Cap.`,
                color,
                datasetPath: `${scale}-to-${datasetPrefix}realized-cap`,
              },
              ...(datasetId
                ? /** @type {const} */ ([
                    {
                      title: "Realized Cap.",
                      color: colors.bitcoin,
                      datasetPath: `${scale}-to-realized-cap`,
                      defaultActive: false,
                    },
                  ])
                : []),
            ],
          },
          {
            scale,
            name: `Capitalization 1M Net Change`,
            title: `${title} Realized Capitalization 1 Month Net Change`,
            description: "",
            unit: "US Dollars",
            icon: "🔀",
            bottom: [
              {
                title: `Net Change`,
                type: "Baseline",
                datasetPath: `${scale}-to-${datasetPrefix}realized-cap-1m-net-change`,
              },
            ],
          },
          {
            scale,
            name: `Profit`,
            title: `${title} Realized Profit`,
            description: "",
            unit: "US Dollars",
            icon: "🎉",
            bottom: [
              {
                title: "Realized Profit",
                datasetPath: `${scale}-to-${datasetPrefix}realized-profit`,
                color: colors.profit,
              },
            ],
          },
          {
            scale,
            name: "Loss",
            title: `${title} Realized Loss`,
            description: "",
            unit: "US Dollars",
            icon: "⚰️",
            bottom: [
              {
                title: "Realized Loss",
                datasetPath: `${scale}-to-${datasetPrefix}realized-loss`,
                color: colors.loss,
              },
            ],
          },
          {
            scale,
            name: `PNL`,
            title: `${title} Realized Profit And Loss`,
            description: "",
            unit: "US Dollars",
            icon: "⚖️",
            bottom: [
              {
                title: "Profit",
                color: colors.profit,
                datasetPath: `${scale}-to-${datasetPrefix}realized-profit`,
                type: "Baseline",
              },
              {
                title: "Loss",
                color: colors.loss,
                datasetPath: `${scale}-to-${datasetPrefix}negative-realized-loss`,
                type: "Baseline",
              },
            ],
          },
          {
            scale,
            name: `Net PNL`,
            title: `${title} Net Realized Profit And Loss`,
            description: "",
            unit: "US Dollars",
            icon: "⚖️",
            bottom: [
              {
                title: "Net PNL",
                type: "Baseline",
                datasetPath: `${scale}-to-${datasetPrefix}net-realized-profit-and-loss`,
              },
            ],
          },
          {
            scale,
            name: `Net PNL Relative To Market Cap`,
            title: `${title} Net Realized Profit And Loss Relative To Market Capitalization`,
            description: "",
            unit: "Percentage",
            icon: "➗",
            bottom: [
              {
                title: "Net",
                type: "Baseline",
                datasetPath: `${scale}-to-${datasetPrefix}net-realized-profit-and-loss-to-market-cap-ratio`,
              },
            ],
          },
          {
            scale,
            name: `Cumulative Profit`,
            title: `${title} Cumulative Realized Profit`,
            description: "",
            unit: "US Dollars",
            icon: "🎊",
            bottom: [
              {
                title: "Cumulative Realized Profit",
                color: colors.profit,
                datasetPath: `${scale}-to-${datasetPrefix}cumulative-realized-profit`,
              },
            ],
          },
          {
            scale,
            name: "Cumulative Loss",
            title: `${title} Cumulative Realized Loss`,
            description: "",
            unit: "US Dollars",
            icon: "☠️",
            bottom: [
              {
                title: "Cumulative Realized Loss",
                color: colors.loss,
                datasetPath: `${scale}-to-${datasetPrefix}cumulative-realized-loss`,
              },
            ],
          },
          {
            scale,
            name: `Cumulative Net PNL`,
            title: `${title} Cumulative Net Realized Profit And Loss`,
            description: "",
            unit: "US Dollars",
            icon: "➕",
            bottom: [
              {
                title: "Cumulative Net Realized PNL",
                type: "Baseline",
                datasetPath: `${scale}-to-${datasetPrefix}cumulative-net-realized-profit-and-loss`,
              },
            ],
          },
          {
            scale,
            name: `Cumulative Net PNL 30 Day Change`,
            title: `${title} Cumulative Net Realized Profit And Loss 30 Day Change`,
            description: "",
            unit: "US Dollars",
            icon: "🗓️",
            bottom: [
              {
                title: "Cumulative Net Realized PNL 30d Change",
                datasetPath: `${scale}-to-${datasetPrefix}cumulative-net-realized-profit-and-loss-1m-net-change`,
                type: "Baseline",
              },
            ],
          },
          {
            scale,
            name: `Value Created`,
            title: `${title} Value Created`,
            description: "",
            unit: "US Dollars",
            icon: "➕",
            bottom: [
              {
                title: "Value",
                color: colors.profit,
                datasetPath: `${scale}-to-${datasetPrefix}value-created`,
              },
            ],
          },
          {
            scale,
            name: `Value Destroyed`,
            title: `${title} Value Destroyed`,
            description: "",
            unit: "US Dollars",
            icon: "☄️",
            bottom: [
              {
                title: "Value",
                color: colors.loss,
                datasetPath: `${scale}-to-${datasetPrefix}value-destroyed`,
              },
            ],
          },
          {
            scale,
            name: `Spent Output Profit Ratio - SOPR`,
            title: `${title} Spent Output Profit Ratio`,
            description: "",
            unit: "Percentage",
            icon: "➗",
            bottom: [
              {
                title: "SOPR",
                datasetPath: `${scale}-to-${datasetPrefix}spent-output-profit-ratio`,
                type: "Baseline",
                options: {
                  baseValue: {
                    price: 1,
                  },
                },
              },
            ],
          },
        ],
      };
    }

    /**
     * @param {Object} args
     * @param {Scale} args.scale
     * @param {AnyPossibleCohortId} args.datasetId
     * @param {string} args.title
     * @param {Color} args.color
     * @returns {PartialPresetFolder}
     */
    function createCohortPresetUnrealizedFolder({
      scale,
      color,
      datasetId,
      title,
    }) {
      const datasetPrefix = datasetIdToPrefix(datasetId);

      return {
        name: "Unrealized",
        tree: [
          {
            scale,
            name: `Profit`,
            title: `${title} Unrealized Profit`,
            description: "",
            unit: "US Dollars",
            icon: "🤑",
            bottom: [
              {
                title: "Profit",
                datasetPath: `${scale}-to-${datasetPrefix}unrealized-profit`,
                color: colors.profit,
              },
            ],
          },
          {
            scale,
            name: "Loss",
            title: `${title} Unrealized Loss`,
            description: "",
            unit: "US Dollars",
            icon: "😭",
            bottom: [
              {
                title: "Loss",
                datasetPath: `${scale}-to-${datasetPrefix}unrealized-loss`,
                color: colors.loss,
              },
            ],
          },
          {
            scale,
            name: `PNL`,
            title: `${title} Unrealized Profit And Loss`,
            description: "",
            unit: "US Dollars",
            icon: "🤔",
            bottom: [
              {
                title: "Profit",
                color: colors.profit,
                datasetPath: `${scale}-to-${datasetPrefix}unrealized-profit`,
                type: "Baseline",
              },
              {
                title: "Loss",
                color: colors.loss,
                datasetPath: `${scale}-to-${datasetPrefix}negative-unrealized-loss`,
                type: "Baseline",
              },
            ],
          },
          {
            scale,
            name: `Net PNL`,
            title: `${title} Net Unrealized Profit And Loss`,
            description: "",
            unit: "US Dollars",
            icon: "⚖️",
            bottom: [
              {
                title: "Net Unrealized PNL",
                datasetPath: `${scale}-to-${datasetPrefix}net-unrealized-profit-and-loss`,
                type: "Baseline",
              },
            ],
          },
          {
            scale,
            name: `Net PNL Relative To Market Cap`,
            title: `${title} Net Unrealized Profit And Loss Relative To Total Market Capitalization`,
            description: "",
            unit: "Percentage",
            icon: "➗",
            bottom: [
              {
                title: "Relative Net Unrealized PNL",
                datasetPath: `${scale}-to-${datasetPrefix}net-unrealized-profit-and-loss-to-market-cap-ratio`,
                type: "Baseline",
              },
            ],
          },
        ],
      };
    }

    /**
     * @param {Object} args
     * @param {Scale} args.scale
     * @param {AnyPossibleCohortId} args.datasetId
     * @param {string} args.title
     * @param {Color} args.color
     * @returns {PartialPresetFolder}
     */
    function createCohortPresetSupplyFolder({
      scale,
      color,
      datasetId,
      title,
    }) {
      const datasetPrefix = datasetIdToPrefix(datasetId);

      return {
        name: "Supply",
        tree: [
          {
            name: "Absolute",
            tree: [
              {
                scale,
                name: "All",
                title: `${title} Profit And Loss`,
                icon: "❌",
                description: "",
                unit: "US Dollars",
                bottom: [
                  {
                    title: "In Profit",
                    color: colors.profit,
                    datasetPath: `${scale}-to-${datasetPrefix}supply-in-profit`,
                  },
                  {
                    title: "In Loss",
                    color: colors.loss,
                    datasetPath: `${scale}-to-${datasetPrefix}supply-in-loss`,
                  },
                  {
                    title: "Total",
                    color: colors.white,
                    datasetPath: `${scale}-to-${datasetPrefix}supply`,
                  },
                  {
                    title: "Halved Total",
                    color: colors.gray,
                    datasetPath: `${scale}-to-${datasetPrefix}halved-supply`,
                    options: {
                      lineStyle: 4,
                    },
                  },
                ],
              },
              {
                scale,
                name: `Total`,
                title: `${title} Total supply`,
                icon: "∑",
                description: "",
                unit: "Bitcoin",
                bottom: [
                  {
                    title: "Supply",
                    color,
                    datasetPath: `${scale}-to-${datasetPrefix}supply`,
                  },
                ],
              },
              {
                scale,
                name: "In Profit",
                title: `${title} Supply In Profit`,
                description: "",
                unit: "Bitcoin",
                icon: "📈",
                bottom: [
                  {
                    title: "Supply",
                    color: colors.profit,
                    datasetPath: `${scale}-to-${datasetPrefix}supply-in-profit`,
                  },
                ],
              },
              {
                scale,
                name: "In Loss",
                title: `${title} Supply In Loss`,
                description: "",
                unit: "Bitcoin",
                icon: "📉",
                bottom: [
                  {
                    title: "Supply",
                    color: colors.loss,
                    datasetPath: `${scale}-to-${datasetPrefix}supply-in-loss`,
                  },
                ],
              },
            ],
          },
          {
            name: "Relative To Circulating",
            tree: [
              {
                scale,
                name: "All",
                title: `${title} Profit And Loss Relative To Circulating Supply`,
                description: "",
                unit: "Percentage",
                icon: "🔀",
                bottom: [
                  {
                    title: "In Profit",
                    color: colors.profit,
                    datasetPath: `${scale}-to-${datasetPrefix}supply-in-profit-to-circulating-supply-ratio`,
                  },
                  {
                    title: "In Loss",
                    color: colors.loss,
                    datasetPath: `${scale}-to-${datasetPrefix}supply-in-loss-to-circulating-supply-ratio`,
                  },
                  {
                    title: "100%",
                    color: colors.white,
                    datasetPath: `${scale}-to-${datasetPrefix}supply-to-circulating-supply-ratio`,
                  },
                  {
                    title: "50%",
                    color: colors.gray,
                    datasetPath: `${scale}-to-${datasetPrefix}halved-supply-to-circulating-supply-ratio`,
                    options: {
                      lineStyle: 4,
                    },
                  },
                ],
              },
              {
                scale,
                name: `Total`,
                title: `${title} Total supply Relative To Circulating Supply`,
                description: "",
                unit: "Percentage",
                icon: "∑",
                bottom: [
                  {
                    title: "Supply",
                    color,
                    datasetPath: `${scale}-to-${datasetPrefix}supply-to-circulating-supply-ratio`,
                  },
                ],
              },
              {
                scale,
                name: "In Profit",
                title: `${title} Supply In Profit Relative To Circulating Supply`,
                description: "",
                unit: "Percentage",
                icon: "📈",
                bottom: [
                  {
                    title: "Supply",
                    color: colors.profit,
                    datasetPath: `${scale}-to-${datasetPrefix}supply-in-profit-to-circulating-supply-ratio`,
                  },
                ],
              },
              {
                scale,
                name: "In Loss",
                title: `${title} Supply In Loss Relative To Circulating Supply`,
                description: "",
                unit: "Percentage",
                icon: "📉",
                bottom: [
                  {
                    title: "Supply",
                    color: colors.loss,
                    datasetPath: `${scale}-to-${datasetPrefix}supply-in-loss-to-circulating-supply-ratio`,
                  },
                ],
              },
            ],
          },
          {
            name: "Relative To Own",
            tree: [
              {
                scale,
                name: "All",
                title: `${title} Supply In Profit And Loss Relative To Own Supply`,
                description: "",
                unit: "Percentage",
                icon: "🔀",
                bottom: [
                  {
                    title: "In Profit",
                    color: colors.profit,
                    datasetPath: `${scale}-to-${datasetPrefix}supply-in-profit-to-own-supply-ratio`,
                  },
                  {
                    title: "In Loss",
                    color: colors.loss,
                    datasetPath: `${scale}-to-${datasetPrefix}supply-in-loss-to-own-supply-ratio`,
                  },
                  {
                    title: "100%",
                    color: colors.white,
                    datasetPath: `${scale}-to-100`,
                    options: {
                      lastValueVisible: false,
                    },
                  },
                  {
                    title: "50%",
                    color: colors.gray,
                    datasetPath: `${scale}-to-50`,
                    options: {
                      lineStyle: 4,
                      lastValueVisible: false,
                    },
                  },
                ],
              },
              {
                scale,
                name: "In Profit",
                title: `${title} Supply In Profit Relative To Own Supply`,
                description: "",
                unit: "Percentage",
                icon: "📈",
                bottom: [
                  {
                    title: "Supply",
                    color: colors.profit,
                    datasetPath: `${scale}-to-${datasetPrefix}supply-in-profit-to-own-supply-ratio`,
                  },
                ],
              },
              {
                scale,
                name: "In Loss",
                title: `${title} Supply In Loss Relative To Own Supply`,
                description: "",
                unit: "Percentage",
                icon: "📉",
                bottom: [
                  {
                    title: "Supply",
                    color: colors.loss,
                    datasetPath: `${scale}-to-${datasetPrefix}supply-in-loss-to-own-supply-ratio`,
                  },
                ],
              },
            ],
          },
          // createMomentumPresetFolder({
          //   datasets: datasets[scale],
          //   scale,
          //   id: `${scale}-${id}-supply-in-profit-and-loss-percentage-self`,
          //   title: `${title} Supply In Profit And Loss (% Self)`,
          //   datasetId: `${datasetId}SupplyPNL%Self`,
          // }),
        ],
      };
    }

    /**
     * @param {Object} args
     * @param {Scale} args.scale
     * @param {AnyPossibleCohortId} args.datasetId
     * @param {string} args.title
     * @param {Color} args.color
     * @returns {PartialPresetFolder}
     */
    function createCohortPresetPricesPaidFolder({
      scale,
      color,
      datasetId,
      title,
    }) {
      /**
       * @param {Object} args
       * @param {Scale} args.scale
       * @param {AnyPossibleCohortId} args.cohortId
       * @param {PercentileId} args.id
       * @returns {AnyDatasetPath}
       */
      function generatePath({ scale, cohortId, id }) {
        const datasetPrefix = datasetIdToPrefix(cohortId);
        return /** @type {const} */ (`${scale}-to-${datasetPrefix}${id}`);
      }

      return {
        name: "Prices Paid",
        tree: [
          {
            scale,
            name: `Average`,
            title: `${title} Average Price Paid - Realized Price`,
            description: "",
            unit: "US Dollars",
            icon: "~",
            top: [
              {
                title: "Average",
                color,
                datasetPath: `${scale}-to-${datasetIdToPrefix(
                  datasetId
                )}realized-price`,
              },
            ],
          },
          {
            scale,
            name: `Deciles`,
            title: `${title} deciles`,
            icon: "🌗",
            description: "",
            unit: "US Dollars",
            top: consts.percentiles
              .filter(({ value }) => Number(value) % 10 === 0)
              .map(({ name, id }) => {
                const datasetPath = generatePath({
                  scale,
                  cohortId: datasetId,
                  id,
                });

                return {
                  datasetPath,
                  color,
                  title: name,
                };
              }),
          },
          ...consts.percentiles.map((percentile) => {
            /** @type {PartialPreset} */
            const preset = {
              scale,
              name: percentile.name,
              title: `${title} ${percentile.title}`,
              description: "",
              unit: "US Dollars",
              icon: "🌓",
              top: [
                {
                  title: percentile.name,
                  color,
                  datasetPath: generatePath({
                    scale,
                    cohortId: datasetId,
                    id: percentile.id,
                  }),
                },
              ],
            };
            return preset;
          }),
        ],
      };
    }

    /**
     * @param {Object} args
     * @param {string} args.name
     * @param {Scale} args.scale
     * @param {AnyPossibleCohortId} args.datasetId
     * @param {string} args.title
     * @param {Color} args.color
     * @returns {PartialPresetTree}
     */
    function createCohortPresetList({ name, scale, color, datasetId, title }) {
      return [
        createCohortPresetUTXOFolder({
          color,
          datasetId,
          scale,
          title,
        }),
        createCohortPresetRealizedFolder({
          color,
          datasetId,
          scale,
          title,
        }),
        createCohortPresetUnrealizedFolder({
          color,
          datasetId,
          scale,
          title,
        }),
        createCohortPresetSupplyFolder({
          color,
          datasetId,
          scale,
          title,
        }),
        createCohortPresetPricesPaidFolder({
          color,
          datasetId,
          scale,
          title,
        }),
      ];
    }

    /**
     * @param {Object} args
     * @param {Scale} args.scale
     * @param {string} args.name
     * @param {AddressCohortId | ""} args.datasetId
     * @param {Color} args.color
     * @returns {PartialPresetFolder}
     */
    function createLiquidityFolder({ scale, color, name, datasetId }) {
      return {
        name: `Split By Liquidity`,
        tree: consts.liquidities.map((liquidity) => {
          /** @type {PartialPresetFolder} */
          const folder = {
            name: liquidity.name,
            tree: createCohortPresetList({
              title: `${liquidity.name} ${name}`,
              name: `${liquidity.name} ${name}`,
              scale,
              color,
              datasetId: !datasetId
                ? liquidity.id
                : `${liquidity.id}-${datasetId}`,
            }),
          };
          return folder;
        }),
      };
    }

    /**
     * @param {Object} args
     * @param {Scale} args.scale
     * @param {string} args.name
     * @param {AnyPossibleCohortId} args.datasetId
     * @param {string} args.title
     * @param {Color} args.color
     * @returns {PartialPresetFolder}
     */
    function createCohortPresetFolder({
      scale,
      color,
      name,
      datasetId,
      title,
    }) {
      return {
        name,
        tree: createCohortPresetList({
          title,
          name,
          scale,
          color,
          datasetId: datasetId,
        }),
      };
    }

    /**
     * @param {Scale} scale
     * @returns {PartialPresetFolder}
     */
    function createHodlersPresets(scale) {
      return {
        name: "Hodlers",
        tree: [
          {
            scale,
            name: `Hodl Supply`,
            title: `Hodl Supply`,
            description: "",
            icon: "🌊",
            unit: "Percentage",
            bottom: [
              {
                title: `24h`,
                color: colors.up_to_1d,
                datasetPath: `${scale}-to-up-to-1d-supply-to-circulating-supply-ratio`,
              },

              ...consts.fromXToY.map(({ key, id, name, legend }) => ({
                title: legend,
                color: colors[key],
                datasetPath: /** @type {const} */ (
                  `${scale}-to-${id}-supply-to-circulating-supply-ratio`
                ),
              })),

              {
                title: `15y+`,
                color: colors.from_15y,
                datasetPath: `${scale}-to-from-15y-supply-to-circulating-supply-ratio`,
              },
            ],
          },
          ...consts.xth.map(({ key, id, name, legend }) =>
            createCohortPresetFolder({
              scale,
              color: colors[key],
              name: legend,
              datasetId: id,
              title: name,
            })
          ),
          {
            name: "Up To X",
            tree: consts.upTo.map(({ key, id, name }) =>
              createCohortPresetFolder({
                scale,
                color: colors[key],
                name,
                datasetId: id,
                title: name,
              })
            ),
          },
          {
            name: "From X To Y",
            tree: consts.fromXToY.map(({ key, id, name }) =>
              createCohortPresetFolder({
                scale,
                color: colors[key],
                name,
                datasetId: id,
                title: name,
              })
            ),
          },
          {
            name: "From X",
            tree: consts.fromX.map(({ key, id, name }) =>
              createCohortPresetFolder({
                scale,
                color: colors[key],
                name,
                datasetId: id,
                title: name,
              })
            ),
          },
          {
            name: "Years",
            tree: consts.year.map(({ key, id, name }) =>
              createCohortPresetFolder({
                scale,
                color: colors[key],
                name,
                datasetId: id,
                title: name,
              })
            ),
          },
        ],
      };
    }

    /**
     * @param {Object} args
     * @param {Scale} args.scale
     * @param {string} args.name
     * @param {AddressCohortId} args.datasetId
     * @param {Color} args.color
     * @returns {PartialPreset}
     */
    function createAddressCountPreset({ scale, color, name, datasetId }) {
      return {
        scale,
        name: `Address Count`,
        title: `${name} Address Count`,
        description: "",
        unit: "Count",
        icon: "📕",
        bottom: [
          {
            title: "Address Count",
            color,
            datasetPath: `${scale}-to-${datasetId}-address-count`,
          },
        ],
      };
    }

    /**
     * @param {Object} args
     * @param {Scale} args.scale
     * @param {string} args.name
     * @param {AddressCohortId} args.datasetId
     * @param {Color} args.color
     * @param { string} [args.filenameAddon]
     * @returns {PartialPresetFolder}
     */
    function createAddressPresetFolder({
      scale,
      color,
      name,
      filenameAddon,
      datasetId,
    }) {
      return {
        name: filenameAddon ? `${name} - ${filenameAddon}` : name,
        tree: [
          createAddressCountPreset({ scale, name, datasetId, color }),
          ...createCohortPresetList({
            title: name,
            scale,
            name,
            color,
            datasetId,
          }),
          createLiquidityFolder({
            scale,
            name,
            datasetId,
            color,
          }),
        ],
      };
    }

    /**
     * @param {Scale} scale
     * @returns {PartialPresetFolder}
     */
    function createAddressesPresets(scale) {
      return {
        name: "Addresses",
        tree: [
          {
            scale,
            name: `Total Non Empty Addresses`,
            title: `Total Non Empty Address`,
            description: "",
            unit: "Count",
            icon: "💳",
            bottom: [
              {
                title: `Total Non Empty Address`,
                color: colors.bitcoin,
                datasetPath: `${scale}-to-address-count`,
              },
            ],
          },
          {
            scale,
            name: `New Addresses`,
            title: `New Addresses`,
            description: "",
            unit: "Count",
            icon: "🏡",
            bottom: [
              {
                title: `New Addresses`,
                color: colors.bitcoin,
                datasetPath: `${scale}-to-new-addresses`,
              },
            ],
          },
          {
            scale,
            name: `Total Addresses Created`,
            title: `Total Addresses Created`,
            description: "",
            unit: "Count",
            icon: "🏠",
            bottom: [
              {
                title: `Total Addresses Created`,
                color: colors.bitcoin,
                datasetPath: `${scale}-to-created-addresses`,
              },
            ],
          },
          {
            scale,
            name: `Total Empty Addresses`,
            title: `Total Empty Addresses`,
            description: "",
            unit: "Count",
            icon: "🗑️",
            bottom: [
              {
                title: `Total Empty Addresses`,
                color: colors.darkWhite,
                datasetPath: `${scale}-to-empty-addresses`,
              },
            ],
          },
          {
            name: "By Size",
            tree: consts.size.map(({ key, name, size }) =>
              createAddressPresetFolder({
                scale,
                color: colors[key],
                name,
                filenameAddon: size,
                datasetId: key,
              })
            ),
          },
          {
            scale,
            name: "By Type",
            tree: consts.type.map(({ key, name }) =>
              createAddressPresetFolder({
                scale,
                color: colors[key],
                name,
                datasetId: key,
              })
            ),
          },
        ],
      };
    }

    /**
     * @param {Scale} scale
     * @returns {PartialPresetFolder}
     */
    function createCointimePresets(scale) {
      return {
        name: "Cointime Economics",
        tree: [
          {
            name: "Prices",
            tree: [
              {
                scale,
                icon: "🔀",
                name: "All",
                title: "All Cointime Prices",
                description: "",
                unit: "US Dollars",
                top: [
                  {
                    title: "Vaulted Price",
                    color: colors.vaultedness,
                    datasetPath: `${scale}-to-vaulted-price`,
                  },
                  {
                    title: "Active Price",
                    color: colors.liveliness,
                    datasetPath: `${scale}-to-active-price`,
                  },
                  {
                    title: "True Market Mean",
                    color: colors.trueMarketMeanPrice,
                    datasetPath: `${scale}-to-true-market-mean`,
                  },
                  {
                    title: "Realized Price",
                    color: colors.bitcoin,
                    datasetPath: `${scale}-to-realized-price`,
                  },
                  {
                    title: "Cointime",
                    color: colors.cointimePrice,
                    datasetPath: `${scale}-to-cointime-price`,
                  },
                ],
              },
              {
                name: "Active",
                tree: [
                  {
                    scale,
                    icon: "❤️",
                    name: "Price",
                    title: "Active Price",
                    description: "",
                    unit: "US Dollars",
                    top: [
                      {
                        title: "Active Price",
                        color: colors.liveliness,
                        datasetPath: `${scale}-to-active-price`,
                      },
                    ],
                  },
                  createRatioFolder({
                    color: colors.liveliness,
                    ratioDatasetPath: `${scale}-to-market-price-to-active-price-ratio`,
                    scale,
                    title: "Active Price",
                    valueDatasetPath: `${scale}-to-active-price`,
                  }),
                ],
              },
              {
                name: "Vaulted",
                tree: [
                  {
                    scale,
                    icon: "🏦",
                    name: "Price",
                    title: "Vaulted Price",
                    description: "",
                    unit: "US Dollars",
                    top: [
                      {
                        title: "Vaulted Price",
                        color: colors.vaultedness,
                        datasetPath: `${scale}-to-vaulted-price`,
                      },
                    ],
                  },
                  createRatioFolder({
                    color: colors.vaultedness,
                    ratioDatasetPath: `${scale}-to-market-price-to-vaulted-price-ratio`,
                    scale,
                    title: "Vaulted Price",
                    valueDatasetPath: `${scale}-to-vaulted-price`,
                  }),
                ],
              },
              {
                name: "True Market Mean",
                tree: [
                  {
                    scale,
                    icon: "〰️",
                    name: "Price",
                    title: "True Market Mean",
                    description: "",
                    unit: "US Dollars",
                    top: [
                      {
                        title: "True Market Mean",
                        color: colors.trueMarketMeanPrice,
                        datasetPath: `${scale}-to-true-market-mean`,
                      },
                    ],
                  },
                  createRatioFolder({
                    color: colors.liveliness,
                    ratioDatasetPath: `${scale}-to-market-price-to-true-market-mean-ratio`,
                    scale,
                    title: "True Market Mean",
                    valueDatasetPath: `${scale}-to-true-market-mean`,
                  }),
                ],
              },
              {
                name: "Cointime Price",
                tree: [
                  {
                    scale,
                    icon: "⏱️",
                    name: "Price",
                    title: "Cointime Price",
                    description: "",
                    unit: "US Dollars",
                    top: [
                      {
                        title: "Cointime",
                        color: colors.cointimePrice,
                        datasetPath: `${scale}-to-cointime-price`,
                      },
                    ],
                  },
                  createRatioFolder({
                    color: colors.cointimePrice,
                    ratioDatasetPath: `${scale}-to-market-price-to-cointime-price-ratio`,
                    scale,
                    title: "Cointime",
                    valueDatasetPath: `${scale}-to-cointime-price`,
                  }),
                ],
              },
            ],
          },
          {
            name: "Capitalizations",
            tree: [
              {
                scale,
                icon: "🔀",
                name: "All",
                title: "Cointime Capitalizations",
                description: "",
                unit: "US Dollars",
                bottom: [
                  {
                    title: "Market Cap",
                    color: colors.white,
                    datasetPath: `${scale}-to-market-cap`,
                  },
                  {
                    title: "Realized Cap",
                    color: colors.realizedCap,
                    datasetPath: `${scale}-to-realized-cap`,
                  },
                  {
                    title: "Investor Cap",
                    color: colors.investorCap,
                    datasetPath: `${scale}-to-investor-cap`,
                  },
                  {
                    title: "Thermo Cap",
                    color: colors.thermoCap,
                    datasetPath: `${scale}-to-thermo-cap`,
                  },
                ],
              },
              {
                scale,
                icon: "⛏️",
                name: "Thermo Cap",
                title: "Thermo Cap",
                description: "",
                unit: "US Dollars",
                bottom: [
                  {
                    title: "Thermo Cap",
                    color: colors.thermoCap,
                    datasetPath: `${scale}-to-thermo-cap`,
                  },
                ],
              },
              {
                scale,
                icon: "🧑‍💼",
                name: "Investor Cap",
                title: "Investor Cap",
                description: "",
                unit: "US Dollars",
                bottom: [
                  {
                    title: "Investor Cap",
                    color: colors.investorCap,
                    datasetPath: `${scale}-to-investor-cap`,
                  },
                ],
              },
              {
                scale,
                icon: "➗",
                name: "Thermo Cap To Investor Cap Ratio",
                title: "Thermo Cap To Investor Cap Ratio",
                description: "",
                unit: "Percentage",
                bottom: [
                  {
                    title: "Ratio",
                    color: colors.bitcoin,
                    datasetPath: `${scale}-to-thermo-cap-to-investor-cap-ratio`,
                  },
                ],
              },
            ],
          },
          {
            name: "Coinblocks",
            tree: [
              {
                scale,
                icon: "🧱",
                name: "All",
                title: "All Coinblocks",
                description: "",
                unit: "Coinblocks",
                bottom: [
                  {
                    title: "Coinblocks Created",
                    color: colors.coinblocksCreated,
                    datasetPath: `${scale}-to-coinblocks-created`,
                  },
                  {
                    title: "Coinblocks Destroyed",
                    color: colors.coinblocksDestroyed,
                    datasetPath: `${scale}-to-coinblocks-destroyed`,
                  },
                  {
                    title: "Coinblocks Stored",
                    color: colors.coinblocksStored,
                    datasetPath: `${scale}-to-coinblocks-stored`,
                  },
                ],
              },
              {
                scale,
                icon: "🧊",
                name: "Created",
                title: "Coinblocks Created",
                description: "",
                unit: "Coinblocks",
                bottom: [
                  {
                    title: "Coinblocks Created",
                    color: colors.coinblocksCreated,
                    datasetPath: `${scale}-to-coinblocks-created`,
                  },
                ],
              },
              {
                scale,
                icon: "⛓️‍💥",
                name: "Destroyed",
                title: "Coinblocks Destroyed",
                description: "",
                unit: "Coinblocks",
                bottom: [
                  {
                    title: "Coinblocks Destroyed",
                    color: colors.coinblocksDestroyed,
                    datasetPath: `${scale}-to-coinblocks-destroyed`,
                  },
                ],
              },
              {
                scale,
                icon: "🗄️",
                name: "Stored",
                title: "Coinblocks Stored",
                description: "",
                unit: "Coinblocks",
                bottom: [
                  {
                    title: "Coinblocks Stored",
                    color: colors.coinblocksStored,
                    datasetPath: `${scale}-to-coinblocks-stored`,
                  },
                ],
              },
            ],
          },
          {
            name: "Cumulative Coinblocks",
            tree: [
              {
                scale,
                icon: "🔀",
                name: "All",
                title: "All Cumulative Coinblocks",
                description: "",
                unit: "Coinblocks",
                bottom: [
                  {
                    title: "Cumulative Coinblocks Created",
                    color: colors.coinblocksCreated,
                    datasetPath: `${scale}-to-cumulative-coinblocks-created`,
                  },
                  {
                    title: "Cumulative Coinblocks Destroyed",
                    color: colors.coinblocksDestroyed,
                    datasetPath: `${scale}-to-cumulative-coinblocks-destroyed`,
                  },
                  {
                    title: "Cumulative Coinblocks Stored",
                    color: colors.coinblocksStored,
                    datasetPath: `${scale}-to-cumulative-coinblocks-stored`,
                  },
                ],
              },
              {
                scale,
                icon: "🧊",
                name: "Created",
                title: "Cumulative Coinblocks Created",
                description: "",
                unit: "Coinblocks",
                bottom: [
                  {
                    title: "Cumulative Coinblocks Created",
                    color: colors.coinblocksCreated,
                    datasetPath: `${scale}-to-cumulative-coinblocks-created`,
                  },
                ],
              },
              {
                scale,
                icon: "⛓️‍💥",
                name: "Destroyed",
                title: "Cumulative Coinblocks Destroyed",
                description: "",
                unit: "Coinblocks",
                bottom: [
                  {
                    title: "Cumulative Coinblocks Destroyed",
                    color: colors.coinblocksDestroyed,
                    datasetPath: `${scale}-to-cumulative-coinblocks-destroyed`,
                  },
                ],
              },
              {
                scale,
                icon: "🗄️",
                name: "Stored",
                title: "Cumulative Coinblocks Stored",
                description: "",
                unit: "Coinblocks",
                bottom: [
                  {
                    title: "Cumulative Coinblocks Stored",
                    color: colors.coinblocksStored,
                    datasetPath: `${scale}-to-cumulative-coinblocks-stored`,
                  },
                ],
              },
            ],
          },
          {
            name: "Liveliness & Vaultedness",
            tree: [
              {
                scale,
                icon: "❤️",
                name: "Liveliness - Activity",
                title: "Liveliness (Activity)",
                description: "",
                unit: "",
                bottom: [
                  {
                    title: "Liveliness",
                    color: colors.liveliness,
                    datasetPath: `${scale}-to-liveliness`,
                  },
                ],
              },
              {
                scale,
                icon: "🏦",
                name: "Vaultedness",
                title: "Vaultedness",
                description: "",
                unit: "",
                bottom: [
                  {
                    title: "Vaultedness",
                    color: colors.vaultedness,
                    datasetPath: `${scale}-to-vaultedness`,
                  },
                ],
              },
              {
                scale,
                icon: "⚔️",
                name: "Versus",
                title: "Liveliness V. Vaultedness",
                description: "",
                unit: "",
                bottom: [
                  {
                    title: "Liveliness",
                    color: colors.liveliness,
                    datasetPath: `${scale}-to-liveliness`,
                  },
                  {
                    title: "Vaultedness",
                    color: colors.vaultedness,
                    datasetPath: `${scale}-to-vaultedness`,
                  },
                ],
              },
              {
                scale,
                icon: "➗",
                name: "Activity To Vaultedness Ratio",
                title: "Activity To Vaultedness Ratio",
                description: "",
                unit: "Percentage",
                bottom: [
                  {
                    title: "Activity To Vaultedness Ratio",
                    color: colors.activityToVaultednessRatio,
                    datasetPath: `${scale}-to-activity-to-vaultedness-ratio`,
                  },
                ],
              },
              {
                scale,
                icon: "❤️",
                name: "Concurrent Liveliness - Supply Adjusted Coindays Destroyed",
                title:
                  "Concurrent Liveliness - Supply Adjusted Coindays Destroyed",
                description: "",
                unit: "",
                bottom: [
                  {
                    title: "Concurrent Liveliness 14d Median",
                    color: colors.liveliness,
                    datasetPath: `${scale}-to-concurrent-liveliness-2w-median`,
                  },
                  {
                    title: "Concurrent Liveliness",
                    color: colors.darkLiveliness,
                    datasetPath: `${scale}-to-concurrent-liveliness`,
                  },
                ],
              },
              {
                scale,
                icon: "📊",
                name: "Liveliness Incremental Change",
                title: "Liveliness Incremental Change",
                description: "",
                unit: "",
                bottom: [
                  {
                    title: "Liveliness Incremental Change",
                    color: colors.darkLiveliness,
                    type: "Baseline",
                    datasetPath: `${scale}-to-liveliness-net-change`,
                  },
                  {
                    title: "Liveliness Incremental Change 14 Day Median",
                    color: colors.liveliness,
                    type: "Baseline",
                    datasetPath: `${scale}-to-liveliness-net-change-2w-median`,
                  },
                ],
              },
            ],
          },
          {
            name: "Supply",
            tree: [
              {
                scale,
                icon: "🏦",
                name: "Vaulted",
                title: "Vaulted Supply",
                description: "",
                unit: "Bitcoin",
                bottom: [
                  {
                    title: "Vaulted Supply",
                    color: colors.vaultedness,
                    datasetPath: `${scale}-to-vaulted-supply`,
                  },
                ],
              },
              {
                scale,
                icon: "❤️",
                name: "Active",
                title: "Active Supply",
                description: "",
                unit: "Bitcoin",
                bottom: [
                  {
                    title: "Active Supply",
                    color: colors.liveliness,
                    datasetPath: `${scale}-to-active-supply`,
                  },
                ],
              },
              {
                scale,
                icon: "⚔️",
                name: "Vaulted V. Active",
                title: "Vaulted V. Active",
                description: "",
                unit: "Bitcoin",
                bottom: [
                  {
                    title: "Circulating Supply",
                    color: colors.coinblocksCreated,
                    datasetPath: `${scale}-to-supply`,
                  },
                  {
                    title: "Vaulted Supply",
                    color: colors.vaultedness,
                    datasetPath: `${scale}-to-vaulted-supply`,
                  },
                  {
                    title: "Active Supply",
                    color: colors.liveliness,
                    datasetPath: `${scale}-to-active-supply`,
                  },
                ],
              },
              // TODO: Fix, Bad data
              // {
              //   id: 'asymptomatic-supply-regions',
              //   icon: IconTablerDirections,
              //   name: 'Asymptomatic Supply Regions',
              //   title: 'Asymptomatic Supply Regions',
              //   description: '',
              //   applyPreset(params) {
              //     return applyMultipleSeries({
              //       ...params,
              //       priceScaleOptions: {
              //         halved: true,
              //       },
              //       list: [
              //         {
              //           id: 'min-vaulted',
              //           title: 'Min Vaulted Supply',
              //           color: colors.vaultedness,
              //           dataset: params.`/${scale}-to-dateToMinVaultedSupply,
              //         },
              //         {
              //           id: 'max-active',
              //           title: 'Max Active Supply',
              //           color: colors.liveliness,
              //           dataset: params.`/${scale}-to-dateToMaxActiveSupply,
              //         },
              //       ],
              //     })
              //   },
              // },
              {
                scale,
                icon: "🏦",
                name: "Vaulted Net Change",
                title: "Vaulted Supply Net Change",
                description: "",
                unit: "Bitcoin",
                bottom: [
                  {
                    title: "Vaulted Supply Net Change",
                    color: colors.vaultedness,
                    datasetPath: `${scale}-to-vaulted-supply-net-change`,
                  },
                ],
              },
              {
                scale,
                icon: "❤️",
                name: "Active Net Change",
                title: "Active Supply Net Change",
                description: "",
                unit: "Bitcoin",
                bottom: [
                  {
                    title: "Active Supply Net Change",
                    color: colors.liveliness,
                    datasetPath: `${scale}-to-active-supply-net-change`,
                  },
                ],
              },
              {
                scale,
                icon: "⚔️",
                name: "Active VS. Vaulted 90D Net Change",
                title: "Active VS. Vaulted 90 Day Supply Net Change",
                description: "",
                unit: "Bitcoin",
                bottom: [
                  {
                    title: "Active Supply Net Change",
                    color: colors.liveliness,
                    datasetPath: `${scale}-to-active-supply-3m-net-change`,
                    type: "Baseline",
                  },
                  {
                    title: "Vaulted Supply Net Change",
                    color: colors.vaultedPrice,
                    type: "Baseline",
                    datasetPath: `${scale}-to-vaulted-supply-3m-net-change`,
                  },
                ],
              },
              // TODO: Fix, Bad data
              // {
              //   id: 'vaulted-supply-annualized-net-change',
              //   icon: IconTablerBuildingBank,
              //   name: 'Vaulted Annualized Net Change',
              //   title: 'Vaulted Supply Annualized Net Change',
              //   description: '',
              //   applyPreset(params) {
              //     return applyMultipleSeries({
              //       ...params,
              //       priceScaleOptions: {
              //         halved: true,
              //       },
              //       list: [
              //         {
              //           id: 'vaulted-annualized-supply-net-change',
              //           title: 'Vaulted Supply Annualized Net Change',
              //           color: colors.vaultedness,
              //           dataset:
              //             `/${scale}-to-vaultedAnnualizedSupplyNetChange,
              //         },
              //       ],
              //     })
              //   },
              // },

              // TODO: Fix, Bad data
              // {
              //   id: 'vaulting-rate',
              //   icon: IconTablerBuildingBank,
              //   name: 'Vaulting Rate',
              //   title: 'Vaulting Rate',
              //   description: '',
              //   applyPreset(params) {
              //     return applyMultipleSeries({
              //       ...params,
              //       priceScaleOptions: {
              //         halved: true,
              //       },
              //       list: [
              //         {
              //           id: 'vaulting-rate',
              //           title: 'Vaulting Rate',
              //           color: colors.vaultedness,
              //           dataset: `${scale}-to-vaultingRate,
              //         },
              //         {
              //           id: 'nominal-inflation-rate',
              //           title: 'Nominal Inflation Rate',
              //           color: colors.orange,
              //           dataset: params.`/${scale}-to-dateToYearlyInflationRate,
              //         },
              //       ],
              //     })
              //   },
              // },

              // TODO: Fix, Bad data
              // {
              //   id: 'active-supply-net-change-decomposition',
              //   icon: IconTablerArrowsCross,
              //   name: 'Active Supply Net Change Decomposition (90D)',
              //   title: 'Active Supply Net 90 Day Change Decomposition',
              //   description: '',
              //   applyPreset(params) {
              //     return applyMultipleSeries({
              //       ...params,
              //       priceScaleOptions: {
              //         halved: true,
              //       },
              //       list: [
              //         {
              //           id: 'issuance-change',
              //           title: 'Change From Issuance',
              //           color: colors.emerald,
              //           dataset:
              //             params.params.datasets[scale]
              //               [scale].activeSupplyChangeFromIssuance90dChange,
              //         },
              //         {
              //           id: 'transactions-change',
              //           title: 'Change From Transactions',
              //           color: colors.rose,
              //           dataset:
              //             params.params.datasets[scale]
              //               [scale].activeSupplyChangeFromTransactions90dChange,
              //         },
              //         // {
              //         //   id: 'active',
              //         //   title: 'Active Supply',
              //         //   color: colors.liveliness,
              //         //   dataset: `${scale}-to-activeSupply,
              //         // },
              //       ],
              //     })
              //   },
              // },

              {
                scale,
                icon: "📈",
                name: "In Profit",
                title: "Cointime Supply In Profit",
                description: "",
                unit: "Bitcoin",
                bottom: [
                  {
                    title: "Circulating Supply",
                    color: colors.coinblocksCreated,
                    datasetPath: `${scale}-to-supply`,
                  },
                  {
                    title: "Vaulted Supply",
                    color: colors.vaultedness,
                    datasetPath: `${scale}-to-vaulted-supply`,
                  },
                  {
                    title: "Supply in profit",
                    color: colors.bitcoin,
                    datasetPath: `${scale}-to-supply-in-profit`,
                  },
                ],
              },
              {
                scale,
                icon: "📉",
                name: "In Loss",
                title: "Cointime Supply In Loss",
                description: "",
                unit: "Bitcoin",
                bottom: [
                  {
                    title: "Circulating Supply",
                    color: colors.coinblocksCreated,
                    datasetPath: `${scale}-to-supply`,
                  },
                  {
                    title: "Active Supply",
                    color: colors.liveliness,
                    datasetPath: `${scale}-to-active-supply`,
                  },
                  {
                    title: "Supply in Loss",
                    color: colors.bitcoin,
                    datasetPath: `${scale}-to-supply-in-loss`,
                  },
                ],
              },
            ],
          },
          {
            scale,
            icon: "🏭",
            name: "Cointime Yearly Inflation Rate",
            title: "Cointime-Adjusted Yearly Inflation Rate",
            description: "",
            unit: "Percentage",
            bottom: [
              {
                title: "Cointime Adjusted",
                color: colors.coinblocksCreated,
                datasetPath: `${scale}-to-cointime-adjusted-yearly-inflation-rate`,
              },
              {
                title: "Nominal",
                color: colors.bitcoin,
                datasetPath: `${scale}-to-yearly-inflation-rate`,
              },
            ],
          },
          {
            scale,
            icon: "💨",
            name: "Cointime Velocity",
            title: "Cointime-Adjusted Transactions Velocity",
            description: "",
            unit: "",
            bottom: [
              {
                title: "Cointime Adjusted",
                color: colors.coinblocksCreated,
                datasetPath: `${scale}-to-cointime-adjusted-velocity`,
              },
              {
                title: "Nominal",
                color: colors.bitcoin,
                datasetPath: `${scale}-to-transaction-velocity`,
              },
            ],
          },
        ],
      };
    }

    return [
      {
        name: "Dashboards",
        tree: [],
      },
      {
        name: "Charts",
        tree: [
          {
            name: "By Date",
            tree: [
              createMarketPresets("date"),
              createBlocksPresets("date"),
              createMinersPresets("date"),
              createTransactionsPresets("date"),
              ...createCohortPresetList({
                scale: "date",
                color: colors.bitcoin,
                datasetId: "",
                name: "",
                title: "",
              }),
              createLiquidityFolder({
                scale: "date",
                color: colors.bitcoin,
                datasetId: "",
                name: "",
              }),
              createHodlersPresets("date"),
              createAddressesPresets("date"),
              createCointimePresets("date"),
            ],
          },
          {
            name: "By Height",
            tree: [
              createMarketPresets("height"),
              createBlocksPresets("height"),
              createMinersPresets("height"),
              createTransactionsPresets("height"),
              ...createCohortPresetList({
                scale: "height",
                color: colors.bitcoin,
                datasetId: "",
                name: "",
                title: "",
              }),
              createLiquidityFolder({
                scale: "height",
                color: colors.bitcoin,
                datasetId: "",
                name: "",
              }),
              createHodlersPresets("height"),
              createAddressesPresets("height"),
              createCointimePresets("height"),
            ],
          },
        ],
      },
    ];
  }

  const partialTree = createPartialTree();

  /** @type {string[]} */
  const presetsIds = [];

  /** @type {Preset[]} */
  const presetsList = [];

  /** @type {HTMLDetailsElement[]} */
  const detailsList = [];

  /** @typedef {'all' | 'favorites' | 'new'} FoldersFilter */

  const foldersFilterLocalStorageKey = "folders-filter";
  const filter = createSignal(
    /** @type {FoldersFilter} */ (
      localStorage.getItem(foldersFilterLocalStorageKey) || "all"
    )
  );

  const favoritesCount = createSignal(0);
  const newCount = createSignal(0);

  /** @param {Preset} preset  */
  function createCountersEffects(preset) {
    let firstFavoritesRun = true;

    createEffect(() => {
      if (preset.isFavorite()) {
        favoritesCount.set((c) => c + 1);
      } else if (!firstFavoritesRun) {
        favoritesCount.set((c) => c - 1);
      }
      firstFavoritesRun = false;
    });

    let firstNewRun = true;

    createEffect(() => {
      if (!preset.visited()) {
        newCount.set((c) => c + 1);
      } else if (!firstNewRun) {
        newCount.set((c) => c - 1);
      }
      firstNewRun = false;
    });
  }

  /** @param {Preset} preset  */
  function presetToVisitedLocalStorageKey(preset) {
    return `${preset.id}-visited`;
  }

  /**
   * @param {Preset} preset
   * @param {Series | SeriesBlueprint} series
   */
  function presetAndSeriesToLocalStorageKey(preset, series) {
    return `${preset.id}-${stringToId(series.title)}`;
  }

  /**
   * @param {Object} args
   * @param {string} args.name
   * @param {string} args.inputName
   * @param {string} args.inputId
   * @param {string} args.inputValue
   * @param {string} args.labelTitle
   * @param {VoidFunction} args.onClick
   */
  function createLabel({
    inputId,
    inputName,
    inputValue,
    labelTitle,
    name,
    onClick,
  }) {
    const label = window.document.createElement("label");

    const input = window.document.createElement("input");
    input.type = "radio";
    input.name = inputName;
    input.id = inputId;
    input.value = inputValue;
    label.append(input);

    label.id = `${inputId}-label`;
    // @ts-ignore
    label.for = inputId;
    label.title = labelTitle;

    const spanMain = window.document.createElement("span");
    spanMain.classList.add("main");
    label.append(spanMain);

    const spanName = window.document.createElement("span");
    spanName.classList.add("name");
    spanName.innerHTML = name;
    spanMain.append(spanName);

    label.addEventListener("click", (event) => {
      event.stopPropagation();
      event.preventDefault();
      onClick();
    });

    return {
      label,
      input,
      spanMain,
      spanName,
    };
  }

  /**
   * @param {Object} args
   * @param {Preset} args.preset
   * @param {string} args.frame
   * @param {string} [args.name]
   * @param {string} [args.top]
   * @param {string} [args.id]
   * @param {Owner | null} [args.owner]
   */
  function createPresetLabel({ preset, frame, name, top, id, owner }) {
    const { input, label, spanMain, spanName } = createLabel({
      inputId: `${preset.id}-${frame}${id || ""}-selector`,
      inputValue: preset.id,
      inputName: `preset-${frame}${id || ""}`,
      labelTitle: preset.title,
      name: name || preset.name,
      onClick: () => selected.set(preset),
    });

    if (top) {
      const small = window.document.createElement("small");
      small.innerHTML = top;
      label.insertBefore(small, spanMain);
    }

    const spanEmoji = window.document.createElement("span");
    spanEmoji.classList.add("emoji");
    spanEmoji.innerHTML = preset.icon;
    spanMain.prepend(spanEmoji);

    /** @type {HTMLSpanElement | undefined} */
    let spanNew;

    if (!preset.visited()) {
      spanNew = window.document.createElement("span");
      spanNew.classList.add("new");
      spanMain.append(spanNew);
    }

    function createFavoriteEffect() {
      // @ts-ignore
      createEffect((_wasFavorite) => {
        const wasFavorite = /** @type {boolean} */ (_wasFavorite);
        const isFavorite = preset.isFavorite();

        if (!wasFavorite && isFavorite) {
          const iconFavorite = window.document.createElement("svg");
          spanMain.append(iconFavorite);
          iconFavorite.outerHTML =
            '<svg viewBox="0 0 20 20" class="favorite"><path fill-rule="evenodd" d="M10.868 2.884c-.321-.772-1.415-.772-1.736 0l-1.83 4.401-4.753.381c-.833.067-1.171 1.107-.536 1.651l3.62 3.102-1.106 4.637c-.194.813.691 1.456 1.405 1.02L10 15.591l4.069 2.485c.713.436 1.598-.207 1.404-1.02l-1.106-4.637 3.62-3.102c.635-.544.297-1.584-.536-1.65l-4.752-.382-1.831-4.401Z" clip-rule="evenodd" /></svg>';
        } else if (wasFavorite && !isFavorite) {
          spanMain.lastElementChild?.remove();
        }

        return isFavorite;
      }, false);
    }

    function createUpdateEffect() {
      createEffect(() => {
        if (selected()?.id === preset.id) {
          input.checked = true;
          spanNew?.remove();
          preset.visited.set(true);
          localStorage.setItem(presetToVisitedLocalStorageKey(preset), "1");
          localStorage.setItem(selectedLocalStorageKey, preset.id);
        } else if (input.checked) {
          input.checked = false;
        }
      });
    }

    if (owner !== undefined) {
      runWithOwner(owner, () => {
        createUpdateEffect();
        createFavoriteEffect();
      });
    } else {
      createUpdateEffect();
      createFavoriteEffect();
    }

    return label;
  }

  /**
   * @param {Preset} preset
   */
  function presetToFavoriteLocalStorageKey(preset) {
    return `${preset.id}-favorite`;
  }

  /**
   * @param {PartialPresetTree} partialTree
   * @param {HTMLElement} parent
   * @param {FilePath | undefined} path
   * @returns {Accessor<number>}
   */
  function processPartialTree(partialTree, parent, path = undefined) {
    const ul = window.document.createElement("ul");
    parent.appendChild(ul);

    /** @type {Accessor<number>[]} */
    const listForSum = [];

    partialTree.forEach((anyPartial, partialIndex) => {
      const li = window.document.createElement("li");
      ul.appendChild(li);

      if ("tree" in anyPartial) {
        const folderId = stringToId(
          `${(path || [])?.map(({ name }) => name).join(" ")} ${
            anyPartial.name
          } folder`
        );

        /** @type {Omit<PresetFolder, keyof PartialPresetFolder>} */
        const restFolder = {
          id: folderId,
        };

        Object.assign(anyPartial, restFolder);

        presetsIds.push(restFolder.id);

        const details = window.document.createElement("details");
        const folderOpenLocalStorageKey = `${folderId}-open`;
        details.open = !!localStorage.getItem(folderOpenLocalStorageKey);
        detailsList.push(details);
        li.appendChild(details);

        const summary = window.document.createElement("summary");
        summary.id = folderId;

        const spanMarker = window.document.createElement("span");
        spanMarker.classList.add("marker");
        spanMarker.innerHTML = "●";
        summary.append(spanMarker);

        const spanName = window.document.createElement("span");
        spanName.classList.add("name");
        spanName.innerHTML = anyPartial.name;
        summary.append(spanName);

        const smallCount = window.document.createElement("small");
        smallCount.hidden = details.open;
        summary.append(smallCount);

        details.appendChild(summary);

        const thisPath = {
          name: anyPartial.name,
          id: restFolder.id,
        };

        details.addEventListener("toggle", () => {
          const open = details.open;

          smallCount.hidden = open;

          if (open) {
            spanMarker.innerHTML = "○";
            localStorage.setItem(folderOpenLocalStorageKey, "1");
          } else {
            spanMarker.innerHTML = "●";
            localStorage.removeItem(folderOpenLocalStorageKey);
          }
        });

        const childPresetsCount = processPartialTree(anyPartial.tree, details, [
          ...(path || []),
          thisPath,
        ]);

        listForSum.push(childPresetsCount);

        createEffect(() => {
          const count = childPresetsCount();
          smallCount.innerHTML = count.toLocaleString();

          if (!count) {
            li.hidden = true;
          } else {
            li.hidden = false;
          }
        });
      } else {
        const id = `${anyPartial.scale}-to-${stringToId(anyPartial.title)}`;

        /** @type {Omit<Preset, keyof PartialPreset>} */
        const restPreset = {
          id,
          path: path || [],
          serializedPath: `/ ${[
            ...(path || []).map(({ name }) => name),
            anyPartial.name,
          ].join(" / ")}`,
          isFavorite: createSignal(false),
          visited: createSignal(false),
        };

        Object.assign(anyPartial, restPreset);

        const preset = /** @type {Preset} */ (anyPartial);

        if (!selected() && (firstTime || savedSelectedId === preset.id)) {
          selected.set(preset);
        }

        preset.isFavorite.set(
          !!localStorage.getItem(presetToFavoriteLocalStorageKey(preset))
        );
        preset.visited.set(
          !!localStorage.getItem(presetToVisitedLocalStorageKey(preset))
        );

        createCountersEffects(preset);

        presetsList.push(preset);
        presetsIds.push(preset.id);

        const label = createPresetLabel({
          preset,
          frame: "folders",
        });
        li.append(label);

        const inDom = createSignal(true);
        function createDomEffect() {
          createEffect(() => {
            switch (filter()) {
              case "all": {
                if (!inDom()) {
                  insertElementAtIndex(ul, li, partialIndex);
                  inDom.set(true);
                }
                break;
              }
              case "favorites": {
                if (preset.isFavorite()) {
                  if (!inDom()) {
                    insertElementAtIndex(ul, li, partialIndex);
                    inDom.set(true);
                  }
                } else if (inDom()) {
                  inDom.set(false);
                  ul.removeChild(li);
                }
                break;
              }
              case "new": {
                if (!preset.visited()) {
                  if (!inDom()) {
                    insertElementAtIndex(ul, li, partialIndex);
                    inDom.set(true);
                  }
                } else if (inDom()) {
                  inDom.set(false);
                  ul.removeChild(li);
                }
                break;
              }
            }
          });
        }
        createDomEffect();

        const memo = createMemo(() => (inDom() ? 1 : 0));
        listForSum.push(memo);
      }
    });

    return createMemo(() => listForSum.reduce((acc, s) => acc + s(), 0));
  }

  const tree = window.document.createElement("div");
  tree.classList.add("tree");
  foldersFrame.append(tree);
  const allCount = processPartialTree(partialTree, tree);

  function checkUniqueIds() {
    if (presetsIds.length !== new Set(presetsIds).size) {
      /** @type {Map<string, number>} */
      const m = new Map();

      presetsIds.forEach((id) => {
        m.set(id, (m.get(id) || 0) + 1);
      });

      console.log(
        [...m.entries()]
          .filter(([_, value]) => value > 1)
          .map(([key, _]) => key)
      );

      throw Error("ID duplicate");
    }
  }
  checkUniqueIds();

  function createCountersDomUpdateEffect() {
    foldersFilterAllCount.innerHTML = allCount().toLocaleString();
    createEffect(() => {
      foldersFilterFavoritesCount.innerHTML = favoritesCount().toLocaleString();
    });
    createEffect(() => {
      foldersFilterNewCount.innerHTML = newCount().toLocaleString();
    });
  }
  createCountersDomUpdateEffect();

  function initFilters() {
    const filterAllInput = /** @type {HTMLInputElement} */ (
      getElementById("folders-filter-all")
    );
    const filterFavoritesInput = /** @type {HTMLInputElement} */ (
      getElementById("folders-filter-favorites")
    );
    const filterNewInput = /** @type {HTMLInputElement} */ (
      getElementById("folders-filter-new")
    );

    filterAllInput.addEventListener("change", () => {
      filter.set("all");
    });
    filterFavoritesInput.addEventListener("change", () => {
      filter.set("favorites");
    });
    filterNewInput.addEventListener("change", () => {
      filter.set("new");
    });

    createEffect(() => {
      const f = filter();
      localStorage.setItem(foldersFilterLocalStorageKey, f);
      switch (f) {
        case "all": {
          filterAllInput.checked = true;
          break;
        }
        case "favorites": {
          filterFavoritesInput.checked = true;
          break;
        }
        case "new": {
          filterNewInput.checked = true;
          break;
        }
      }
    });
  }
  initFilters();

  function initCloseAllButton() {
    getElementById("button-close-all-folders").addEventListener("click", () => {
      detailsList.forEach((details) => (details.open = false));
    });
  }
  initCloseAllButton();

  function goToSelected() {
    filter.set("all");

    if (!selected()) throw "Selected should be set by now";
    const selectedId = selected().id;

    selected().path.forEach(({ id }) => {
      const summary = getElementById(id);
      const details = /** @type {HTMLDetailsElement | undefined} */ (
        summary.parentElement
      );
      if (!details) throw "Parent details should exist";
      if (!details.open) {
        summary.click();
      }
    });

    setTimeout(() => {
      getElementById(`${selectedId}-folders-selector`).scrollIntoView({
        behavior: "instant",
        block: "center",
      });
    }, 0);
  }

  if (firstTime) {
    goToSelected();
  }

  function createUpdateSelectedHeaderEffect() {
    createEffect(() => {
      const preset = selected();
      presetTitle.innerHTML = preset.title;
      presetDescription.innerHTML = preset.serializedPath;
    });
  }
  createUpdateSelectedHeaderEffect();

  const LOCAL_STORAGE_TIME_RANGE_KEY = "chart-range";
  const URL_PARAMS_TIME_RANGE_FROM_KEY = "from";
  const URL_PARAMS_TIME_RANGE_TO_KEY = "to";
  const HEIGHT_CHUNK_SIZE = 10_000;

  runWhenIdle(() =>
    import("./libraries/lightweight-charts/script.js").then(
      ({
        createChart: createClassicChart,
        createChartEx: createCustomChart,
      }) => {
        /**
         * @param {Scale} scale
         * @returns {string}
         */
        function getVisibleTimeRangeLocalStorageKey(scale) {
          return `${LOCAL_STORAGE_TIME_RANGE_KEY}-${scale}`;
        }

        /**
         * @param {Scale} scale
         * @returns {TimeRange}
         */
        function getInitialVisibleTimeRange(scale) {
          const urlParams = new URLSearchParams(window.location.search);

          const urlFrom = urlParams.get(URL_PARAMS_TIME_RANGE_FROM_KEY);
          const urlTo = urlParams.get(URL_PARAMS_TIME_RANGE_TO_KEY);

          if (urlFrom && urlTo) {
            if (
              scale === "date" &&
              urlFrom.includes("-") &&
              urlTo.includes("-")
            ) {
              console.log({
                from: new Date(urlFrom).toJSON().split("T")[0],
                to: new Date(urlTo).toJSON().split("T")[0],
              });
              return {
                from: new Date(urlFrom).toJSON().split("T")[0],
                to: new Date(urlTo).toJSON().split("T")[0],
              };
            } else if (
              scale === "height" &&
              (!urlFrom.includes("-") || !urlTo.includes("-"))
            ) {
              console.log({
                from: Number(urlFrom),
                to: Number(urlTo),
              });
              return {
                from: Number(urlFrom),
                to: Number(urlTo),
              };
            }
          }

          function getSavedTimeRange() {
            return /** @type {TimeRange | null} */ (
              JSON.parse(
                localStorage.getItem(
                  getVisibleTimeRangeLocalStorageKey(scale)
                ) || "null"
              )
            );
          }

          const savedTimeRange = getSavedTimeRange();

          console.log(savedTimeRange);

          if (savedTimeRange) {
            return savedTimeRange;
          }

          function getDefaultTimeRange() {
            switch (scale) {
              case "date": {
                const defaultTo = new Date();
                const defaultFrom = new Date();
                defaultFrom.setDate(defaultFrom.getUTCDate() - 6 * 30);

                return {
                  from: defaultFrom.toJSON().split("T")[0],
                  to: defaultTo.toJSON().split("T")[0],
                };
              }
              case "height": {
                return {
                  from: 850_000,
                  to: 900_000,
                };
              }
            }
          }

          return getDefaultTimeRange();
        }

        /**
         * @param {IChartApi} chart
         */
        function setInitialVisibleTimeRange(chart) {
          const range = visibleTimeRange();

          if (range) {
            chart.timeScale().setVisibleRange(/** @type {any} */ (range));

            // On small screen it doesn't it might not set it  in time
            setTimeout(() => {
              try {
                chart.timeScale().setVisibleRange(/** @type {any} */ (range));
              } catch {}
            }, 50);
          }
        }

        const scale = createMemo(() => selected().scale);
        const activeDatasets = createSignal(
          /** @type {Set<ResourceDataset<any, any>>} */ (new Set()),
          {
            equals: false,
          }
        );
        const visibleTimeRange = createSignal(
          getInitialVisibleTimeRange(scale())
        );
        const visibleDatasetIds = createSignal(/** @type {number[]} */ ([]), {
          equals: false,
        });
        const lastVisibleDatasetIndex = createMemo(() => {
          const last = visibleDatasetIds().at(-1);
          return last !== undefined ? chunkIdToIndex(scale(), last) : undefined;
        });
        const priceSeriesType = createSignal(
          /** @type {PriceSeriesType} */ ("Candlestick")
        );

        function updateVisibleDatasetIds() {
          /** @type {number[]} */
          let ids = [];

          const today = new Date();
          const { from: rawFrom, to: rawTo } = visibleTimeRange();

          if (typeof rawFrom === "string" && typeof rawTo === "string") {
            const from = new Date(rawFrom).getUTCFullYear();
            const to = new Date(rawTo).getUTCFullYear();

            ids = Array.from(
              { length: to - from + 1 },
              (_, i) => i + from
            ).filter((year) => year >= 2009 && year <= today.getUTCFullYear());
          } else {
            const from = Math.floor(Number(rawFrom) / HEIGHT_CHUNK_SIZE);
            const to = Math.floor(Number(rawTo) / HEIGHT_CHUNK_SIZE);

            const length = to - from + 1;

            ids = Array.from(
              { length },
              (_, i) => (from + i) * HEIGHT_CHUNK_SIZE
            );
          }

          const old = visibleDatasetIds();

          if (
            old.length !== ids.length ||
            old.at(0) !== ids.at(0) ||
            old.at(-1) !== ids.at(-1)
          ) {
            console.log("range:", ids);

            visibleDatasetIds.set(ids);
          }
        }
        updateVisibleDatasetIds();

        const debouncedUpdateVisibleDatasetIds = debounce(
          updateVisibleDatasetIds,
          100
        );

        function saveVisibleRange() {
          const range = visibleTimeRange();

          urlParamsHelpers.write(
            URL_PARAMS_TIME_RANGE_FROM_KEY,
            String(range.from)
          );

          urlParamsHelpers.write(
            URL_PARAMS_TIME_RANGE_TO_KEY,
            String(range.to)
          );

          localStorage.setItem(
            getVisibleTimeRangeLocalStorageKey(scale()),
            JSON.stringify(range)
          );
        }
        const debouncedSaveVisibleRange = debounce(saveVisibleRange, 250);

        function createFetchChunksOfVisibleDatasetsEffect() {
          createEffect(() => {
            const ids = visibleDatasetIds();
            const datasets = Array.from(activeDatasets());

            if (ids.length === 0 || datasets.length === 0) return;

            untrack(() => {
              console.log(ids, datasets);
              for (let i = 0; i < ids.length; i++) {
                const id = ids[i];
                for (let j = 0; j < datasets.length; j++) {
                  datasets[j].fetch(id);
                }
              }
            });
          });
        }
        createFetchChunksOfVisibleDatasetsEffect();

        /** @param {number} value  */
        function numberToShortUSLocale(value) {
          const absoluteValue = Math.abs(value);

          // value = absoluteValue;

          if (isNaN(value)) {
            return "";
            // } else if (value === 0) {
            //   return "0";
          } else if (absoluteValue < 10) {
            return numberToUSLocale(value, 3);
          } else if (absoluteValue < 100) {
            return numberToUSLocale(value, 2);
          } else if (absoluteValue < 1_000) {
            return numberToUSLocale(value, 1);
          } else if (absoluteValue < 100_000) {
            return numberToUSLocale(value, 0);
          } else if (absoluteValue < 1_000_000) {
            return `${numberToUSLocale(value / 1_000, 1)}K`;
          } else if (absoluteValue >= 1_000_000_000_000_000_000) {
            return "Inf.";
          }

          const log = Math.floor(Math.log10(absoluteValue) - 6);

          const suffices = ["M", "B", "T", "Q"];
          const letterIndex = Math.floor(log / 3);
          const letter = suffices[letterIndex];

          const modulused = log % 3;

          if (modulused === 0) {
            return `${numberToUSLocale(
              value / (1_000_000 * 1_000 ** letterIndex),
              3
            )}${letter}`;
          } else if (modulused === 1) {
            return `${numberToUSLocale(
              value / (1_000_000 * 1_000 ** letterIndex),
              2
            )}${letter}`;
          } else {
            return `${numberToUSLocale(
              value / (1_000_000 * 1_000 ** letterIndex),
              1
            )}${letter}`;
          }
        }

        /**
         * @param {number} value
         * @param {number} [digits]
         * @param {Intl.NumberFormatOptions} [options]
         */
        function numberToUSLocale(value, digits, options) {
          return value.toLocaleString("en-us", {
            ...options,
            minimumFractionDigits: digits,
            maximumFractionDigits: digits,
          });
        }

        /**
         * @class
         * @implements {IHorzScaleBehavior<number>}
         */
        class HorzScaleBehaviorHeight {
          options() {
            return /** @type {any} */ (undefined);
          }
          setOptions() {}
          preprocessData() {}
          updateFormatter() {}

          createConverterToInternalObj() {
            /** @type {(p: any) => any} */
            return (price) => price;
          }

          /** @param {any} item  */
          key(item) {
            return item;
          }

          /** @param {any} item  */
          cacheKey(item) {
            return item;
          }

          /** @param {any} item  */
          convertHorzItemToInternal(item) {
            return item;
          }

          /** @param {any} item  */
          formatHorzItem(item) {
            return item;
          }

          /** @param {any} tickMark  */
          formatTickmark(tickMark) {
            return tickMark.time.toLocaleString("en-us");
          }

          /** @param {any} tickMarks  */
          maxTickMarkWeight(tickMarks) {
            return tickMarks.reduce(this.getMarkWithGreaterWeight, tickMarks[0])
              .weight;
          }

          /**
           * @param {any} sortedTimePoints
           * @param {number} startIndex
           */
          fillWeightsForPoints(sortedTimePoints, startIndex) {
            for (
              let index = startIndex;
              index < sortedTimePoints.length;
              ++index
            ) {
              sortedTimePoints[index].timeWeight = this.computeHeightWeight(
                sortedTimePoints[index].time
              );
            }
          }

          /**
           * @param {any} a
           * @param {any} b
           */
          getMarkWithGreaterWeight(a, b) {
            return a.weight > b.weight ? a : b;
          }

          /** @param {number} value  */
          computeHeightWeight(value) {
            // if (value === Math.ceil(value / 1000000) * 1000000) {
            //   return 12;
            // }
            if (value === Math.ceil(value / 100000) * 100000) {
              return 11;
            }
            if (value === Math.ceil(value / 10000) * 10000) {
              return 10;
            }
            if (value === Math.ceil(value / 1000) * 1000) {
              return 9;
            }
            if (value === Math.ceil(value / 100) * 100) {
              return 8;
            }
            if (value === Math.ceil(value / 50) * 50) {
              return 7;
            }
            if (value === Math.ceil(value / 25) * 25) {
              return 6;
            }
            if (value === Math.ceil(value / 10) * 10) {
              return 5;
            }
            if (value === Math.ceil(value / 5) * 5) {
              return 4;
            }
            if (value === Math.ceil(value)) {
              return 3;
            }
            if (value * 2 === Math.ceil(value * 2)) {
              return 1;
            }

            return 0;
          }
        }

        /** @arg {{scale: Scale, element: HTMLElement}} args */
        function createChart({ scale, element }) {
          /** @satisfies {DeepPartial<ChartOptions>} */
          const options = {
            autoSize: true,
            layout: {
              fontFamily: "Satoshi Chart",
              // fontSize: 13,
              background: { color: "transparent" },
              attributionLogo: false,
            },
            grid: {
              vertLines: { visible: false },
              horzLines: { visible: false },
            },
            timeScale: {
              minBarSpacing: 0.05,
              shiftVisibleRangeOnNewBar: false,
              allowShiftVisibleRangeOnWhitespaceReplacement: false,
            },
            handleScale: {
              axisDoubleClickReset: {
                time: false,
              },
            },
            crosshair: {
              mode: 0,
            },
            localization: {
              priceFormatter: numberToShortUSLocale,
              locale: "en-us",
            },
          };

          /** @type {IChartApi} */
          let chart;

          if (scale === "date") {
            chart = createClassicChart(element, options);
          } else {
            const horzScaleBehavior = new HorzScaleBehaviorHeight();
            // @ts-ignore
            chart = createCustomChart(element, horzScaleBehavior, options);
          }

          chart.priceScale("right").applyOptions({
            scaleMargins: {
              top: 0.075,
              bottom: 0.05,
            },
            minimumWidth: 78,
          });

          createEffect(() => {
            const { white, darkWhite } = colors;

            const color = white();

            chart.applyOptions({
              layout: {
                textColor: darkWhite(),
              },
              rightPriceScale: {
                borderVisible: false,
              },
              timeScale: {
                borderVisible: false,
              },
              crosshair: {
                horzLine: {
                  color: color,
                  labelBackgroundColor: color,
                },
                vertLine: {
                  color: color,
                  labelBackgroundColor: color,
                },
              },
            });
          });

          return chart;
        }

        function resetChartListElement() {
          while (
            chartListElement.lastElementChild?.classList.contains(
              "chart-wrapper"
            )
          ) {
            chartListElement.lastElementChild?.remove();
          }
        }

        function initWhitespace() {
          const whitespaceStartDate = new Date("1970-01-01");
          const whitespaceStartDateYear = whitespaceStartDate.getUTCFullYear();
          const whitespaceStartDateMonth = whitespaceStartDate.getUTCMonth();
          const whitespaceStartDateDate = whitespaceStartDate.getUTCDate();
          const whitespaceEndDate = new Date("2141-01-01");
          const whitespaceDateDataset =
            /** @type {(WhitespaceData | SingleValueData)[]} */ (
              new Array(
                getNumberOfDaysBetweenTwoDates(
                  whitespaceStartDate,
                  whitespaceEndDate
                )
              )
            );
          // Hack to be able to scroll freely
          // Setting them all to NaN is much slower
          for (let i = 0; i < whitespaceDateDataset.length; i++) {
            const date = new Date(
              whitespaceStartDateYear,
              whitespaceStartDateMonth,
              whitespaceStartDateDate + i
            );

            const time = dateToString(date);

            if (i === whitespaceDateDataset.length - 1) {
              whitespaceDateDataset[i] = {
                time,
                value: NaN,
              };
            } else {
              whitespaceDateDataset[i] = {
                time,
              };
            }
          }

          const heightStart = -50_000;
          const whitespaceHeightDataset = /** @type {WhitespaceData[]} */ (
            new Array((new Date().getUTCFullYear() - 2009 + 1) * 60_000)
          );
          for (let i = 0; i < whitespaceHeightDataset.length; i++) {
            const height = heightStart + i;

            whitespaceHeightDataset[i] = {
              time: /** @type {Time} */ (height),
            };
          }

          /**
           * @param {IChartApi} chart
           * @param {Scale} scale
           * @returns {ISeriesApi<'Line'>}
           */
          function setWhitespace(chart, scale) {
            const whitespace = chart.addLineSeries();

            if (scale === "date") {
              whitespace.setData(whitespaceDateDataset);
            } else {
              whitespace.setData(whitespaceHeightDataset);

              const time = whitespaceHeightDataset.length;
              whitespace.update({
                time: /** @type {Time} */ (time),
                value: NaN,
              });
            }

            return whitespace;
          }

          return { setWhitespace };
        }
        const { setWhitespace } = initWhitespace();

        /**
         * @param {HTMLElement} parent
         * @param {number} chartIndex
         * @param {Preset} preset
         */
        function createChartDiv(parent, chartIndex, preset) {
          const chartWrapper = window.document.createElement("div");
          chartWrapper.classList.add("chart-wrapper");
          parent.append(chartWrapper);

          const chartDiv = window.document.createElement("div");
          chartDiv.classList.add("chart-div");
          chartWrapper.append(chartDiv);

          const unitField = window.document.createElement("fieldset");
          const unitName = window.document.createElement("span");
          unitName.innerHTML = chartIndex
            ? preset.unit
            : /** @satisfies {Unit} */ ("US Dollars");
          unitField.append(unitName);
          const unitDash = window.document.createElement("span");
          unitDash.innerHTML = "—";

          chartWrapper.append(chartDiv);

          return chartDiv;
        }

        /**
         * @param {IChartApi} chart
         */
        function subscribeVisibleTimeRangeChange(chart) {
          chart.timeScale().subscribeVisibleTimeRangeChange((range) => {
            if (!range) return;

            visibleTimeRange.set(range);

            debouncedUpdateVisibleDatasetIds();

            debouncedSaveVisibleRange();
          });
        }

        /**
         * @param {{ chart: IChartApi; visibleLogicalRange?: LogicalRange; visibleTimeRange?: TimeRange }} args
         */
        function updateVisiblePriceSeriesType({
          chart,
          visibleLogicalRange,
          visibleTimeRange,
        }) {
          try {
            const width = chart.timeScale().width();

            /** @type {number} */
            let ratio;

            if (visibleLogicalRange) {
              ratio =
                (visibleLogicalRange.to - visibleLogicalRange.from) / width;
            } else if (visibleTimeRange) {
              if (scale() === "date") {
                const to = /** @type {Time} */ (visibleTimeRange.to);
                const from = /** @type {Time} */ (visibleTimeRange.from);

                ratio =
                  getNumberOfDaysBetweenTwoDates(
                    dateFromTime(from),
                    dateFromTime(to)
                  ) / width;
              } else {
                const to = /** @type {number} */ (visibleTimeRange.to);
                const from = /** @type {number} */ (visibleTimeRange.from);

                ratio = (to - from) / width;
              }
            } else {
              throw Error();
            }

            if (ratio <= 0.5) {
              priceSeriesType.set("Candlestick");
            } else {
              priceSeriesType.set("Line");
            }
          } catch {}
        }

        /** @param {Time} time  */
        function dateFromTime(time) {
          return typeof time === "string"
            ? new Date(time)
            : // @ts-ignore
              new Date(time.year, time.month, time.day);
        }

        const debouncedUpdateVisiblePriceSeriesType = debounce(
          updateVisiblePriceSeriesType,
          50
        );

        /**
         * @param {Scale} scale
         * @param {number} id
         */
        function chunkIdToIndex(scale, id) {
          return scale === "date"
            ? id - 2009
            : Math.floor(id / HEIGHT_CHUNK_SIZE);
        }

        function createDatasets() {
          /** @type {Map<DatePath, ResourceDataset<"date">>} */
          const date = new Map();
          /** @type {Map<HeightPath, ResourceDataset<"height">>} */
          const height = new Map();

          const USE_LOCAL_URL = true;
          const LOCAL_URL = "/api";
          const WEB_URL = "https://kibo.money/api";
          const BACKUP_WEB_URL = "https://backup.kibo.money/api";

          const datasetsOwner = getOwner();

          /**
           * @template {Scale} S
           * @template {number | OHLC} [T=number]
           * @param {S} scale
           * @param {string} path
           */
          function createResourceDataset(scale, path) {
            return /** @type {ResourceDataset<S, T>} */ (
              runWithOwner(datasetsOwner, () => {
                /** @typedef {DatasetValue<T extends number ? SingleValueData : CandlestickData>} Value */

                const baseURL = `${
                  USE_LOCAL_URL && location.hostname === "localhost"
                    ? LOCAL_URL
                    : WEB_URL
                }/${path}`;

                const backupURL = `${
                  USE_LOCAL_URL && location.hostname === "localhost"
                    ? LOCAL_URL
                    : BACKUP_WEB_URL
                }/${path}`;

                const fetchedJSONs = new Array(
                  (new Date().getFullYear() -
                    new Date("2009-01-01").getFullYear() +
                    2) *
                    (scale === "date" ? 1 : 6)
                )
                  .fill(null)
                  .map(() => {
                    const json = createSignal(
                      /** @type {FetchedJSON<S, T> | null} */ (null)
                    );

                    /** @type {FetchedResult<S, T>} */
                    const fetchedResult = {
                      at: null,
                      json,
                      loading: false,
                      vec: createMemo(() => {
                        const map = json()?.dataset.map;

                        if (!map) {
                          return null;
                        }

                        const chunkId = json()?.chunk.id;

                        if (chunkId === undefined) {
                          throw `ChunkId ${chunkId} is undefined`;
                        }

                        if (Array.isArray(map)) {
                          const values = new Array(map.length);

                          for (let i = 0; i < map.length; i++) {
                            const value = map[i];

                            values[i] = /** @type {Value} */ ({
                              time: /** @type {Time} */ (chunkId + i),
                              ...(typeof value !== "number" && value !== null
                                ? {
                                    .../** @type {OHLC} */ (value),
                                    value: value.close,
                                  }
                                : {
                                    value:
                                      value === null
                                        ? NaN
                                        : /** @type {number} */ (value),
                                  }),
                            });
                          }

                          return values;
                        } else {
                          return Object.entries(map).map(
                            ([date, value]) =>
                              /** @type {Value} */ ({
                                time: date,
                                ...(typeof value !== "number" && value !== null
                                  ? {
                                      .../** @type {OHLC} */ (value),
                                      value: value.close,
                                    }
                                  : {
                                      value:
                                        value === null
                                          ? NaN
                                          : /** @type {number} */ (value),
                                    }),
                              })
                          );
                        }
                      }),
                    };

                    return fetchedResult;
                  });

                /**
                 * @param {number} id
                 */
                async function _fetch(id) {
                  const index = chunkIdToIndex(scale, id);

                  if (
                    index < 0 ||
                    (scale === "date" && id > new Date().getUTCFullYear()) ||
                    (scale === "height" &&
                      id > 165 * 365 * (new Date().getUTCFullYear() - 2009))
                  ) {
                    return;
                  }

                  const fetched = fetchedJSONs.at(index);

                  if (scale === "height" && index > 0) {
                    const length = fetchedJSONs.at(index - 1)?.vec()?.length;

                    if (length !== undefined && length < HEIGHT_CHUNK_SIZE) {
                      return;
                    }
                  }

                  if (!fetched || fetched.loading) {
                    return;
                  } else if (fetched.at) {
                    const diff = new Date().getTime() - fetched.at.getTime();

                    if (
                      diff < ONE_MINUTE_IN_MS ||
                      (index < fetchedJSONs.findLastIndex((json) => json.at) &&
                        diff < ONE_HOUR_IN_MS)
                    ) {
                      return;
                    }
                  }

                  fetched.loading = true;

                  /** @type {Cache | undefined} */
                  let cache;

                  const urlWithQuery = `${baseURL}?chunk=${id}`;
                  const backupUrlWithQuery = `${backupURL}?chunk=${id}`;

                  if (!fetched.json()) {
                    try {
                      cache = await caches.open("resources");

                      const cachedResponse = await cache.match(urlWithQuery);

                      if (cachedResponse) {
                        /** @type {FetchedJSON<S, T> | null} */
                        const json = await convertResponseToJSON(
                          cachedResponse
                        );

                        if (json) {
                          console.log(`cache: ${path}?chunk=${id}`);

                          fetched.json.set(() => json);
                        }
                      }
                    } catch {}
                  }

                  if (!navigator.onLine) {
                    fetched.loading = false;
                    return;
                  }

                  /** @type {Response | undefined} */
                  let fetchedResponse;

                  /** @type {RequestInit} */
                  const fetchConfig = {
                    signal: AbortSignal.timeout(5000),
                  };

                  try {
                    fetchedResponse = await fetch(urlWithQuery, fetchConfig);

                    if (!fetchedResponse.ok) {
                      throw Error;
                    }
                  } catch {
                    try {
                      fetchedResponse = await fetch(
                        backupUrlWithQuery,
                        fetchConfig
                      );
                    } catch {
                      fetched.loading = false;
                      return;
                    }

                    if (!fetchedResponse || !fetchedResponse.ok) {
                      fetched.loading = false;
                      return;
                    }
                  }

                  const clonedResponse = fetchedResponse.clone();

                  /** @type {FetchedJSON<S, T> | null} */
                  const json = await convertResponseToJSON(fetchedResponse);

                  if (!json) {
                    fetched.loading = false;
                    return;
                  }

                  console.log(`fetch: ${path}?chunk=${id}`);

                  const previousMap = fetched.json()?.dataset;
                  const newMap = json.dataset.map;

                  const previousLength = Object.keys(previousMap || []).length;
                  const newLength = Object.keys(newMap).length;

                  if (!newLength) {
                    fetched.loading = false;
                    return;
                  }

                  if (previousLength && previousLength === newLength) {
                    const previousLastValue = Object.values(
                      previousMap || []
                    ).at(-1);
                    const newLastValue = Object.values(newMap).at(-1);

                    if (newLastValue === null && previousLastValue === null) {
                      fetched.at = new Date();
                      fetched.loading = false;
                      return;
                    } else if (typeof newLastValue === "number") {
                      if (previousLastValue === newLastValue) {
                        fetched.at = new Date();
                        fetched.loading = false;
                        return;
                      }
                    } else {
                      const previousLastOHLC = /** @type {OHLC} */ (
                        previousLastValue
                      );
                      const newLastOHLC = /** @type {OHLC} */ (newLastValue);

                      if (
                        previousLastOHLC.open === newLastOHLC.open &&
                        previousLastOHLC.high === newLastOHLC.high &&
                        previousLastOHLC.low === newLastOHLC.low &&
                        previousLastOHLC.close === newLastOHLC.close
                      ) {
                        fetched.loading = false;
                        fetched.at = new Date();
                        return;
                      }
                    }
                  }

                  fetched.json.set(() => json);

                  runWhenIdle(async function () {
                    try {
                      await cache?.put(urlWithQuery, clonedResponse);
                    } catch (_) {}
                  });

                  fetched.at = new Date();
                  fetched.loading = false;
                }

                /** @type {ResourceDataset<S, T>} */
                const resource = {
                  scale,
                  url: baseURL,
                  fetch: _fetch,
                  fetchedJSONs,
                  // drop() {
                  //   dispose();
                  //   fetchedJSONs.forEach((fetched) => {
                  //     fetched.at = null;
                  //     fetched.json.set(null);
                  //   });
                  // },
                };

                return resource;
              })
            );
          }

          /**
           * @template {Scale} S
           * @template {number | OHLC} T
           * @param {Response} response
           */
          async function convertResponseToJSON(response) {
            try {
              return /** @type {FetchedJSON<S, T>} */ (await response.json());
            } catch (_) {
              return null;
            }
          }

          /**
           * @template {Scale} S
           * @param {S} scale
           * @param {DatasetPath<S>} path
           * @returns {ResourceDataset<S>}
           */
          function getOrImport(scale, path) {
            if (scale === "date") {
              const found = date.get(/** @type {DatePath} */ (path));
              if (found) return /** @type {ResourceDataset<S>} */ (found);
            } else {
              const found = height.get(/** @type {HeightPath} */ (path));
              if (found) return /** @type {ResourceDataset<S>} */ (found);
            }

            /** @type {ResourceDataset<S, any>} */
            let dataset;

            if (path === `/${scale}-to-price`) {
              /** @type {ResourceDataset<S, OHLC>} */
              dataset = createResourceDataset(scale, path);
            } else {
              /** @type {ResourceDataset<S, number>} */
              dataset = createResourceDataset(scale, path);
            }

            if (scale === "date") {
              date.set(
                /** @type {DatePath} */ (path),
                /** @type {any} */ (dataset)
              );
            } else {
              height.set(
                /** @type {HeightPath} */ (path),
                /** @type {any} */ (dataset)
              );
            }

            return dataset;
          }

          return {
            getOrImport,
          };
        }

        const datasets = createDatasets();

        /** @type {DeepPartial<SeriesOptionsCommon>} */
        const defaultSeriesOptions = {
          // @ts-ignore
          lineWidth: 1.5,
          priceLineVisible: false,
          baseLineVisible: false,
          baseLineColor: "",
        };

        /**
         * @param {SpecificSeriesBlueprintWithChart<BaselineSpecificSeriesBlueprint>} args
         */
        function createBaseLineSeries({ chart, color, options, owner }) {
          const topLineColor = color || colors.profit;
          const bottomLineColor = color || colors.loss;

          function computeColors() {
            return {
              topLineColor: topLineColor(),
              bottomLineColor: bottomLineColor(),
            };
          }

          const transparent = "transparent";

          /** @type {DeepPartial<BaselineStyleOptions & SeriesOptionsCommon>} */
          const seriesOptions = {
            priceScaleId: "right",
            ...defaultSeriesOptions,
            ...options,
            topFillColor1: transparent,
            topFillColor2: transparent,
            bottomFillColor1: transparent,
            bottomFillColor2: transparent,
            ...computeColors(),
          };

          const series = chart.addBaselineSeries(seriesOptions);

          runWithOwner(owner, () => {
            createEffect(() => {
              series.applyOptions(computeColors());
            });
          });

          return series;
        }

        /**
         * @param {SpecificSeriesBlueprintWithChart<CandlestickSpecificSeriesBlueprint>} args
         */
        function createCandlesticksSeries({ chart, options, owner }) {
          function computeColors() {
            const upColor = colors.profit();
            const downColor = colors.loss();

            return {
              upColor,
              wickUpColor: upColor,
              downColor,
              wickDownColor: downColor,
            };
          }

          const candlestickSeries = chart.addCandlestickSeries({
            baseLineVisible: false,
            borderVisible: false,
            priceLineVisible: false,
            baseLineColor: "",
            borderColor: "",
            borderDownColor: "",
            borderUpColor: "",
            ...options,
            ...computeColors(),
          });

          runWithOwner(owner, () => {
            createEffect(() => {
              candlestickSeries.applyOptions(computeColors());
            });
          });

          return candlestickSeries;
        }

        /**
         * @param {SpecificSeriesBlueprintWithChart<LineSpecificSeriesBlueprint>} args
         */
        function createLineSeries({ chart, color, options, owner }) {
          function computeColors() {
            return {
              color: color(),
            };
          }

          const series = chart.addLineSeries({
            ...defaultSeriesOptions,
            ...options,
            ...computeColors(),
          });

          runWithOwner(owner, () => {
            createEffect(() => {
              series.applyOptions(computeColors());
            });
          });

          return series;
        }

        const hoveredLegend = createSignal(
          /** @type {{label: HTMLLabelElement, series: Series} | undefined} */ (
            undefined
          )
        );
        const notHoveredLegendTransparency = "66";
        /**
         * @param {Object} args
         * @param {Series} args.series
         * @param {Accessor<boolean>} [args.disabled]
         * @param {string} [args.name]
         */
        function createLegend({ series, disabled, name }) {
          const div = window.document.createElement("div");
          createEffect(() => {
            div.hidden = disabled?.() ? true : false;
          });
          legendElement.prepend(div);

          const { input, label, spanMain } = createLabel({
            inputId: `legend-${series.title}`,
            inputName: `selected-${series.title}${name}`,
            inputValue: "value",
            labelTitle: "Click to toggle",
            name: series.title,
            onClick: () => {
              input.checked = !input.checked;
              series.active.set(input.checked);
            },
          });
          div.append(label);
          label.addEventListener("mouseover", () => {
            const hovered = hoveredLegend();

            if (!hovered || hovered.label !== label) {
              hoveredLegend.set({ label, series });
            }
          });
          label.addEventListener("mouseleave", () => {
            hoveredLegend.set(undefined);
          });

          createEffect(() => {
            input.checked = series.active();
          });

          function shouldHighlight() {
            const hovered = hoveredLegend();
            return (
              !hovered ||
              (hovered.label === label && hovered.series.active()) ||
              (hovered.label !== label && !hovered.series.active())
            );
          }

          const spanColors = window.document.createElement("span");
          spanColors.classList.add("colors");
          spanMain.prepend(spanColors);
          const colors = Array.isArray(series.color)
            ? series.color
            : [series.color];
          colors.forEach((color) => {
            const spanColor = window.document.createElement("span");
            spanColors.append(spanColor);

            createEffect(() => {
              const c = color();
              if (shouldHighlight()) {
                spanColor.style.backgroundColor = c;
              } else {
                spanColor.style.backgroundColor = `${c}${notHoveredLegendTransparency}`;
              }
            });
          });

          function createHoverEffect() {
            const initialColors = /** @type {Record<string, any>} */ ({});
            const darkenedColors = /** @type {Record<string, any>} */ ({});

            // @ts-ignore
            createEffect((previouslyHovered) => {
              const hovered = hoveredLegend();

              if (!hovered && !previouslyHovered) return hovered;

              const ids = visibleDatasetIds();

              for (let i = 0; i < ids.length; i++) {
                const chunkId = ids[i];
                const chunkIndex = chunkIdToIndex(scale(), chunkId);
                const chunk = series.chunks[chunkIndex]?.();

                if (!chunk) return;

                if (hovered) {
                  const seriesOptions = chunk.options();
                  if (!seriesOptions) return;

                  initialColors[i] = {};
                  darkenedColors[i] = {};

                  Object.entries(seriesOptions).forEach(([k, v]) => {
                    if (k.toLowerCase().includes("color") && v) {
                      if (typeof v === "string" && !v.startsWith("#")) {
                        return;
                      }

                      v = /** @type {string} */ (v).substring(0, 7);
                      initialColors[i][k] = v;
                      darkenedColors[i][
                        k
                      ] = `${v}${notHoveredLegendTransparency}`;
                    } else if (k === "lastValueVisible" && v) {
                      initialColors[i][k] = true;
                      darkenedColors[i][k] = false;
                    }
                  });
                }

                if (shouldHighlight()) {
                  chunk.applyOptions(initialColors[i]);
                } else {
                  chunk.applyOptions(darkenedColors[i]);
                }
              }

              return hovered;
            }, undefined);
          }
          createHoverEffect();

          const anchor = window.document.createElement("a");
          anchor.href = series.dataset.url;
          anchor.innerHTML = `<svg viewBox="0 0 16 16"><path d="M8.75 2.75a.75.75 0 0 0-1.5 0v5.69L5.03 6.22a.75.75 0 0 0-1.06 1.06l3.5 3.5a.75.75 0 0 0 1.06 0l3.5-3.5a.75.75 0 0 0-1.06-1.06L8.75 8.44V2.75Z" /><path d="M3.5 9.75a.75.75 0 0 0-1.5 0v1.5A2.75 2.75 0 0 0 4.75 14h6.5A2.75 2.75 0 0 0 14 11.25v-1.5a.75.75 0 0 0-1.5 0v1.5c0 .69-.56 1.25-1.25 1.25h-6.5c-.69 0-1.25-.56-1.25-1.25v-1.5Z" /></svg>`;
          anchor.target = "_target";
          anchor.rel = "noopener noreferrer";
          div.append(anchor);
        }

        /**
         * @template {Scale} S
         * @param {Object} args
         * @param {ResourceDataset<S>} args.dataset
         * @param {SeriesBlueprint} args.seriesBlueprint
         * @param {Preset} args.preset
         * @param {IChartApi} args.chart
         * @param {number} args.index
         * @param {Series[]} args.chartSeries
         * @param {Accessor<number | undefined>} args.lastVisibleDatasetIndex
         * @param {VoidFunction} args.setMinMaxMarkersWhenIdle
         * @param {Accessor<boolean>} [args.disabled]
         */
        function createSeries({
          chart,
          preset,
          index: seriesIndex,
          disabled: _disabled,
          lastVisibleDatasetIndex,
          setMinMaxMarkersWhenIdle,
          dataset,
          seriesBlueprint,
          chartSeries,
        }) {
          const { title, color, defaultActive, type, options } =
            seriesBlueprint;

          /** @type {Signal<ISeriesApi<SeriesType> | undefined>[]} */
          const chunks = new Array(dataset.fetchedJSONs.length);

          const id = stringToId(title);
          const storageId = presetAndSeriesToLocalStorageKey(
            preset,
            seriesBlueprint
          );

          const active = createSignal(
            urlParamsHelpers.readBool(id) ??
              localeStorageHelpers.readBool(storageId) ??
              defaultActive ??
              true
          );

          const disabled = createMemo(_disabled || (() => false));

          const visible = createMemo(() => active() && !disabled());

          createEffect(() => {
            if (disabled()) {
              return;
            }

            const a = active();

            if (a !== (defaultActive || true)) {
              urlParamsHelpers.write(id, a);
              localeStorageHelpers.write(storageId, a);
            } else {
              urlParamsHelpers.remove(id);
              localeStorageHelpers.remove(storageId);
            }
          });

          /** @type {Series} */
          const series = {
            active,
            chunks,
            color: color || [colors.profit, colors.loss],
            dataset,
            disabled,
            id,
            title,
            visible,
          };

          chartSeries.push(series);

          const owner = getOwner();

          dataset.fetchedJSONs.forEach((json, index) => {
            const chunk = createSignal(
              /** @type {ISeriesApi<SeriesType> | undefined} */ (undefined)
            );

            chunks[index] = chunk;

            createEffect(() => {
              const values = json.vec();

              if (!values) return;

              if (seriesIndex > 0) {
                const previousSeriesChunk = chartSeries.at(seriesIndex - 1)
                  ?.chunks[index];
                const isPreviousSeriesOnChart = previousSeriesChunk?.();
                if (!isPreviousSeriesOnChart) {
                  return;
                }
              }

              untrack(() => {
                let s = chunk();

                if (!s) {
                  switch (type) {
                    case "Baseline": {
                      s = createBaseLineSeries({
                        chart,
                        color,
                        options,
                        owner,
                      });
                      break;
                    }
                    case "Candlestick": {
                      s = createCandlesticksSeries({
                        chart,
                        options,
                        owner,
                      });
                      break;
                    }
                    // case "Histogram": {
                    //   s = createHistogramSeries({
                    //     chart,
                    //     options,
                    //   });
                    //   break;
                    // }
                    default:
                    case "Line": {
                      s = createLineSeries({
                        chart,
                        color,
                        options,
                        owner,
                      });
                      break;
                    }
                  }

                  // if (priceScaleOptions) {
                  //   s.priceScale().applyOptions(priceScaleOptions);
                  // }

                  chunk.set(s);
                }

                s.setData(values);

                setMinMaxMarkersWhenIdle();
              });
            });

            createEffect(() => {
              const _chunk = chunk();
              const currentVec = dataset.fetchedJSONs.at(index)?.vec();
              const nextVec = dataset.fetchedJSONs.at(index + 1)?.vec();

              if (_chunk && currentVec?.length && nextVec?.length) {
                _chunk.update(nextVec[0]);
              }
            });

            const isChunkLastVisible = createMemo(() => {
              const last = lastVisibleDatasetIndex();
              return last !== undefined && last === index;
            });

            createEffect(() => {
              chunk()?.applyOptions({
                lastValueVisible: series.visible() && isChunkLastVisible(),
              });
            });

            const shouldChunkBeVisible = createMemo(() => {
              if (visibleDatasetIds().length) {
                const start = chunkIdToIndex(
                  scale(),
                  /** @type {number} */ (visibleDatasetIds().at(0))
                );
                const end = chunkIdToIndex(
                  scale(),
                  /** @type {number} */ (visibleDatasetIds().at(-1))
                );

                if (index >= start && index <= end) {
                  return true;
                }
              }

              return false;
            });

            let wasChunkVisible = false;
            const chunkVisible = createMemo(() => {
              if (series.disabled()) {
                wasChunkVisible = false;
              } else {
                wasChunkVisible = wasChunkVisible || shouldChunkBeVisible();
              }
              return wasChunkVisible;
            });

            createEffect(() => {
              const visible = series.visible() && chunkVisible();
              chunk()?.applyOptions({
                visible,
              });
            });
          });

          createLegend({ series, disabled, name: type });

          return series;
        }

        /**
         * @param {Object} args
         * @param {PriceSeriesType} args.type
         * @param {VoidFunction} args.setMinMaxMarkersWhenIdle
         * @param {Preset} args.preset
         * @param {IChartApi} args.chart
         * @param {Series[]} args.chartSeries
         * @param {Accessor<number | undefined>} args.lastVisibleDatasetIndex
         */
        function createPriceSeries({
          type,
          setMinMaxMarkersWhenIdle,
          preset,
          chart,
          chartSeries,
          lastVisibleDatasetIndex,
        }) {
          const s = scale();

          /** @type {AnyDatasetPath} */
          const datasetPath = `${s}-to-price`;

          const dataset = datasets.getOrImport(s, datasetPath);

          // Don't trigger reactivity by design
          activeDatasets().add(dataset);

          const title = "Price";

          /** @type {SeriesBlueprint} */
          let seriesBlueprint;

          if (type === "Candlestick") {
            seriesBlueprint = {
              datasetPath,
              title,
              type: "Candlestick",
            };
          } else {
            seriesBlueprint = {
              datasetPath,
              title,
              color: colors.white,
            };
          }

          const disabled = createMemo(() => priceSeriesType() !== type);

          const priceSeries = createSeries({
            seriesBlueprint,
            dataset,
            preset,
            index: -1,
            chart,
            chartSeries,
            lastVisibleDatasetIndex,
            disabled,
            setMinMaxMarkersWhenIdle,
          });

          // createEffect(() => {
          //   const latest = webSockets.liveKrakenCandle.latest();

          //   if (!latest) return;

          //   const index = chunkIdToIndex(s, latest.year);

          //   const series = priceSeries.seriesList.at(index)?.();

          //   series?.update(latest);
          // });

          return priceSeries;
        }

        function resetLegendElement() {
          legendElement.innerHTML = "";
        }

        /** @type {Array<(IChartApi & {whitespace: ISeriesApi<"Line">})>} */
        let charts = [];

        function applyPreset() {
          const preset = selected();
          const scale = preset.scale;
          visibleTimeRange.set(getInitialVisibleTimeRange(scale));

          activeDatasets.set((s) => {
            s.clear();
            return s;
          });

          const chartCount = 1 + (preset.bottom?.length ? 1 : 0);
          const blueprintCount =
            1 + (preset.top?.length || 0) + (preset.bottom?.length || 0);
          const chartsBlueprints = [preset.top || [], preset.bottom].flatMap(
            (list) => (list ? [list] : [])
          );

          resetLegendElement();
          resetChartListElement();

          /** @type {Series[]} */
          const allSeries = [];

          charts = chartsBlueprints.map((seriesBlueprints, chartIndex) => {
            const chartDiv = createChartDiv(
              chartListElement,
              chartIndex,
              preset
            );

            const chart =
              /** @type {IChartApi & {whitespace: ISeriesApi<"Line">}} */ (
                createChart({
                  scale,
                  element: chartDiv,
                })
              );
            chart.whitespace = setWhitespace(chart, scale);

            setInitialVisibleTimeRange(chart);

            /** @type {Series[]} */
            const chartSeries = [];

            function setMinMaxMarkers() {
              try {
                const { from, to } = visibleTimeRange();

                const dateFrom = new Date(String(from));
                const dateTo = new Date(String(to));

                /** @type {Marker | undefined} */
                let max = undefined;
                /** @type {Marker | undefined} */
                let min = undefined;

                const ids = visibleDatasetIds();

                for (let i = 0; i < chartSeries.length; i++) {
                  const { chunks, dataset } = chartSeries[i];

                  for (let j = 0; j < ids.length; j++) {
                    const id = ids[j];

                    const chunkIndex = chunkIdToIndex(scale, id);

                    const chunk = chunks.at(chunkIndex)?.();

                    if (!chunk || !chunk?.options().visible) continue;

                    chunk.setMarkers([]);

                    const isCandlestick = chunk.seriesType() === "Candlestick";

                    const vec = dataset.fetchedJSONs.at(chunkIndex)?.vec();

                    if (!vec) return;

                    for (let k = 0; k < vec.length; k++) {
                      const data = vec[k];

                      let number;

                      if (scale === "date") {
                        const date = dateFromTime(data.time);

                        number = date.getTime();

                        if (date <= dateFrom || date >= dateTo) {
                          continue;
                        }
                      } else {
                        const height = data.time;

                        number = /** @type {number} */ (height);

                        if (height <= from || height >= to) {
                          continue;
                        }
                      }

                      // @ts-ignore
                      const high = isCandlestick ? data["high"] : data.value;
                      // @ts-ignore
                      const low = isCandlestick ? data["low"] : data.value;

                      if (!max || high > max.value) {
                        max = {
                          weight: number,
                          time: data.time,
                          value: high,
                          seriesChunk: chunk,
                        };
                      }
                      if (!min || low < min.value) {
                        min = {
                          weight: number,
                          time: data.time,
                          value: low,
                          seriesChunk: chunk,
                        };
                      }
                    }
                  }
                }

                /** @type {(SeriesMarker<Time> & Weighted) | undefined} */
                let minMarker;
                /** @type {(SeriesMarker<Time> & Weighted) | undefined} */
                let maxMarker;

                if (min) {
                  minMarker = {
                    weight: min.weight,
                    time: min.time,
                    color: colors.white(),
                    position: "belowBar",
                    shape: "arrowUp",
                    size: 0,
                    text: numberToShortUSLocale(min.value),
                  };
                }

                if (max) {
                  maxMarker = {
                    weight: max.weight,
                    time: max.time,
                    color: colors.white(),
                    position: "aboveBar",
                    shape: "arrowDown",
                    size: 0,
                    text: numberToShortUSLocale(max.value),
                  };
                }

                if (
                  min &&
                  max &&
                  min.seriesChunk === max.seriesChunk &&
                  minMarker &&
                  maxMarker
                ) {
                  min.seriesChunk.setMarkers(
                    [minMarker, maxMarker].sort((a, b) => a.weight - b.weight)
                  );
                } else {
                  if (min && minMarker) {
                    min.seriesChunk.setMarkers([minMarker]);
                  }

                  if (max && maxMarker) {
                    max.seriesChunk.setMarkers([maxMarker]);
                  }
                }
              } catch (e) {}
            }

            const setMinMaxMarkersWhenIdle = () =>
              runWhenIdle(
                () => {
                  setMinMaxMarkers();
                },
                blueprintCount * 10 + scale === "date" ? 50 : 100
              );

            function createSetMinMaxMarkersWhenIdleEffect() {
              createEffect(() => {
                visibleTimeRange();
                dark();
                untrack(setMinMaxMarkersWhenIdle);
              });
            }
            createSetMinMaxMarkersWhenIdleEffect();

            if (!chartIndex) {
              subscribeVisibleTimeRangeChange(chart);

              updateVisiblePriceSeriesType({
                chart,
                visibleTimeRange: visibleTimeRange(),
              });

              /** @param {PriceSeriesType} type */
              function _createPriceSeries(type) {
                return createPriceSeries({
                  chart,
                  chartSeries,
                  lastVisibleDatasetIndex,
                  preset,
                  setMinMaxMarkersWhenIdle,
                  type,
                });
              }

              const priceCandlestickSeries = _createPriceSeries("Candlestick");
              const priceLineSeries = _createPriceSeries("Line");

              function createLinkPriceSeriesEffect() {
                createEffect(() => {
                  priceCandlestickSeries.active.set(priceLineSeries.active());
                });

                createEffect(() => {
                  priceLineSeries.active.set(priceCandlestickSeries.active());
                });
              }
              createLinkPriceSeriesEffect();
            }

            [...seriesBlueprints]
              .reverse()
              .forEach((seriesBlueprint, index) => {
                const dataset = datasets.getOrImport(
                  scale,
                  seriesBlueprint.datasetPath
                );

                // Don't trigger reactivity by design
                activeDatasets().add(dataset);

                createSeries({
                  index,
                  seriesBlueprint,
                  chart,
                  preset,
                  lastVisibleDatasetIndex,
                  setMinMaxMarkersWhenIdle,
                  chartSeries,
                  dataset,
                });
              });

            setMinMaxMarkers();

            activeDatasets.set((s) => s);

            chartSeries.forEach((series) => {
              allSeries.unshift(series);

              createEffect(() => {
                series.active();
                untrack(setMinMaxMarkersWhenIdle);
              });
            });

            const chartVisible = createMemo(() =>
              chartSeries.some((series) => series.visible())
            );

            function createChartVisibilityEffect() {
              createEffect(() => {
                const chartWrapper = chartDiv.parentElement;
                if (!chartWrapper) throw "Should exist";
                chartWrapper.hidden = !chartVisible();
              });
            }
            createChartVisibilityEffect();

            function createTimeScaleVisibilityEffect() {
              createEffect(() => {
                const visible = chartIndex === chartCount - 1 && chartVisible();

                chart.timeScale().applyOptions({
                  visible,
                });

                if (chartIndex === 1) {
                  charts[0].timeScale().applyOptions({
                    visible: !visible,
                  });
                }
              });
            }
            createTimeScaleVisibilityEffect();

            // createEffect(() =>
            //   chart.priceScale("right").applyOptions({
            //     mode: chartPriceMode.selected() === "Linear" ? 0 : 1,
            //   })
            // );

            chart
              .timeScale()
              .subscribeVisibleLogicalRangeChange((logicalRange) => {
                if (!logicalRange) return;

                // Must be the chart with the visible timeScale
                if (chartIndex === chartCount - 1) {
                  debouncedUpdateVisiblePriceSeriesType({
                    chart,
                    visibleLogicalRange: logicalRange,
                  });
                }

                for (
                  let otherChartIndex = 0;
                  otherChartIndex <= chartCount - 1;
                  otherChartIndex++
                ) {
                  if (chartIndex !== otherChartIndex) {
                    charts[otherChartIndex]
                      .timeScale()
                      .setVisibleLogicalRange(logicalRange);
                  }
                }
              });

            chart.subscribeCrosshairMove(({ time, sourceEvent }) => {
              // Don't override crosshair position from scroll event
              if (time && !sourceEvent) return;

              for (
                let otherChartIndex = 0;
                otherChartIndex <= chartCount - 1;
                otherChartIndex++
              ) {
                const otherChart = charts[otherChartIndex];

                if (otherChart && chartIndex !== otherChartIndex) {
                  if (time) {
                    otherChart.setCrosshairPosition(
                      NaN,
                      time,
                      otherChart.whitespace
                    );
                  } else {
                    // No time when mouse goes outside the chart
                    otherChart.clearCrosshairPosition();
                  }
                }
              }
            });

            return chart;
          });
        }

        function createSelectedEffect() {
          // @ts-ignore
          createEffect((_previouslySelected) => {
            const previouslySelected = /** @type {Preset | undefined} */ (
              _previouslySelected
            );

            const preset = selected();

            untrack(() => {
              if (previouslySelected) {
                urlParamsHelpers.reset(preset.id);
              }
              urlParamsHelpers.replaceHistory({ pathname: preset.id });
              createRoot(applyPreset);
            });

            return preset;
          }, undefined);
        }
        createSelectedEffect();

        function initTimeScaleElement() {
          function initScrollButtons() {
            const buttonBackward = getElementById("button-backward");
            const buttonBackwardIcon = getElementById("button-backward-icon");
            const buttonBackwardPauseIcon = getElementById(
              "button-backward-pause-icon"
            );
            const buttonForward = getElementById("button-forward");
            const buttonForwardIcon = getElementById("button-forward-icon");
            const buttonForwardPauseIcon = getElementById(
              "button-forward-pause-icon"
            );

            let interval = /** @type {number | undefined} */ (undefined);
            let direction = /** @type  {1 | -1 | 0} */ (0);

            const DELAY = 5;
            const MULTIPLIER = DELAY / 10000;

            function scrollChart() {
              if (direction <= 0) {
                buttonForwardIcon.removeAttribute("hidden");
                buttonForwardPauseIcon.setAttribute("hidden", "");
              }
              if (direction >= 0) {
                buttonBackwardIcon.removeAttribute("hidden");
                buttonBackwardPauseIcon.setAttribute("hidden", "");
              }
              if (direction === -1) {
                buttonBackwardIcon.setAttribute("hidden", "");
                buttonBackwardPauseIcon.removeAttribute("hidden");
              }
              if (direction === 1) {
                buttonForwardIcon.setAttribute("hidden", "");
                buttonForwardPauseIcon.removeAttribute("hidden");
              }

              if (!direction) {
                clearInterval(interval);
                return;
              }

              interval = setInterval(() => {
                const time = charts.at(-1)?.timeScale();

                if (!time) return;

                const range = time.getVisibleLogicalRange();

                if (!range) return;

                const speed = (range.to - range.from) * MULTIPLIER * direction;

                // @ts-ignore
                range.from += speed;
                // @ts-ignore
                range.to += speed;

                time.setVisibleLogicalRange(range);
              }, DELAY);
            }

            buttonBackward.addEventListener("click", () => {
              if (direction !== -1) {
                direction = -1;
              } else {
                direction = 0;
              }
              scrollChart();
            });

            buttonForward.addEventListener("click", () => {
              if (direction !== 1) {
                direction = 1;
              } else {
                direction = 0;
              }
              scrollChart();
            });
          }
          initScrollButtons();

          const GENESIS_DAY = "2009-01-03";

          /**
           * @param {HTMLButtonElement} button
           */
          function setTimeScale(button) {
            const chart = charts.at(-1);
            if (!chart) return;
            const timeScale = chart.timeScale();

            const year = button.dataset.year;
            let days = button.dataset.days;
            let toHeight = button.dataset.to;

            if (scale() === "date") {
              let from = new Date();
              let to = new Date();
              to.setUTCHours(0, 0, 0, 0);

              if (!days && typeof button.dataset.yearToDate === "string") {
                days = String(
                  Math.ceil(
                    (to.getTime() -
                      new Date(`${to.getUTCFullYear()}-01-01`).getTime()) /
                      ONE_DAY_IN_MS
                  )
                );
              }

              if (year) {
                from = new Date(`${year}-01-01`);
                to = new Date(`${year}-12-31`);
              } else if (days) {
                from.setDate(from.getUTCDate() - Number(days));
              } else {
                from = new Date(GENESIS_DAY);
              }

              timeScale.setVisibleRange({
                from: /** @type {Time} */ (from.getTime() / 1000),
                to: /** @type {Time} */ (to.getTime() / 1000),
              });
            } else if (scale() === "height") {
              timeScale.setVisibleRange({
                from: /** @type {Time} */ (0),
                to: /** @type {Time} */ (
                  Number(toHeight?.slice(0, -1)) * 1_000
                ),
              });
            }
          }

          /**
           * @param {HTMLElement} timeScaleButtons
           */
          function initGoToButtons(timeScaleButtons) {
            Array.from(timeScaleButtons.children).forEach((button) => {
              if (button.tagName !== "BUTTON") throw "Expect a button";
              button.addEventListener("click", () => {
                setTimeScale(/** @type {HTMLButtonElement} */ (button));
              });
            });
          }
          initGoToButtons(timeScaleDateButtons);
          initGoToButtons(timeScaleHeightButtons);

          function createScaleButtonsToggleEffect() {
            createEffect(() => {
              const scaleIsDate = scale() === "date";

              timeScaleDateButtons.hidden = !scaleIsDate;
              timeScaleHeightButtons.hidden = scaleIsDate;
            });
          }
          createScaleButtonsToggleEffect();
        }
        initTimeScaleElement();

        function initFavoriteButton() {
          buttonFavorite.addEventListener("click", () => {
            const preset = selected();

            preset.isFavorite.set((f) => {
              const newState = !f;

              const localStorageKey = presetToFavoriteLocalStorageKey(preset);
              if (newState) {
                localStorage.setItem(localStorageKey, "1");
              } else {
                localStorage.removeItem(localStorageKey);
              }

              return newState;
            });
          });

          createEffect(() => {
            if (selected().isFavorite()) {
              buttonFavorite.dataset.highlight = "";
            } else {
              delete buttonFavorite.dataset.highlight;
            }
          });
        }
        initFavoriteButton();
      }
    )
  );

  function initSearch() {
    const resetSearchButton = getElementById("reset-search");

    /**
     * @param {string} [value = '']
     */
    function setInputValue(value = "") {
      searchInput.focus();
      searchInput.value = value;
      searchInput.dispatchEvent(new Event("input"));
    }

    resetSearchButton.addEventListener("click", () => {
      setInputValue();
    });

    const localStorageSearchKey = "search";

    function initInput() {
      const haystack = presetsList.map(
        (preset) => `${preset.title}\t${preset.serializedPath}`
      );

      const searchSmallOgInnerHTML = searchSmall.innerHTML;

      const RESULTS_PER_PAGE = 100;

      import("./libraries/ufuzzy/script.js").then(({ default: ufuzzy }) => {
        /**
         * @param {uFuzzy.SearchResult} searchResult
         * @param {number} pageIndex
         */
        function computeResultPage(searchResult, pageIndex) {
          /** @type {{ preset: Preset, path: string, title: string }[]} */
          let list = [];

          let [indexes, info, order] = searchResult || [null, null, null];

          const minIndex = pageIndex * RESULTS_PER_PAGE;

          if (indexes?.length) {
            const maxIndex = Math.min(
              (order || indexes).length - 1,
              minIndex + RESULTS_PER_PAGE - 1
            );

            list = Array(maxIndex - minIndex + 1);

            if (info && order) {
              for (let i = minIndex; i <= maxIndex; i++) {
                let infoIdx = order[i];

                const [title, path] = ufuzzy
                  .highlight(haystack[info.idx[infoIdx]], info.ranges[infoIdx])
                  .split("\t");

                list[i % 100] = {
                  preset: presetsList[info.idx[infoIdx]],
                  path,
                  title,
                };
              }
            } else {
              for (let i = minIndex; i <= maxIndex; i++) {
                let index = indexes[i];

                const [title, path] = haystack[index].split("\t");

                list[i % 100] = {
                  preset: presetsList[index],
                  path,
                  title,
                };
              }
            }
          }

          return list;
        }

        /** @type {uFuzzy.Options} */
        const config = {
          intraIns: Infinity,
          intraChars: `[a-z\d' ]`,
        };

        const fuzzyMultiInsert = /** @type {uFuzzy} */ (
          ufuzzy({
            intraIns: 1,
          })
        );
        const fuzzyMultiInsertFuzzier = /** @type {uFuzzy} */ (ufuzzy(config));
        const fuzzySingleError = /** @type {uFuzzy} */ (
          ufuzzy({
            intraMode: 1,
            ...config,
          })
        );
        const fuzzySingleErrorFuzzier = /** @type {uFuzzy} */ (
          ufuzzy({
            intraMode: 1,
            ...config,
          })
        );

        /** @type {VoidFunction | undefined} */
        let dispose;

        function inputEvent() {
          createRoot((_dispose) => {
            const needle = /** @type {string} */ (searchInput.value);

            localeStorageHelpers.write(localStorageSearchKey, needle);

            dispose?.();

            dispose = _dispose;

            searchResults.scrollTo({
              top: 0,
            });

            if (!needle) {
              searchSmall.innerHTML = searchSmallOgInnerHTML;
              searchResults.innerHTML = "";
              return;
            }

            const outOfOrder = 5;
            const infoThresh = 5_000;

            let result = fuzzyMultiInsert?.search(
              haystack,
              needle,
              undefined,
              infoThresh
            );

            if (!result?.[0]?.length || !result?.[1]) {
              result = fuzzyMultiInsert?.search(
                haystack,
                needle,
                outOfOrder,
                infoThresh
              );
            }

            if (!result?.[0]?.length || !result?.[1]) {
              result = fuzzySingleError?.search(
                haystack,
                needle,
                outOfOrder,
                infoThresh
              );
            }

            if (!result?.[0]?.length || !result?.[1]) {
              result = fuzzySingleErrorFuzzier?.search(
                haystack,
                needle,
                outOfOrder,
                infoThresh
              );
            }

            if (!result?.[0]?.length || !result?.[1]) {
              result = fuzzyMultiInsertFuzzier?.search(
                haystack,
                needle,
                undefined,
                infoThresh
              );
            }

            if (!result?.[0]?.length || !result?.[1]) {
              result = fuzzyMultiInsertFuzzier?.search(
                haystack,
                needle,
                outOfOrder,
                infoThresh
              );
            }

            searchSmall.innerHTML = `Found <strong>${
              result?.[0]?.length || 0
            }</strong> preset(s)`;
            searchResults.innerHTML = "";

            const list = computeResultPage(result, 0);

            list.forEach(({ preset, path, title }) => {
              const li = window.document.createElement("li");
              searchResults.appendChild(li);

              const label = createPresetLabel({
                preset,
                frame: "search",
                name: title,
                top: path,
              });

              li.append(label);
            });
          });
        }

        if (searchInput.value) {
          inputEvent();
        }

        searchInput.addEventListener("input", inputEvent);
      });
    }

    searchInput.addEventListener("focus", initInput, {
      once: true,
    });

    if (!searchFrame.hidden) {
      setInputValue(localStorage.getItem(localStorageSearchKey) || "");
    }
  }
  initSearch();

  function initHistory() {
    const LOCAL_STORAGE_HISTORY_KEY = "history";
    const MAX_HISTORY_LENGTH = 1_000;

    const owner = getOwner();

    const history = /** @type {SerializedPresetsHistory} */ (
      JSON.parse(localStorage.getItem(LOCAL_STORAGE_HISTORY_KEY) || "[]")
    ).flatMap(([presetId, timestamp]) => {
      const preset = presetsList.find((preset) => preset.id === presetId);
      return preset ? [{ preset, date: new Date(timestamp) }] : [];
    });

    /** @param {Date} date  */
    function dateToTestedString(date) {
      return date.toLocaleString().split(",")[0];
    }

    /** @param {Date} date  */
    function dateToDisplayedString(date) {
      const formattedDate = dateToTestedString(date);

      const now = new Date();
      if (dateToTestedString(now) === formattedDate) {
        return "Today";
      }

      now.setUTCDate(now.getUTCDate() - 1);

      if (dateToTestedString(now) === formattedDate) {
        return "Yesterday";
      }

      return date.toLocaleDateString(undefined, {
        weekday: "long",
        year: "numeric",
        month: "long",
        day: "numeric",
      });
    }

    const grouped = history.reduce((grouped, { preset, date }) => {
      grouped[dateToTestedString(date)] ||= [];
      grouped[dateToTestedString(date)].push({ preset, date });
      return grouped;
    }, /** @type {Record<string, {preset: Preset, date: Date}[]>} */ ({}));

    /** @type {[string|undefined, string|undefined]} */
    const firstTwo = [undefined, undefined];

    function initHistoryListInDom() {
      Object.entries(grouped).forEach(([key, tuples], index) => {
        if (index < 2) {
          firstTwo[index] = key;
        }

        const heading = window.document.createElement("h4");
        heading.id = key;
        heading.innerHTML = dateToDisplayedString(tuples[0].date);
        historyList.append(heading);

        tuples.forEach(({ preset, date }) => {
          historyList.append(
            createPresetLabel({
              preset,
              frame: "history",
              name: preset.title,
              id: date.valueOf().toString(),
              top: date.toLocaleTimeString(),
              owner,
            })
          );
        });
      });
    }
    initHistoryListInDom();

    function createUpdateHistoryEffect() {
      createEffect(() => {
        const preset = selected();
        const date = new Date();
        const testedString = dateToTestedString(date);

        const label = createPresetLabel({
          preset,
          frame: "history",
          name: preset.title,
          id: date.valueOf().toString(),
          top: date.toLocaleTimeString(),
          owner,
        });

        const li = window.document.createElement("li");
        li.append(label);

        if (testedString === firstTwo[0]) {
          if (selected() === grouped[testedString].at(0)?.preset) {
            return;
          }

          grouped[testedString].unshift({ preset, date });
          getElementById(testedString).after(li);
        } else {
          const [first, second] = firstTwo;
          /** @param {string | undefined} id  */
          function updateHeading(id) {
            if (!id) return;
            getElementById(id).innerHTML = dateToDisplayedString(
              grouped[id][0].date
            );
          }

          updateHeading(first);
          updateHeading(second);

          const heading = window.document.createElement("h4");
          heading.innerHTML = dateToDisplayedString(date);
          heading.id = testedString;

          historyList.prepend(li);
          historyList.prepend(heading);

          grouped[testedString] = [{ preset, date }];

          firstTwo[1] = firstTwo[0];
          firstTwo[0] = testedString;
        }

        history.unshift({
          date: new Date(),
          preset,
        });

        runWhenIdle(() => {
          /** @type {SerializedPresetsHistory} */
          const serializedHistory = history.map(({ preset, date }) => [
            preset.id,
            date.getTime(),
          ]);

          if (serializedHistory.length > MAX_HISTORY_LENGTH) {
            serializedHistory.length = MAX_HISTORY_LENGTH;
          }

          localStorage.setItem(
            LOCAL_STORAGE_HISTORY_KEY,
            JSON.stringify(serializedHistory)
          );
        });
      });
    }
    createUpdateHistoryEffect();
  }
  initHistory();

  function initSettings() {
    function initTheme() {
      const inputLight = /** @type {HTMLInputElement} */ (
        getElementById("settings-theme-light-input")
      );
      const inputDark = /** @type {HTMLInputElement} */ (
        getElementById("settings-theme-dark-input")
      );
      const inputSystem = /** @type {HTMLInputElement} */ (
        getElementById("settings-theme-system-input")
      );

      const settingsThemeLocalStorageKey = "settings-theme";

      let savedTheme = /** @type {SettingsTheme} */ (
        localStorage.getItem(settingsThemeLocalStorageKey)
      );

      switch (savedTheme) {
        case "dark": {
          inputDark.checked = true;
          break;
        }
        case "light": {
          inputLight.checked = true;
          break;
        }
        default:
        case "system": {
          inputSystem.checked = true;
          savedTheme = "system";
          break;
        }
      }

      const theme = createSignal(savedTheme);

      const preferredColorSchemeMatchMedia = window.matchMedia(
        "(prefers-color-scheme: dark)"
      );

      /**
       * @param {boolean} shouldBeDark
       */
      function updateTheme(shouldBeDark) {
        dark.set(shouldBeDark);

        if (shouldBeDark) {
          window.document.documentElement.dataset.theme = "dark";
        } else {
          delete window.document.documentElement.dataset.theme;
        }

        const backgroundColor = getComputedStyle(
          window.document.documentElement
        ).getPropertyValue("--background-color");
        const meta = dom.queryOrCreateMetaElement("theme-color");
        meta.content = backgroundColor;
      }

      function createUpdateDataThemeEffect() {
        createEffect(() => {
          localStorage.setItem(settingsThemeLocalStorageKey, theme());
          updateTheme(
            theme() === "dark" ||
              (theme() === "system" && preferredColorSchemeMatchMedia.matches)
          );
        });
      }
      createUpdateDataThemeEffect();

      preferredColorSchemeMatchMedia.addEventListener("change", (media) => {
        if (theme() === "system") {
          updateTheme(media.matches);
        }
      });

      getElementById("settings-theme-field").addEventListener(
        "change",
        (event) => {
          const newTheme = /** @type {SettingsTheme | string} */ (
            // @ts-ignore
            event.target?.value
          );
          switch (newTheme) {
            case "dark":
            case "light":
            case "system": {
              theme.set(newTheme);
              break;
            }
            default: {
              throw "Bad theme";
            }
          }
        }
      );
    }
    initTheme();

    function initLeaderboard() {
      const leaderboard = getElementById("leaderboard");

      const donations = [
        {
          name: "_Checkɱate",
          // url: "https://xcancel.com/_Checkmatey_",
          url: "https://primal.net/p/npub1qh5sal68c8swet6ut0w5evjmj6vnw29x3k967h7atn45unzjyeyq6ceh9r",
          amount: 500_000,
        },
        {
          name: "avvi |",
          url: "https://primal.net/p/npub1md2q6fexrtmd5hx9gw2p5640vg662sjlpxyz3tdmu4j4g8hhkm6scn6hx3",
          amount: 5_000,
        },
        {
          name: "mutatrum",
          url: "https://primal.net/p/npub1hklphk7fkfdgmzwclkhshcdqmnvr0wkfdy04j7yjjqa9lhvxuflsa23u2k",
          amount: 5_000,
        },
        {
          name: "Gunnar",
          url: "https://primal.net/p/npub1rx9wg2d5lhah45xst3580sajcld44m0ll9u5dqhu2t74p6xwufaqwghtd4",
          amount: 1_000,
        },
        {
          name: "Blokchain Boog",
          url: "https://xcancel.com/BlokchainB",
          amount: 1_500 + 1590,
        },
        {
          name: "Josh",
          url: "https://primal.net/p/npub1pc57ls4rad5kvsp733suhzl2d4u9y7h4upt952a2pucnalc59teq33dmza",
          amount: 1_000,
        },
        {
          name: "Alp",
          url: "https://primal.net/p/npub175nul9cvufswwsnpy99lvyhg7ad9nkccxhkhusznxfkr7e0zxthql9g6w0",
          amount: 1_000,
        },
        {
          name: "Ulysses",
          url: "https://primal.net/p/npub1n7n3dssm90hfsfjtamwh2grpzwjlvd2yffae9pqgg99583lxdypsnn9gtv",
          amount: 1_000,
        },
        {
          name: "btcschellingpt",
          url: "https://primal.net/p/npub1nvfgglea9zlcs58tcqlc6j26rt50ngkgdk7699wfq4txrx37aqcsz4e7zd",
          amount: 1_000 + 1_000,
        },
        {
          name: "Coinatra",
          url: "https://primal.net/p/npub1eut9kcejweegwp9waq3a4g03pvprdzkzvjjvl8fvj2a2wlx030eswzfna8",
          amount: 1_000,
        },
        {
          name: "Printer Go Brrrr",
          url: "https://primal.net/p/npub1l5pxvjzhw77h86tu0sml2gxg8jpwxch7fsj6d05n7vuqpq75v34syk4q0n",
          amount: 1_000,
        },
        {
          name: "b81776c32d7b",
          url: "https://primal.net/p/npub1hqthdsed0wpg57sqsc5mtyqxxgrh3s7493ja5h49v23v2nhhds4qk4w0kz",
          amount: 17_509,
        },
        {
          name: "DerGigi",
          url: "https://primal.net/p/npub1dergggklka99wwrs92yz8wdjs952h2ux2ha2ed598ngwu9w7a6fsh9xzpc",
          amount: 6001,
        },
        {
          name: "Adarnit",
          url: "https://primal.net/p/npub17armdveqy42uhuuuwjc5m2dgjkz7t7epgvwpuccqw8jusm8m0g4sn86n3s",
          amount: 17_726,
        },
        {
          name: "Auburn Citadel",
          url: "https://primal.net/p/npub1730y5k2s9u82w9snx3hl37r8gpsrmqetc2y3xyx9h65yfpf28rtq0y635y",
          amount: 17_471,
        },
        {
          name: "anon",
          amount: 210_000,
        },
        {
          name: "Daniel ∞/21M",
          url: "https://twitter.com/DanielAngelovBG",
          amount: 21_000,
        },
        {
          name: "Ivo",
          url: "https://primal.net/p/npub1mnwjn40hr042rsmzu64rsnwsw07uegg4tjkv620c94p6e797wkvq3qeujc",
          amount: 5_000,
        },
        {
          name: "lassdas",
          url: "https://primal.net/p/npub1gmhctt2hmjqz8ay2x8h5f8fl3h4fpfcezwqneal3usu3u65qca4s8094ea",
          amount: 210_000,
        },
        {
          name: "anon",
          amount: 21_000,
        },
        {
          name: "xplbzx",
          url: "https://primal.net/p/npub1e0f808a350rxrhppu4zylzljt3arfpvrrpqdg6ft78xy6u49kq5slf0g92",
          amount: 12_110,
        },
        {
          name: "SoundMoney=Prosperity4ALL",
          url: "https://xcancel.com/SoundmoneyP",
          amount: 420_000,
        },
        {
          name: "Johan",
          url: "https://primal.net/p/npub1a4sd4cprrucfkvkfq9zs99ur4xe7lxw3uhhgvuzx6nqxhnpa2yyqlsa26u",
          amount: 500_000,
        },
        {
          name: "highperfocused",
          url: "https://primal.net/p/npub1fq8vrf63vsrqjrwqgtwlvauqauc0yme6se8g8dqhcpf6tfs3equqntmzut",
          amount: 4620,
        },
        {
          name: "ClearMined",
          url: "https://primal.net/p/npub1dj8zwktp3eyktfhs5mjlw8v0v2838xlquxr7ddsanayhcw98fcks8ddrq9",
          amount: 300_000,
        },
      ];

      donations.sort((a, b) =>
        b.amount !== a.amount
          ? b.amount - a.amount
          : a.name.localeCompare(b.name)
      );

      donations.slice(0, 21).forEach(({ name, url, amount }) => {
        const li = window.document.createElement("li");
        leaderboard.append(li);

        const a = window.document.createElement("a");
        a.href = url || "";
        a.target = "_blank";
        a.rel = "noopener noreferrer";
        a.innerHTML = name;
        li.append(a);

        li.append(" — ");

        const small = window.document.createElement("small");
        small.classList.add("sats");
        small.innerHTML = `${amount.toLocaleString("en-us")} sats`;
        li.append(small);
      });
    }
    initLeaderboard();

    function initInstallInstructions() {
      if (
        !env.standalone &&
        env.safariOnly &&
        (env.macOS || env.ipad || env.iphone)
      ) {
        const installInstructionsElement = getElementById(
          "settings-install-instructions"
        );
        installInstructionsElement.hidden = false;

        const hr = window.document.createElement("hr");
        installInstructionsElement.before(hr);

        const heading = window.document.createElement("h4");
        heading.innerHTML = "Install";
        installInstructionsElement.append(heading);

        const p = window.document.createElement("p");
        installInstructionsElement.append(p);

        if (env.macOS) {
          p.innerHTML = `This app can be installed by clicking on the <strong>File</strong> tab on the menu bar and then on <strong>Add to dock</strong>.`;
        } else {
          p.innerHTML = `This app can be installed by tapping on the <strong>Share</strong> button tab of Safari and then on <strong>Add to Home Screen</strong>.`;
        }
      }
    }
    initInstallInstructions();

    function initMobileNav() {
      const anchorApi = /** @type {HTMLAnchorElement} */ (
        getElementById("anchor-api").cloneNode(true)
      );

      const anchorGit = /** @type {HTMLAnchorElement} */ (
        getElementById("anchor-git").cloneNode(true)
      );

      const anchorNostr = /** @type {HTMLAnchorElement} */ (
        getElementById("anchor-nostr").cloneNode(true)
      );

      const anchorGeyser = /** @type {HTMLAnchorElement} */ (
        getElementById("anchor-geyser").cloneNode(true)
      );

      if (!anchorApi || !anchorGit || !anchorNostr || !anchorGeyser)
        throw "Anchors should exist by now";

      anchorApi.id = "";
      anchorGit.id = "";
      anchorNostr.id = "";
      anchorGeyser.id = "";

      const nav = getElementById("settings-nav");

      nav.append(anchorApi);
      nav.append(anchorGit);
      nav.append(anchorNostr);
      nav.append(anchorGeyser);
    }
    initMobileNav();
  }
  initSettings();
}
initEverythingRelatedToPresets();
