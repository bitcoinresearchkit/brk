// @ts-check

/** @import {IChartApi, ISeriesApi, SeriesDefinition, SingleValueData as _SingleValueData, CandlestickData as _CandlestickData, BaselineData, SeriesType, IPaneApi, BaselineStyleOptions} from './v5.0.6-treeshaked/types' */

/**
 * @typedef {[number, number, number, number]} OHLCTuple
 *
 * @typedef {Object} Valued
 * @property {number} value
 *
 * @typedef {Object} Indexed
 * @property {number} index
 */

/**
 * @template T
 * @typedef {T & Valued & Indexed} ChartData<T>
 */

/**
 * @typedef {ChartData<_SingleValueData>} SingleValueData
 * @typedef {ChartData<_CandlestickData>} CandlestickData
 */

export default import("./v5.0.6-treeshaked/script.js").then((lc) => {
  const oklchToRGBA = createOklchToRGBA();

  /**
   * @param {Object} args
   * @param {HTMLElement} args.element
   * @param {Signals} args.signals
   * @param {Colors} args.colors
   * @param {Index} args.index
   * @param {Utilities} args.utils
   * @param {Elements} args.elements
   * @param {DeepPartial<ChartOptions>} [args.options]
   */
  function createLightweightChart({
    element,
    signals,
    colors,
    index,
    utils,
    elements,
    options: _options = {},
  }) {
    /** @satisfies {DeepPartial<ChartOptions>} */
    const options = {
      autoSize: true,
      layout: {
        fontFamily: elements.style.fontFamily,
        background: { color: "transparent" },
        attributionLogo: false,
        colorSpace: "display-p3",
        colorParsers: [oklchToRGBA],
      },
      grid: {
        vertLines: { visible: false },
        horzLines: { visible: false },
      },
      localization: {
        priceFormatter: utils.locale.numberToShortUSFormat,
        locale: "en-us",
      },
      ..._options,
    };

    /** @type {IChartApi} */
    const chart = lc.createChart(element, options);

    chart.priceScale("right").applyOptions({
      minimumWidth: 80,
    });

    signals.createEffect(
      () => ({
        defaultColor: colors.default(),
        offColor: colors.gray(),
        borderColor: colors.border(),
      }),
      ({ defaultColor, offColor, borderColor }) => {
        chart.applyOptions({
          layout: {
            textColor: offColor,
            panes: {
              separatorColor: borderColor,
            },
          },
          rightPriceScale: {
            borderVisible: false,
          },
          timeScale: {
            borderVisible: false,
            timeVisible:
              index === /** @satisfies {Height} */ (5) ||
              index === /** @satisfies {DifficultyEpoch} */ (2) ||
              index === /** @satisfies {HalvingEpoch} */ (4),
            minBarSpacing:
              index === /** @satisfies {MonthIndex} */ (7)
                ? 1
                : index === /** @satisfies {QuarterIndex} */ (19)
                  ? 3
                  : index === /** @satisfies {YearIndex} */ (23)
                    ? 12
                    : index === /** @satisfies {DecadeIndex} */ (1)
                      ? 120
                      : undefined,
          },
          crosshair: {
            horzLine: {
              color: offColor,
              labelBackgroundColor: defaultColor,
            },
            vertLine: {
              color: offColor,
              labelBackgroundColor: defaultColor,
            },
            mode: 3,
          },
        });
      },
    );

    return chart;
  }

  /**
   * @param {Object} args
   * @param {string} args.id
   * @param {HTMLElement} args.parent
   * @param {Signals} args.signals
   * @param {Colors} args.colors
   * @param {Utilities} args.utils
   * @param {Elements} args.elements
   * @param {VecsResources} args.vecsResources
   * @param {Owner | null} [args.owner]
   * @param {true} [args.fitContentOnResize]
   * @param {{unit: Unit; blueprints: AnySeriesBlueprint[]}[]} [args.config]
   */
  function createChartElement({
    parent,
    signals,
    colors,
    utils,
    elements,
    id,
    vecsResources,
    owner: _owner,
    fitContentOnResize,
    config,
  }) {
    let owner = _owner || signals.getOwner();

    const div = window.document.createElement("div");
    div.classList.add("chart");
    parent.append(div);

    const legendTop = createLegend({
      parent: div,
      utils,
      signals,
      paneIndex: 0,
    });

    const chartDiv = window.document.createElement("div");
    chartDiv.classList.add("lightweight-chart");
    div.append(chartDiv);

    const legendBottom = createLegend({
      parent: div,
      utils,
      signals,
      paneIndex: 1,
    });

    let ichart = /** @type {IChartApi | null} */ (null);
    let timeScaleSet = false;

    if (fitContentOnResize) {
      new ResizeObserver(() => ichart?.timeScale().fitContent()).observe(
        chartDiv,
      );
    }

    /** @type {Index} */
    let vecIndex = 0; // Default value, overwritten

    let timeResource = /** @type {VecResource| null} */ (null);

    let timeScaleSetCallback =
      /** @type {((unknownTimeScaleCallback: VoidFunction) => void) | null} */ (
        null
      );

    /**
     * @param {ISeriesApi<SeriesType>} series
     * @param {VecResource} valuesResource
     */
    function createSetFetchedDataEffect(series, valuesResource) {
      const fetchedKey = vecsResources.defaultFetchedKey;

      signals.runWithOwner(owner, () =>
        signals.createEffect(
          () => [
            timeResource?.fetched[fetchedKey].vec(),
            valuesResource.fetched[fetchedKey].vec(),
          ],
          ([indexes, _ohlcs]) => {
            if (!ichart) throw Error("IChart should be initialized");

            if (!indexes || !_ohlcs) return;
            const ohlcs = /** @type {OHLCTuple[]} */ (_ohlcs);
            let length = Math.min(indexes.length, ohlcs.length);
            const data = new Array(length);
            let prevTime = null;
            let offset = 0;

            for (let i = 0; i < length; i++) {
              const time = indexes[i];
              if (prevTime === time) {
                offset += 1;
              }
              const v = ohlcs[i];
              if (v === null) {
                data[i - offset] = {
                  time,
                  value: NaN,
                };
              } else if (typeof v === "number") {
                data[i - offset] = {
                  time,
                  value: v,
                };
              } else {
                data[i - offset] = {
                  time,
                  open: v[0],
                  high: v[1],
                  low: v[2],
                  close: v[3],
                };
              }
              prevTime = time;
            }

            data.length -= offset;
            series.setData(data);

            timeScaleSetCallback?.(() => {
              if (
                !timeScaleSet &&
                (vecIndex === /** @satisfies {QuarterIndex} */ (19) ||
                  vecIndex === /** @satisfies {YearIndex} */ (23) ||
                  vecIndex === /** @satisfies {DecadeIndex} */ (1))
              ) {
                ichart
                  ?.timeScale()
                  .setVisibleLogicalRange({ from: -1, to: data.length });
              }
            });
            timeScaleSet = true;
          },
        ),
      );
    }

    const activeResources = /** @type {VecResource[]} */ ([]);

    ichart?.subscribeCrosshairMove(
      utils.debounce(() => {
        activeResources.forEach((v) => {
          v.fetch();
        });
      }),
    );

    const chart = {
      inner: () => ichart,
      /**
       * @param {Object} args
       * @param {Index} args.index
       * @param {((unknownTimeScaleCallback: VoidFunction) => void)} [args.timeScaleSetCallback]
       */
      create({ index: _index, timeScaleSetCallback: _timeScaleSetCallback }) {
        vecIndex = _index;
        timeScaleSetCallback = _timeScaleSetCallback || null;

        if (ichart) throw Error("IChart shouldn't be initialized");

        timeResource = vecsResources.getOrCreate(
          vecIndex,
          vecIndex === /** @satisfies {Height} */ (5)
            ? "timestamp-fixed"
            : "timestamp",
        );
        timeResource.fetch();

        ichart = createLightweightChart({
          index: vecIndex,
          element: chartDiv,
          signals,
          colors,
          utils,
          elements,
        });

        if (fitContentOnResize) {
          ichart.applyOptions({
            handleScroll: false,
            handleScale: false,
            timeScale: {
              minBarSpacing: 0.001,
            },
          });
        }
      },
      /**
       * @param {Object} args
       * @param {string} args.name
       * @param {Unit} args.unit
       * @param {VecId} [args.vecId]
       * @param {Accessor<CandlestickData[]>} [args.data]
       * @param {number} [args.paneIndex]
       * @param {boolean} [args.defaultActive]
       * @param {boolean} [args.inverse]
       */
      addCandlestickSeries({
        vecId,
        name,
        unit,
        paneIndex: _paneIndex,
        defaultActive,
        data,
        inverse,
      }) {
        const paneIndex = _paneIndex ?? 0;

        if (!ichart || !timeResource) throw Error("Chart not fully set");

        const green = inverse ? colors.red() : colors.green();
        const red = inverse ? colors.green() : colors.red();
        const series = ichart.addSeries(
          /** @type {SeriesDefinition<'Candlestick'>} */ (lc.CandlestickSeries),
          {
            upColor: green,
            downColor: red,
            wickUpColor: green,
            wickDownColor: red,
            borderVisible: false,
            visible: defaultActive !== false,
          },
          paneIndex,
        );

        let url = /** @type {string | undefined} */ (undefined);

        if (vecId) {
          const valuesResource = vecsResources.getOrCreate(vecIndex, vecId);
          valuesResource.fetch();
          activeResources.push(valuesResource);
          createSetFetchedDataEffect(series, valuesResource);

          url = valuesResource.url;
        } else if (data) {
          signals.runWithOwner(owner, () =>
            signals.createEffect(data, (data) => {
              series.setData(data);
            }),
          );
        }

        (paneIndex ? legendBottom : legendTop).add({
          series,
          name,
          defaultActive,
          colors: [colors.green, colors.red],
          url,
        });

        createPaneHeightObserver({
          ichart,
          paneIndex,
          signals,
          utils,
        });

        this.addPriceScaleSelectorIfNeeded({
          paneIndex,
          seriesType: "Candlestick",
          id: `${id}-${paneIndex}`,
          unit,
        });

        return series;
      },
      /**
       * @param {Object} args
       * @param {string} args.name
       * @param {Unit} args.unit
       * @param {Accessor<LineData[]>} [args.data]
       * @param {VecId} [args.vecId]
       * @param {Color} [args.color]
       * @param {number} [args.paneIndex]
       * @param {boolean} [args.defaultActive]
       * @param {DeepPartial<LineStyleOptions & SeriesOptionsCommon & CreatePriceLineOptions>} [args.options]
       */
      addLineSeries({
        vecId,
        name,
        unit,
        color,
        paneIndex: _paneIndex,
        defaultActive,
        data,
        options,
      }) {
        if (!ichart || !timeResource) throw Error("Chart not fully set");

        const paneIndex = _paneIndex ?? 0;

        color ||= unit === "USD" ? colors.green : colors.orange;

        const series = ichart.addSeries(
          /** @type {SeriesDefinition<'Line'>} */ (lc.LineSeries),
          {
            lineWidth: /** @type {any} */ (1.5),
            visible: defaultActive !== false,
            priceLineVisible: false,
            color: color(),
            ...options,
          },
          paneIndex,
        );

        const priceLineOptions = options?.createPriceLine;
        if (priceLineOptions) {
          createPriceLine(series, priceLineOptions, colors);
        }

        let url = /** @type {string | undefined} */ (undefined);

        if (vecId) {
          const valuesResource = vecsResources.getOrCreate(vecIndex, vecId);
          valuesResource.fetch();
          activeResources.push(valuesResource);
          createSetFetchedDataEffect(series, valuesResource);
          url = valuesResource.url;
        } else if (data) {
          signals.runWithOwner(owner, () =>
            signals.createEffect(data, (data) => {
              series.setData(data);
              ichart
                ?.timeScale()
                .setVisibleLogicalRange({ from: -1, to: data.length });
            }),
          );
        }

        (paneIndex ? legendBottom : legendTop).add({
          series,
          colors: [color],
          name,
          defaultActive,
          url,
        });

        createPaneHeightObserver({
          ichart,
          paneIndex,
          signals,
          utils,
        });

        this.addPriceScaleSelectorIfNeeded({
          paneIndex,
          seriesType: "Line",
          id: `${id}-${paneIndex}`,
          unit,
        });

        return series;
      },
      /**
       * @param {Object} args
       * @param {string} args.name
       * @param {Unit} args.unit
       * @param {Accessor<BaselineData[]>} [args.data]
       * @param {VecId} [args.vecId]
       * @param {number} [args.paneIndex]
       * @param {boolean} [args.defaultActive]
       * @param {DeepPartial<BaselineStyleOptions & SeriesOptionsCommon & CreatePriceLineOptions>} [args.options]
       */
      addBaselineSeries({
        vecId,
        name,
        unit,
        paneIndex: _paneIndex,
        defaultActive,
        data,
        options,
      }) {
        if (!ichart || !timeResource) throw Error("Chart not fully set");

        const paneIndex = _paneIndex ?? 0;

        console.log("OPTIONS", options);

        const series = ichart.addSeries(
          /** @type {SeriesDefinition<'Baseline'>} */ (lc.BaselineSeries),
          {
            lineWidth: /** @type {any} */ (1.5),
            visible: defaultActive !== false,
            baseValue: {
              price: 0,
            },
            ...options,
            topLineColor: options?.topLineColor ?? colors.green(),
            bottomLineColor: options?.bottomLineColor ?? colors.red(),
            priceLineVisible: false,
            bottomFillColor1: "transparent",
            bottomFillColor2: "transparent",
            topFillColor1: "transparent",
            topFillColor2: "transparent",
            lineVisible: true,
          },
          paneIndex,
        );

        const priceLineOptions = options?.createPriceLine;
        if (priceLineOptions) {
          createPriceLine(series, priceLineOptions, colors);
        }

        let url = /** @type {string | undefined} */ (undefined);

        if (vecId) {
          const valuesResource = vecsResources.getOrCreate(vecIndex, vecId);
          valuesResource.fetch();
          activeResources.push(valuesResource);
          createSetFetchedDataEffect(series, valuesResource);

          url = valuesResource.url;
        } else if (data) {
          signals.runWithOwner(owner, () =>
            signals.createEffect(data, (data) => {
              series.setData(data);
              ichart
                ?.timeScale()
                .setVisibleLogicalRange({ from: -1, to: data.length });
            }),
          );
        }

        (paneIndex ? legendBottom : legendTop).add({
          series,
          colors: [
            () => options?.topLineColor ?? colors.green(),
            () => options?.bottomLineColor ?? colors.red(),
          ],
          name,
          defaultActive,
          url,
        });

        createPaneHeightObserver({
          ichart,
          paneIndex,
          signals,
          utils,
        });

        this.addPriceScaleSelectorIfNeeded({
          paneIndex,
          seriesType: "Baseline",
          id: `${id}-${paneIndex}`,
          unit,
        });

        return series;
      },
      /**
       * @param {Object} args
       * @param {Unit} args.unit
       * @param {string} args.id
       * @param {SeriesType} args.seriesType
       * @param {number} args.paneIndex
       */
      addPriceScaleSelectorIfNeeded({ unit, paneIndex, id, seriesType }) {
        id = `${id}-scale`;

        this.addFieldsetIfNeeded({
          id,
          paneIndex,
          position: "sw",
          createChild({ owner, pane }) {
            const { field, selected } = utils.dom.createHorizontalChoiceField({
              choices: /** @type {const} */ (["lin", "log"]),
              id: utils.stringToId(`${id} ${unit}`),
              defaultValue:
                unit === "USD" && seriesType !== "Baseline" ? "log" : "lin",
              key: `${id}-price-scale-${paneIndex}`,
              signals,
            });

            signals.runWithOwner(owner, () => {
              signals.createEffect(selected, (selected) => {
                try {
                  pane.priceScale("right").applyOptions({
                    mode: selected === "lin" ? 0 : 1,
                  });
                } catch {}
              });
            });

            return field;
          },
        });
      },
      /**
       * @param {Object} args
       * @param {string} args.id
       * @param {number} args.paneIndex
       * @param {"nw" | "ne" | "se" | "sw"} args.position
       * @param {number} [args.timeout]
       * @param {(args: {owner: Owner | null, pane: IPaneApi<Time>}) => HTMLElement} args.createChild
       */
      addFieldsetIfNeeded({ paneIndex, id, position, createChild }) {
        const owner = signals.getOwner();
        setTimeout(
          () => {
            const parent = ichart
              ?.panes()
              .at(paneIndex)
              ?.getHTMLElement()
              .children?.item(1)?.firstChild;

            if (!parent) throw Error("Parent should exist");

            if (
              Array.from(parent.childNodes).filter(
                (element) =>
                  /** @type {HTMLElement} */ (element).dataset.position ===
                  position,
              ).length
            ) {
              return;
            }

            const fieldset = window.document.createElement("fieldset");
            fieldset.dataset.size = "xs";
            fieldset.dataset.position = position;
            fieldset.id = `${id}-${paneIndex}`;
            const pane = ichart?.panes().at(paneIndex);
            if (!pane) throw Error("Expect pane");
            pane
              .getHTMLElement()
              .children?.item(1)
              ?.firstChild?.appendChild(fieldset);

            fieldset.append(createChild({ owner, pane }));
          },
          paneIndex ? 50 : 0,
        );
      },
      /**
       *
       * @param {Object} args
       * @param {Owner | null} args.owner
       */
      reset({ owner: _owner }) {
        owner = _owner;
        ichart?.remove();
        ichart = null;
        timeScaleSet = false;
        activeResources.length = 0;
        legendTop.reset();
        legendBottom.reset();
      },
    };

    config?.forEach(({ unit, blueprints }, paneIndex) => {
      chart.create({ index: /** @satisfies {DateIndex} */ (0) });

      blueprints.forEach((blueprint) => {
        if (blueprint.type === "Candlestick") {
          chart.addCandlestickSeries({
            name: blueprint.title,
            unit,
            data: blueprint.data,
            defaultActive: blueprint.defaultActive,
            paneIndex,
          });
        } else if (blueprint.type === "Baseline") {
          chart.addBaselineSeries({
            name: blueprint.title,
            unit,
            data: blueprint.data,
            defaultActive: blueprint.defaultActive,
            paneIndex,
          });
        } else {
          chart.addLineSeries({
            name: blueprint.title,
            unit,
            data: blueprint.data,
            defaultActive: blueprint.defaultActive,
            paneIndex,
            color: blueprint.color,
          });
        }
      });
    });

    return chart;
  }

  return {
    inner: lc,
    createChartElement,
  };
});

