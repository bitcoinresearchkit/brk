const localhost = window.location.hostname === "localhost";

/**
 * @param {string} serialized
 * @returns {boolean}
 */
export function isSerializedBooleanTrue(serialized) {
  return serialized === "true" || serialized === "1";
}

/**
 * @param {number} ms
 */
export function sleep(ms) {
  return new Promise((resolve) => {
    setTimeout(resolve, ms);
  });
}

export function next() {
  return sleep(0);
}

export const array = {
  /**
   * @param {number} start
   * @param {number} end
   */
  range(start, end) {
    const range = [];
    while (start <= end) {
      range.push(start);
      start += 1;
    }
    return range;
  },
  /**
   * @template T
   * @param {T[]} array
   */
  random(array) {
    return array[Math.floor(Math.random() * array.length)];
  },
};

export const dom = {
  /**
   * @param {string} id
   * @returns {HTMLElement}
   */
  getElementById(id) {
    const element = window.document.getElementById(id);
    if (!element) throw `Element with id = "${id}" should exist`;
    return element;
  },
  /**
   * @param {HTMLElement} element
   */
  isHidden(element) {
    return element.tagName !== "BODY" && !element.offsetParent;
  },
  /**
   *
   * @param {HTMLElement} element
   * @param {VoidFunction} callback
   */
  onFirstIntersection(element, callback) {
    const observer = new IntersectionObserver((entries) => {
      for (let i = 0; i < entries.length; i++) {
        if (entries[i].isIntersecting) {
          callback();
          observer.disconnect();
        }
      }
    });
    observer.observe(element);
  },
  /**
   * @param {string} name
   */
  createSpanName(name) {
    const spanName = window.document.createElement("span");
    spanName.classList.add("name");
    const [first, second, third] = name.split(" - ");
    spanName.innerHTML = first;

    if (second) {
      const smallRest = window.document.createElement("small");
      smallRest.innerHTML = ` — ${second}`;
      spanName.append(smallRest);

      if (third) {
        throw "Shouldn't have more than one dash";
      }
    }

    return spanName;
  },
  /**
   * @param {Object} arg
   * @param {string} arg.href
   * @param {string} arg.title
   * @param {string} [arg.text]
   * @param {boolean} [arg.blank]
   * @param {VoidFunction} [arg.onClick]
   * @param {boolean} [arg.preventDefault]
   */
  createAnchorElement({ text, href, blank, onClick, title, preventDefault }) {
    const anchor = window.document.createElement("a");
    anchor.href = href;
    anchor.title = title.toUpperCase();

    if (text) {
      anchor.innerText = text;
    }

    if (blank) {
      anchor.target = "_blank";
      anchor.rel = "noopener noreferrer";
    }

    if (onClick || preventDefault) {
      if (onClick) {
        anchor.addEventListener("click", (event) => {
          event.preventDefault();
          onClick();
        });
      }
    }

    return anchor;
  },
  /**
   * @param {Object} arg
   * @param {string | HTMLElement} arg.inside
   * @param {string} arg.title
   * @param {(event: MouseEvent) => void} arg.onClick
   */
  createButtonElement({ inside: text, onClick, title }) {
    const button = window.document.createElement("button");

    button.append(text);

    button.title = title.toUpperCase();

    button.addEventListener("click", onClick);

    return button;
  },

  /**
   * @param {Object} args
   * @param {string} args.inputName
   * @param {string} args.inputId
   * @param {string} args.inputValue
   * @param {boolean} [args.inputChecked=false]
   * @param {string} [args.title]
   * @param {'radio' | 'checkbox'} args.type
   * @param {(event: MouseEvent) => void} [args.onClick]
   */
  createLabeledInput({
    inputId,
    inputName,
    inputValue,
    inputChecked = false,
    title,
    onClick,
    type,
  }) {
    const label = window.document.createElement("label");

    inputId = inputId.toLowerCase();

    const input = window.document.createElement("input");
    if (type === "radio") {
      input.type = "radio";
      input.name = inputName;
    } else {
      input.type = "checkbox";
    }
    input.id = inputId;
    input.value = inputValue;
    input.checked = inputChecked;
    label.append(input);

    label.id = `${inputId}-label`;
    if (title) {
      label.title = title;
    }
    label.htmlFor = inputId;

    if (onClick) {
      label.addEventListener("click", onClick);
    }

    return {
      label,
      input,
    };
  },
  /**
   * @param {HTMLElement} parent
   * @param {HTMLElement} child
   * @param {number} index
   */
  insertElementAtIndex(parent, child, index) {
    if (!index) index = 0;
    if (index >= parent.children.length) {
      parent.appendChild(child);
    } else {
      parent.insertBefore(child, parent.children[index]);
    }
  },
  /**
   * @param {string} url
   * @param {boolean} [targetBlank]
   */
  open(url, targetBlank) {
    console.log(`open: ${url}`);
    const a = window.document.createElement("a");
    window.document.body.append(a);
    a.href = url;

    if (targetBlank) {
      a.target = "_blank";
      a.rel = "noopener noreferrer";
    }

    a.click();
    a.remove();
  },
  /**
   * @param {string} href
   */
  importStyle(href) {
    const link = document.createElement("link");
    link.href = href;
    link.type = "text/css";
    link.rel = "stylesheet";
    link.media = "screen,print";
    const head = window.document.getElementsByTagName("head")[0];
    head.appendChild(link);
    return link;
  },
  /**
   * @template {Readonly<string[]>} T
   * @param {Object} args
   * @param {T[number]} args.defaultValue
   * @param {string} [args.id]
   * @param {T | Accessor<T>} args.choices
   * @param {string} [args.keyPrefix]
   * @param {string} args.key
   * @param {boolean} [args.sorted]
   * @param {Signals} args.signals
   */
  createHorizontalChoiceField({
    id,
    choices: unsortedChoices,
    defaultValue,
    keyPrefix,
    key,
    signals,
    sorted,
  }) {
    const choices = signals.createMemo(() => {
      /** @type {T} */
      let c;
      if (typeof unsortedChoices === "function") {
        c = unsortedChoices();
      } else {
        c = unsortedChoices;
      }

      return sorted
        ? /** @type {T} */ (
            /** @type {any} */ (c.toSorted((a, b) => a.localeCompare(b)))
          )
        : c;
    });

    /** @type {Signal<T[number]>} */
    const selected = signals.createSignal(defaultValue, {
      save: {
        ...serde.string,
        keyPrefix: keyPrefix ?? "",
        key,
        saveDefaultValue: true,
      },
    });

    const field = window.document.createElement("div");
    field.classList.add("field");

    const div = window.document.createElement("div");
    field.append(div);

    signals.createEffect(choices, (choices) => {
      const s = selected();
      if (!choices.includes(s)) {
        if (choices.includes(defaultValue)) {
          selected.set(() => defaultValue);
        } else if (choices.length) {
          selected.set(() => choices[0]);
        }
      }

      div.innerHTML = "";

      choices.forEach((choice) => {
        const inputValue = choice;
        const { label } = this.createLabeledInput({
          inputId: `${id ?? key}-${choice.toLowerCase()}`,
          inputName: id ?? key,
          inputValue,
          inputChecked: inputValue === selected(),
          // title: choice,
          type: "radio",
        });

        const text = window.document.createTextNode(choice);
        label.append(text);
        div.append(label);
      });
    });

    field.addEventListener("change", (event) => {
      // @ts-ignore
      const value = event.target.value;
      selected.set(value);
    });

    return { field, selected };
  },
  /**
   * @param {string} [title]
   * @param {1 | 2 | 3} [level]
   */
  createHeader(title = "", level = 1) {
    const headerElement = window.document.createElement("header");

    const headingElement = window.document.createElement(`h${level}`);
    headingElement.innerHTML = title;
    headerElement.append(headingElement);
    headingElement.style.display = "block";

    return {
      headerElement,
      headingElement,
    };
  },
  /**
   * @template {string} Name
   * @template {string} Value
   * @template {Value | {name: Name; value: Value}} T
   * @param {T} arg
   */
  createOption(arg) {
    const option = window.document.createElement("option");
    if (typeof arg === "object") {
      option.value = arg.value;
      option.innerText = arg.name;
    } else {
      option.value = arg;
      option.innerText = arg;
    }
    return option;
  },
  /**
   * @template {string} Name
   * @template {string} Value
   * @template {Value | {name: Name; value: Value}} T
   * @param {Object} args
   * @param {string} [args.id]
   * @param {boolean} [args.deep]
   * @param {readonly ((T) | {name: string; list: T[]})[]} args.list
   * @param {Signal<T>} args.signal
   */
  createSelect({ id, list, signal, deep = false }) {
    const select = window.document.createElement("select");

    if (id) {
      select.name = id;
      select.id = id;
    }

    /** @type {Record<string, VoidFunction>} */
    const setters = {};

    list.forEach((anyOption, index) => {
      if (typeof anyOption === "object" && "list" in anyOption) {
        const { name, list } = anyOption;
        const optGroup = window.document.createElement("optgroup");
        optGroup.label = name;
        select.append(optGroup);
        list.forEach((option) => {
          optGroup.append(this.createOption(option));
          const key = /** @type {string} */ (
            typeof option === "object" ? option.value : option
          );
          setters[key] = () => signal.set(() => option);
        });
      } else {
        select.append(this.createOption(anyOption));
        const key = /** @type {string} */ (
          typeof anyOption === "object" ? anyOption.value : anyOption
        );
        setters[key] = () => signal.set(() => anyOption);
      }
      if (deep && index !== list.length - 1) {
        select.append(window.document.createElement("hr"));
      }
    });

    select.addEventListener("change", () => {
      const callback = setters[select.value];
      // @ts-ignore
      if (callback) {
        callback();
      }
    });

    const initialSignal = signal();
    const initialValue =
      typeof initialSignal === "object" ? initialSignal.value : initialSignal;
    select.value = String(initialValue);

    return { select, signal };
  },
  /**
   * @param {Object} args
   * @param {string} args.title
   * @param {string} args.description
   * @param {HTMLElement} args.input
   */
  createFieldElement({ title, description, input }) {
    const div = window.document.createElement("div");

    const label = window.document.createElement("label");
    div.append(label);

    const titleElement = window.document.createElement("span");
    titleElement.innerHTML = title;
    label.append(titleElement);

    const descriptionElement = window.document.createElement("small");
    descriptionElement.innerHTML = description;
    label.append(descriptionElement);

    div.append(input);

    const forId = input.id || input.firstElementChild?.id;

    if (!forId) {
      console.log(input);
      throw `Input should've an ID`;
    }

    label.htmlFor = forId;

    return div;
  },
  /**
   * @param {'left' | 'bottom' | 'top' | 'right'} position
   */
  createShadow(position) {
    const div = window.document.createElement("div");
    div.classList.add(`shadow-${position}`);
    return div;
  },
};