/**
 * @param {Object} args
 * @param {Element} args.parent
 * @param {Signals} args.signals
 * @param {number} args.paneIndex
 * @param {Utilities} args.utils
 */
function createLegend({ parent, signals, utils, paneIndex }) {
  const legendElement = window.document.createElement("legend");
  parent.append(legendElement);

  const hovered = signals.createSignal(
    /** @type {ISeriesApi<SeriesType> | null} */ (null),
  );

  return {
    /**
     * @param {Object} args
     * @param {ISeriesApi<SeriesType>} args.series
     * @param {string} args.name
     * @param {Color[]} args.colors
     * @param {boolean} [args.defaultActive]
     * @param {string} [args.url]
     */
    add({ series, name, colors, defaultActive, url }) {
      const div = window.document.createElement("div");

      legendElement.append(div);

      const nameId = `${utils.stringToId(name)}-${paneIndex}`;

      const active = signals.createSignal(defaultActive ?? true, {
        save: {
          keyPrefix: "",
          key: nameId,
          ...utils.serde.boolean,
        },
      });

      signals.createEffect(active, (active) => {
        series.applyOptions({
          visible: active,
        });
      });

      const { input, label } = utils.dom.createLabeledInput({
        inputId: utils.stringToId(`legend-${nameId}`),
        inputName: utils.stringToId(`selected-${nameId}`),
        inputValue: "value",
        labelTitle: "Click to toggle",
        inputChecked: active(),
        onClick: () => {
          active.set(input.checked);
        },
        type: "checkbox",
      });

      const spanMain = window.document.createElement("span");
      spanMain.classList.add("main");
      label.append(spanMain);

      const spanName = utils.dom.createSpanName(name);
      spanMain.append(spanName);

      div.append(label);
      label.addEventListener("mouseover", () => {
        const h = hovered();
        if (!h || h !== series) {
          hovered.set(series);
        }
      });
      label.addEventListener("mouseleave", () => {
        hovered.set(null);
      });

      function shouldHighlight() {
        const h = hovered();
        return !h || h === series;
      }

      /**
       * @param {string} color
       */
      function tameColor(color) {
        return `${color.slice(0, -1)} / 50%)`;
      }

      const spanColors = window.document.createElement("span");
      spanColors.classList.add("colors");
      spanMain.prepend(spanColors);
      colors.forEach((color) => {
        const spanColor = window.document.createElement("span");
        spanColors.append(spanColor);

        signals.createEffect(
          () => ({
            color: color(),
            shouldHighlight: shouldHighlight(),
          }),
          ({ color, shouldHighlight }) => {
            if (shouldHighlight) {
              spanColor.style.backgroundColor = color;
            } else {
              spanColor.style.backgroundColor = tameColor(color);
            }
          },
        );
      });

      const initialColors = /** @type {Record<string, any>} */ ({});
      const darkenedColors = /** @type {Record<string, any>} */ ({});

      const seriesOptions = series.options();
      if (!seriesOptions) return;

      Object.entries(seriesOptions).forEach(([k, v]) => {
        if (k.toLowerCase().includes("color") && typeof v === "string") {
          if (!v.startsWith("oklch")) return;
          initialColors[k] = v;
          darkenedColors[k] = tameColor(v);
        } else if (k === "lastValueVisible" && v) {
          initialColors[k] = true;
          darkenedColors[k] = false;
        }
      });

      signals.createEffect(shouldHighlight, (shouldHighlight) => {
        if (shouldHighlight) {
          series.applyOptions(initialColors);
        } else {
          series.applyOptions(darkenedColors);
        }
      });

      if (url) {
        const anchor = window.document.createElement("a");
        anchor.href = url;
        anchor.target = "_blank";
        anchor.rel = "noopener noreferrer";
        anchor.title = "Click to view data";
        div.append(anchor);
      }
    },
    reset() {
      legendElement.innerHTML = "";
    },
  };
}