/**
 * @param {string | string[]} [pathname]
 */
function processPathname(pathname) {
  pathname ||= window.location.pathname;
  return Array.isArray(pathname) ? pathname.join("/") : pathname;
}

export const url = {
  chartParamsWhitelist: ["from", "to"],
  /**
   * @param {string | string[]} pathname
   */
  pushHistory(pathname) {
    const urlParams = new URLSearchParams(window.location.search);
    pathname = processPathname(pathname);
    try {
      const url = `/${pathname}?${urlParams.toString()}`;
      console.log(`push history: ${url}`);
      window.history.pushState(null, "", url);
    } catch (_) {}
  },
  /**
   * @param {Object} args
   * @param {URLSearchParams} [args.urlParams]
   * @param {string | string[]} [args.pathname]
   */
  replaceHistory({ urlParams, pathname }) {
    urlParams ||= new URLSearchParams(window.location.search);
    pathname = processPathname(pathname);
    try {
      const url = `/${pathname}?${urlParams.toString()}`;
      console.log(`replace history: ${url}`);
      window.history.replaceState(null, "", url);
    } catch (_) {}
  },
  /**
   * @param {Option} option
   */
  resetParams(option) {
    const urlParams = new URLSearchParams();
    if (option.kind === "chart") {
      [...new URLSearchParams(window.location.search).entries()]
        .filter(([key, _]) => this.chartParamsWhitelist.includes(key))
        .forEach(([key, value]) => {
          urlParams.set(key, value);
        });
    }
    this.replaceHistory({ urlParams, pathname: option.path.join("/") });
  },
  /**
   * @param {string} key
   * @param {string | boolean | null | undefined} value
   */
  writeParam(key, value) {
    const urlParams = new URLSearchParams(window.location.search);

    if (value !== null && value !== undefined) {
      urlParams.set(key, String(value));
    } else {
      urlParams.delete(key);
    }

    this.replaceHistory({ urlParams });
  },
  /**
   * @param {string} key
   */
  removeParam(key) {
    this.writeParam(key, undefined);
  },
  /**
   *
   * @param {string} key
   * @returns {boolean | null}
   */
  readBoolParam(key) {
    const parameter = this.readParam(key);

    if (parameter) {
      return isSerializedBooleanTrue(parameter);
    }

    return null;
  },
  /**
   *
   * @param {string} key
   * @returns {number | null}
   */
  readNumberParam(key) {
    const parameter = this.readParam(key);

    if (parameter) {
      return Number(parameter);
    }

    return null;
  },
  /**
   *
   * @param {string} key
   * @returns {string | null}
   */
  readParam(key) {
    const urlParams = new URLSearchParams(window.location.search);
    return urlParams.get(key);
  },
};