function createOklchToRGBA() {
  {
    /**
     *
     * @param {readonly [number, number, number, number, number, number, number, number, number]} A
     * @param {readonly [number, number, number]} B
     * @returns
     */
    function multiplyMatrices(A, B) {
      return /** @type {const} */ ([
        A[0] * B[0] + A[1] * B[1] + A[2] * B[2],
        A[3] * B[0] + A[4] * B[1] + A[5] * B[2],
        A[6] * B[0] + A[7] * B[1] + A[8] * B[2],
      ]);
    }
    /**
     * @param {readonly [number, number, number]} param0
     */
    function oklch2oklab([l, c, h]) {
      return /** @type {const} */ ([
        l,
        isNaN(h) ? 0 : c * Math.cos((h * Math.PI) / 180),
        isNaN(h) ? 0 : c * Math.sin((h * Math.PI) / 180),
      ]);
    }
    /**
     * @param {readonly [number, number, number]} rgb
     */
    function srgbLinear2rgb(rgb) {
      return rgb.map((c) =>
        Math.abs(c) > 0.0031308
          ? (c < 0 ? -1 : 1) * (1.055 * Math.abs(c) ** (1 / 2.4) - 0.055)
          : 12.92 * c,
      );
    }
    /**
     * @param {readonly [number, number, number]} lab
     */
    function oklab2xyz(lab) {
      const LMSg = multiplyMatrices(
        /** @type {const} */ ([
          1, 0.3963377773761749, 0.2158037573099136, 1, -0.1055613458156586,
          -0.0638541728258133, 1, -0.0894841775298119, -1.2914855480194092,
        ]),
        lab,
      );
      const LMS = /** @type {[number, number, number]} */ (
        LMSg.map((val) => val ** 3)
      );
      return multiplyMatrices(
        /** @type {const} */ ([
          1.2268798758459243, -0.5578149944602171, 0.2813910456659647,
          -0.0405757452148008, 1.112286803280317, -0.0717110580655164,
          -0.0763729366746601, -0.4214933324022432, 1.5869240198367816,
        ]),
        LMS,
      );
    }
    /**
     * @param {readonly [number, number, number]} xyz
     */
    function xyz2rgbLinear(xyz) {
      return multiplyMatrices(
        [
          3.2409699419045226, -1.537383177570094, -0.4986107602930034,
          -0.9692436362808796, 1.8759675015077202, 0.04155505740717559,
          0.05563007969699366, -0.20397695888897652, 1.0569715142428786,
        ],
        xyz,
      );
    }

    /** @param {string} oklch */
    return function (oklch) {
      oklch = oklch.replace("oklch(", "");
      oklch = oklch.replace(")", "");
      let splitOklch = oklch.split(" / ");
      let alpha = 1;
      if (splitOklch.length === 2) {
        alpha = Number(splitOklch.pop()?.replace("%", "")) / 100;
      }
      splitOklch = oklch.split(" ");
      const lch = splitOklch.map((v, i) => {
        if (!i && v.includes("%")) {
          return Number(v.replace("%", "")) / 100;
        } else {
          return Number(v);
        }
      });
      const rgb = srgbLinear2rgb(
        xyz2rgbLinear(
          oklab2xyz(oklch2oklab(/** @type {[number, number, number]} */ (lch))),
        ),
      ).map((v) => {
        return Math.max(Math.min(Math.round(v * 255), 255), 0);
      });
      return [...rgb, alpha];
    };
  }
}

/**
 * @param {Object} args
 * @param {IChartApi} args.ichart
 * @param {number} args.paneIndex
 * @param {Signals} args.signals
 * @param {Utilities} args.utils
 */
function createPaneHeightObserver({ ichart, paneIndex, signals, utils }) {
  if (!paneIndex) return;

  const owner = signals.getOwner();

  const one = "1";

  const callback = () =>
    setTimeout(() => {
      try {
        const _element = ichart?.panes().at(paneIndex)?.getHTMLElement();
        if (!_element) return callback();
        const element = _element;

        if (element.dataset.observed === one) return;
        element.dataset.observed = one;

        signals.runWithOwner(owner, () => {
          const height = signals.createSignal(null, {
            save: {
              keyPrefix: "charts",
              key: `height-${paneIndex}`,
              ...utils.serde.optNumber,
            },
          });

          const savedHeight = height();
          if (savedHeight !== null) {
            ichart.panes().at(paneIndex)?.setHeight(savedHeight);
          }

          let firstRun = true;
          new ResizeObserver(() => {
            if (firstRun && savedHeight !== null) {
              firstRun = false;
            } else {
              const h = ichart.panes().at(paneIndex)?.getHeight();
              if (h === undefined) return;
              height.set(h);
            }
          }).observe(element);
        });
      } catch {
        callback();
      }
    }, 5);

  callback();
}

/**
 * @param {ISeriesApi<SeriesType>} series
 * @param {DeepPartial<CreatePriceLine>} options
 * @param {Colors} colors
 */
function createPriceLine(series, options, colors) {
  series.createPriceLine({
    price: options.value || 0,
    color: colors.gray(),
    axisLabelVisible: false,
    lineWidth: 1,
    lineStyle: 4,
  });
}