/**
 * @param {number} value
 * @param {number} [digits]
 * @param {Intl.NumberFormatOptions} [options]
 */
function numberToUSFormat(value, digits, options) {
  return value.toLocaleString("en-us", {
    ...options,
    minimumFractionDigits: digits,
    maximumFractionDigits: digits,
  });
}

const thoroughUnitCheck = localhost;

/**
 * @param {VecId} id
 */
export function vecidToUnit(id) {
  /** @type {Unit | undefined} */
  let unit;

  /**

     * @param {Unit} u
     */
  function setUnit(u) {
    if (unit)
      throw Error(
        `Can't assign "${u}" to unit, "${unit}" is already assigned to "${id}"`,
      );
    unit = u;
  }

  if (
    (!unit || thoroughUnitCheck) &&
    (id.includes("in_sats") ||
      (id.endsWith("supply") &&
        !(id.endsWith("circulating_supply") || id.endsWith("_own_supply"))) ||
      id === "sent" ||
      id === "annualized_volume" ||
      id.endsWith("supply_half") ||
      id.endsWith("supply_breakeven") ||
      id.endsWith("supply_in_profit") ||
      id.endsWith("supply_in_loss") ||
      id.endsWith("stack") ||
      (id.endsWith("value") && !id.includes("realized")) ||
      ((id.includes("coinbase") ||
        id.includes("fee") ||
        id.includes("subsidy") ||
        id.includes("rewards")) &&
        !(
          id.startsWith("is_") ||
          id.includes("_btc") ||
          id.includes("_usd") ||
          id.includes("fee_rate") ||
          id.endsWith("dominance")
        )))
  ) {
    setUnit("sats");
  }
  if (
    (!unit || thoroughUnitCheck) &&
    !id.endsWith("velocity") &&
    ((id.includes("_btc") &&
      !(id.includes("0k_btc") || id.includes("1k_btc"))) ||
      id.endsWith("_btc"))
  ) {
    setUnit("btc");
  }
  if ((!unit || thoroughUnitCheck) && id === "chain") {
    setUnit("block");
  }
  if ((!unit || thoroughUnitCheck) && id.startsWith("blocks_before")) {
    setUnit("blocks");
  }
  if (
    (!unit || thoroughUnitCheck) &&
    (id === "emptyaddressdata" || id === "loadedaddressdata")
  ) {
    setUnit("address data");
  }
  if (
    (!unit || thoroughUnitCheck) &&
    (id === "price_high" ||
      id === "price_ohlc" ||
      id === "price_low" ||
      id === "price_close" ||
      id === "price_open" ||
      id === "price_ath" ||
      id === "market_cap" ||
      id.startsWith("price_true_range") ||
      (id.includes("_usd") && !id.endsWith("velocity")) ||
      id.includes("cointime_value") ||
      id.endsWith("_ago") ||
      id.endsWith("price_paid") ||
      id.endsWith("_price") ||
      (id.startsWith("price") && (id.endsWith("min") || id.endsWith("max"))) ||
      (id.endsWith("_cap") && !id.includes("rel_to")) ||
      id.endsWith("value_created") ||
      id.endsWith("value_destroyed") ||
      ((id.includes("realized") || id.includes("true_market_mean")) &&
        !id.includes("ratio") &&
        !id.includes("rel_to")) ||
      ((id.endsWith("sma") || id.includes("sma_x") || id.endsWith("ema")) &&
        !id.includes("ratio") &&
        !id.includes("sopr") &&
        !id.includes("hash_rate")) ||
      id === "ath")
  ) {
    setUnit("usd");
  }
  if ((!unit || thoroughUnitCheck) && id.endsWith("cents")) {
    setUnit("cents");
  }
  if (
    ((!unit || thoroughUnitCheck) &&
      (id.endsWith("ratio") ||
        (id.includes("ratio") &&
          (id.endsWith("sma") ||
            id.endsWith("ema") ||
            id.endsWith("zscore"))) ||
        id.includes("sopr") ||
        id.endsWith("_5sd") ||
        id.endsWith("1sd") ||
        id.endsWith("2sd") ||
        id.endsWith("3sd") ||
        id.endsWith("pct1") ||
        id.endsWith("pct2") ||
        id.endsWith("pct5") ||
        id.endsWith("pct95") ||
        id.endsWith("pct98") ||
        id.endsWith("pct99"))) ||
    id.includes("liveliness") ||
    id.includes("vaultedness") ||
    id == "puell_multiple" ||
    id.endsWith("velocity")
  ) {
    setUnit("ratio");
  }
  if (
    (!unit || thoroughUnitCheck) &&
    (id === "price_drawdown" ||
      id === "difficulty_adjustment" ||
      id.endsWith("inflation_rate") ||
      id.endsWith("_oscillator") ||
      id.endsWith("_dominance") ||
      id.endsWith("_returns") ||
      id.endsWith("_rebound") ||
      id.endsWith("_volatility") ||
      id.endsWith("_cagr"))
  ) {
    setUnit("percentage");
  }
  if (
    (!unit || thoroughUnitCheck) &&
    (id.endsWith("count") ||
      id.includes("_count_") ||
      id.startsWith("block_count") ||
      id.includes("blocks_mined") ||
      (id.includes("tx_v") && !id.includes("vsize")))
  ) {
    setUnit("count");
  }
  if (
    (!unit || thoroughUnitCheck) &&
    (id.startsWith("hash_rate") || id.endsWith("as_hash"))
  ) {
    setUnit("h/s");
  }
  if ((!unit || thoroughUnitCheck) && id === "pool") {
    setUnit("id");
  }
  if ((!unit || thoroughUnitCheck) && id.includes("fee_rate")) {
    setUnit("sat/vb");
  }
  if ((!unit || thoroughUnitCheck) && id.startsWith("is_")) {
    setUnit("bool");
  }
  if ((!unit || thoroughUnitCheck) && id.endsWith("type")) {
    setUnit("type");
  }
  if (
    (!unit || thoroughUnitCheck) &&
    (id === "interval" || id.startsWith("block_interval"))
  ) {
    setUnit("secs");
  }
  if ((!unit || thoroughUnitCheck) && id.endsWith("_per_sec")) {
    setUnit("/sec");
  }
  if ((!unit || thoroughUnitCheck) && id.endsWith("locktime")) {
    setUnit("locktime");
  }

  if ((!unit || thoroughUnitCheck) && id.endsWith("version")) {
    setUnit("version");
  }
  if (
    (!unit || thoroughUnitCheck) &&
    (id === "txid" ||
      (id.endsWith("bytes") && !id.endsWith("vbytes")) ||
      id.endsWith("base_size") ||
      id.endsWith("total_size") ||
      id.includes("block_size"))
  ) {
    setUnit("bytes");
  }
  if ((!unit || thoroughUnitCheck) && id.endsWith("_sd")) {
    setUnit("sd");
  }
  if (
    (!unit || thoroughUnitCheck) &&
    (id.includes("vsize") || id.includes("vbytes"))
  ) {
    setUnit("vb");
  }
  if ((!unit || thoroughUnitCheck) && id.includes("weight")) {
    setUnit("wu");
  }
  if ((!unit || thoroughUnitCheck) && id.endsWith("index")) {
    setUnit("index");
  }
  if ((!unit || thoroughUnitCheck) && (id === "date" || id === "date_fixed")) {
    setUnit("date");
  }
  if (
    (!unit || thoroughUnitCheck) &&
    (id === "timestamp" || id === "timestamp_fixed")
  ) {
    setUnit("timestamp");
  }
  if ((!unit || thoroughUnitCheck) && id.includes("coinblocks")) {
    setUnit("coinblocks");
  }
  if ((!unit || thoroughUnitCheck) && id.includes("coindays")) {
    setUnit("coindays");
  }
  if ((!unit || thoroughUnitCheck) && id.includes("satblocks")) {
    setUnit("satblocks");
  }
  if ((!unit || thoroughUnitCheck) && id.includes("satdays")) {
    setUnit("satdays");
  }
  if ((!unit || thoroughUnitCheck) && id.endsWith("height")) {
    setUnit("height");
  }
  if ((!unit || thoroughUnitCheck) && id.endsWith("rel_to_market_cap")) {
    setUnit("%mcap");
  }
  if ((!unit || thoroughUnitCheck) && id.endsWith("rel_to_own_market_cap")) {
    setUnit("%cmcap");
  }
  if (
    (!unit || thoroughUnitCheck) &&
    id.endsWith("rel_to_own_total_unrealized_pnl")
  ) {
    setUnit("%cp+l");
  }
  if ((!unit || thoroughUnitCheck) && id.endsWith("rel_to_realized_cap")) {
    setUnit("%rcap");
  }
  if (
    (!unit || thoroughUnitCheck) &&
    id.endsWith("rel_to_circulating_supply")
  ) {
    setUnit("%all");
  }
  if (
    (!unit || thoroughUnitCheck) &&
    (id.includes("rel_to_realized_profit") ||
      id.includes("rel_to_realized_loss"))
  ) {
    setUnit("%pnl");
  }
  if ((!unit || thoroughUnitCheck) && id.endsWith("rel_to_own_supply")) {
    setUnit("%self");
  }
  if ((!unit || thoroughUnitCheck) && id.endsWith("epoch")) {
    setUnit("epoch");
  }
  if ((!unit || thoroughUnitCheck) && id === "difficulty") {
    setUnit("difficulty");
  }
  if ((!unit || thoroughUnitCheck) && id === "blockhash") {
    setUnit("hash");
  }
  if ((!unit || thoroughUnitCheck) && id.startsWith("hash_price_phs")) {
    setUnit("usd/(ph/s)/day");
  }
  if ((!unit || thoroughUnitCheck) && id.startsWith("hash_price_ths")) {
    setUnit("usd/(th/s)/day");
  }
  if ((!unit || thoroughUnitCheck) && id.startsWith("hash_value_phs")) {
    setUnit("sats/(ph/s)/day");
  }
  if ((!unit || thoroughUnitCheck) && id.startsWith("hash_value_ths")) {
    setUnit("sats/(th/s)/day");
  }

  if (
    (!unit || thoroughUnitCheck) &&
    (id.includes("days_between") ||
      id.includes("days_since") ||
      id.startsWith("days_before"))
  ) {
    setUnit("days");
  }
  if ((!unit || thoroughUnitCheck) && id.includes("years_between")) {
    setUnit("years");
  }
  if ((!unit || thoroughUnitCheck) && id == "len") {
    setUnit("len");
  }
  if ((!unit || thoroughUnitCheck) && id == "position") {
    setUnit("position");
  }
  if ((!unit || thoroughUnitCheck) && id.startsWith("constant")) {
    setUnit("constant");
  }

  if (!unit) {
    console.log();
    throw Error(`Unit not set for "${id}"`);
  }
  return /** @type {Unit} */ (unit);
}

export const locale = {
  numberToUSFormat,
};

export const storage = {
  /**
   * @param {string} key
   */
  readNumber(key) {
    const saved = this.read(key);
    if (saved) {
      return Number(saved);
    }
    return null;
  },
  /**
   * @param {string} key
   */
  readBool(key) {
    const saved = this.read(key);
    if (saved) {
      return isSerializedBooleanTrue(saved);
    }
    return null;
  },
  /**
   * @param {string} key
   */
  read(key) {
    try {
      return localStorage.getItem(key);
    } catch (_) {
      return null;
    }
  },
  /**
   * @param {string} key
   * @param {string | boolean | null | undefined} value
   */
  write(key, value) {
    try {
      value !== undefined && value !== null
        ? localStorage.setItem(key, String(value))
        : localStorage.removeItem(key);
    } catch (_) {}
  },
  /**
   * @param {string} key
   */
  remove(key) {
    this.write(key, undefined);
  },
};

export const serde = {
  string: {
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
  },
  vecIds: {
    /**
     * @param {VecId[]} v
     */
    serialize(v) {
      return v.join(",");
    },
    /**
     * @param {string} v
     */
    deserialize(v) {
      return /** @type {VecId[]} */ (v.split(","));
    },
  },
  number: {
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
  },
  optNumber: {
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
  },
  optDate: {
    /**
     * @param {Date |  null} date
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
  },
  boolean: {
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
      if (v === "true") {
        return true;
      } else if (v === "false") {
        return false;
      } else {
        throw "deser bool err";
      }
    },
  },
  index: {
    /**
     * @param {Index} v
     */
    serialize(v) {
      switch (v) {
        case /** @satisfies {DateIndex} */ (0):
          return "dateindex";
        case /** @satisfies {DecadeIndex} */ (1):
          return "decadeindex";
        case /** @satisfies {DifficultyEpoch} */ (2):
          return "difficultyepoch";
        case /** @satisfies {EmptyOutputIndex} */ (3):
          return "emptyoutputindex";
        case /** @satisfies {HalvingEpoch} */ (4):
          return "halvingepoch";
        case /** @satisfies {Height} */ (5):
          return "height";
        case /** @satisfies {InputIndex} */ (6):
          return "inputindex";
        case /** @satisfies {MonthIndex} */ (7):
          return "monthindex";
        case /** @satisfies {OpReturnIndex} */ (8):
          return "opreturnindex";
        case /** @satisfies {OutputIndex} */ (9):
          return "outputindex";
        case /** @satisfies {P2AAddressIndex} */ (10):
          return "p2aaddressindex";
        case /** @satisfies {P2MSOutputIndex} */ (11):
          return "p2msoutputindex";
        case /** @satisfies {P2PK33AddressIndex} */ (12):
          return "p2pk33addressindex";
        case /** @satisfies {P2PK65AddressIndex} */ (13):
          return "p2pk65addressindex";
        case /** @satisfies {P2PKHAddressIndex} */ (14):
          return "p2pkhaddressindex";
        case /** @satisfies {P2SHAddressIndex} */ (15):
          return "p2shaddressindex";
        case /** @satisfies {P2TRAddressIndex} */ (16):
          return "p2traddressindex";
        case /** @satisfies {P2WPKHAddressIndex} */ (17):
          return "p2wpkhaddressindex";
        case /** @satisfies {P2WSHAddressIndex} */ (18):
          return "p2wshaddressindex";
        case /** @satisfies {QuarterIndex} */ (19):
          return "quarterindex";
        case /** @satisfies {SemesterIndex} */ (20):
          return "semesterindex";
        case /** @satisfies {TxIndex} */ (21):
          return "txindex";
        case /** @satisfies {UnknownOutputIndex} */ (22):
          return "unknownoutputindex";
        case /** @satisfies {WeekIndex} */ (23):
          return "weekindex";
        case /** @satisfies {YearIndex} */ (24):
          return "yearindex";
        case /** @satisfies {LoadedAddressIndex} */ (25):
          return "loadedaddressindex";
        case /** @satisfies {EmptyAddressIndex} */ (26):
          return "emptyaddressindex";
      }
    },
  },
  chartableIndex: {
    /**
     * @param {number} v
     * @returns {SerializedChartableIndex | null}
     */
    serialize(v) {
      switch (v) {
        case /** @satisfies {DateIndex} */ (0):
          return "date";
        case /** @satisfies {DecadeIndex} */ (1):
          return "decade";
        case /** @satisfies {DifficultyEpoch} */ (2):
          return "d.epoch";
        // case /** @satisfies {HalvingEpoch} */ (4):
        //   return "halving";
        case /** @satisfies {Height} */ (5):
          return "timestamp";
        case /** @satisfies {MonthIndex} */ (7):
          return "month";
        case /** @satisfies {QuarterIndex} */ (19):
          return "quarter";
        case /** @satisfies {SemesterIndex} */ (20):
          return "semester";
        case /** @satisfies {WeekIndex} */ (23):
          return "week";
        case /** @satisfies {YearIndex} */ (24):
          return "year";
        default:
          return null;
      }
    },
    /**
     * @param {SerializedChartableIndex} v
     * @returns {Index}
     */
    deserialize(v) {
      switch (v) {
        case "timestamp":
          return /** @satisfies {Height} */ (5);
        case "date":
          return /** @satisfies {DateIndex} */ (0);
        case "week":
          return /** @satisfies {WeekIndex} */ (23);
        case "d.epoch":
          return /** @satisfies {DifficultyEpoch} */ (2);
        case "month":
          return /** @satisfies {MonthIndex} */ (7);
        case "quarter":
          return /** @satisfies {QuarterIndex} */ (19);
        case "semester":
          return /** @satisfies {SemesterIndex} */ (20);
        case "year":
          return /** @satisfies {YearIndex} */ (24);
        case "decade":
          return /** @satisfies {DecadeIndex} */ (1);
        default:
          throw Error("todo");
      }
    },
  },
};

export const formatters = {
  dollars: new Intl.NumberFormat("en-US", {
    style: "currency",
    currency: "USD",
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  }),
  percentage: new Intl.NumberFormat("en-US", {
    style: "percent",
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  }),
};

export const date = {
  ONE_DAY_IN_MS: 1000 * 60 * 60 * 24,
  todayUTC() {
    const today = new Date();
    return new Date(
      Date.UTC(
        today.getUTCFullYear(),
        today.getUTCMonth(),
        today.getUTCDate(),
        0,
        0,
        0,
      ),
    );
  },
  /**
   * @param {Date} date
   */
  toString(date) {
    return date.toJSON().split("T")[0];
  },
  /**
   * @param {Date} date
   */
  toDateIndex(date) {
    if (
      date.getUTCFullYear() === 2009 &&
      date.getUTCMonth() === 0 &&
      date.getUTCDate() === 3
    )
      return 0;
    return this.differenceBetween(date, new Date("2009-01-09"));
  },
  /**
   * @param {Date} start
   * @param {Date} end
   */
  getRange(start, end) {
    const dates = /** @type {Date[]} */ ([]);
    let currentDate = new Date(start);
    while (currentDate <= end) {
      dates.push(new Date(currentDate));
      currentDate.setUTCDate(currentDate.getUTCDate() + 1);
    }
    return dates;
  },
  /**
   * @param {Date} date1
   * @param {Date} date2
   */
  differenceBetween(date1, date2) {
    return Math.abs(date1.valueOf() - date2.valueOf()) / this.ONE_DAY_IN_MS;
  },
};

/**
 *
 * @template {(...args: any[]) => any} F
 * @param {F} callback
 * @param {number} [wait]
 */
export function throttle(callback, wait = 1000) {
  /** @type {number | null} */
  let timeoutId = null;
  /** @type {Parameters<F>} */
  let latestArgs;

  return (/** @type {Parameters<F>} */ ...args) => {
    latestArgs = args;

    if (!timeoutId) {
      // Otherwise it optimizes away timeoutId in Chrome and FF
      timeoutId = timeoutId;
      timeoutId = setTimeout(() => {
        callback(...latestArgs); // Execute with latest args
        timeoutId = null;
      }, wait);
    }
  };
}

/**
 * @param {VoidFunction} callback
 * @param {number} [timeout = 1]
 */
export function runWhenIdle(callback, timeout = 1) {
  if ("requestIdleCallback" in window) {
    requestIdleCallback(callback);
  } else {
    setTimeout(callback, timeout);
  }
}

/**
 * @param {Date} oldest
 * @param {Date} youngest
 * @returns {number}
 */
export function getNumberOfDaysBetweenTwoDates(oldest, youngest) {
  return Math.round(
    Math.abs((youngest.getTime() - oldest.getTime()) / date.ONE_DAY_IN_MS),
  );
}

/**
 * @param {string} s
 */
export function stringToId(s) {
  return (
    s
      // .replace(/\W/g, " ")
      .trim()
      .replace(/ +/g, "-")
      .toLowerCase()
  );
}

export const api = (() => {
  const CACHE_NAME = "api";
  const API_VECS_PREFIX = "/api/vecs";

  /**
   * @template T
   * @param {(value: T) => void} callback
   * @param {string} path
   * @param {boolean} [mustBeArray]
   */
  async function fetchApi(callback, path, mustBeArray) {
    const url = `${API_VECS_PREFIX}${path}`;

    /** @type {T | null} */
    let cachedJson = null;

    /** @type {Cache | undefined} */
    let cache;
    try {
      cache = await caches.open(CACHE_NAME);
      const cachedResponse = await cache.match(url);
      if (cachedResponse) {
        console.debug(`cache: ${url}`);
        const json = /** @type {T} */ await cachedResponse.json();
        cachedJson = json;
        callback(json);
      }
    } catch {}

    if (navigator.onLine) {
      // TODO: rerun after 10s instead of returning (due to some kind of error)

      /** @type {Response | undefined} */
      let fetchedResponse;
      try {
        fetchedResponse = await fetch(url, {
          signal: AbortSignal.timeout(5000),
        });
        if (!fetchedResponse.ok) {
          throw Error;
        }
      } catch {
        return cachedJson;
      }

      const clonedResponse = fetchedResponse.clone();

      let fetchedJson = /** @type {T | null} */ (null);
      try {
        const f = await fetchedResponse.json();
        fetchedJson = /** @type {T} */ (
          mustBeArray && !Array.isArray(f) ? [f] : f
        );
      } catch (_) {
        return cachedJson;
      }

      if (!fetchedJson) return cachedJson;

      console.debug(`fetch: ${url}`);

      if (Array.isArray(cachedJson) && Array.isArray(fetchedJson)) {
        const previousLength = cachedJson?.length || 0;
        const newLength = fetchedJson.length;

        if (!newLength) {
          return cachedJson;
        }

        if (previousLength && previousLength === newLength) {
          const previousLastValue = Object.values(cachedJson || []).at(-1);
          const newLastValue = Object.values(fetchedJson).at(-1);
          if (
            JSON.stringify(previousLastValue) === JSON.stringify(newLastValue)
          ) {
            return cachedJson;
          }
        }
      }

      callback(fetchedJson);

      runWhenIdle(async function () {
        try {
          await cache?.put(url, clonedResponse);
        } catch (_) {}
      });

      return fetchedJson;
    } else {
      return cachedJson;
    }
  }

  /**
   * @param {Index} index
   * @param {VecId} vecId
   * @param {number} [from]
   * @param {number} [to]
   */
  function genPath(index, vecId, from, to) {
    let path = `/${serde.index.serialize(index)}-to-${vecId.replaceAll(
      "_",
      "-",
    )}?`;

    if (from !== undefined) {
      path += `from=${from}`;
    }
    if (to !== undefined) {
      if (!path.endsWith("?")) {
        path += `&`;
      }
      path += `to=${to}`;
    }
    return path;
  }

  return {
    /**
     * @param {Index} index
     * @param {VecId} vecId
     * @param {number} from
     */
    genUrl(index, vecId, from) {
      return `${API_VECS_PREFIX}${genPath(index, vecId, from)}`;
    },
    /**
     * @template {number | OHLCTuple} [T=number]
     * @param {(v: T[]) => void} callback
     * @param {Index} index
     * @param {VecId} vecId
     * @param {number} [from]
     * @param {number} [to]
     */
    fetchVec(callback, index, vecId, from, to) {
      return fetchApi(callback, genPath(index, vecId, from, to), true);
    },
    /**
     * @template {number | OHLCTuple} [T=number]
     * @param {(v: T) => void} callback
     * @param {Index} index
     * @param {VecId} vecId
     */
    fetchLast(callback, index, vecId) {
      return fetchApi(callback, genPath(index, vecId, -1));
    },
  };
})();
