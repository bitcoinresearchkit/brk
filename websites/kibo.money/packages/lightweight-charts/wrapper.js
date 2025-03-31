// @ts-check

/** @import {SeriesDefinition} from './v5.0.5/types' */

export default import("./v5.0.5/script.js").then((lc) => {
  /**
   * @param {Object} args
   * @param {HTMLElement} args.element
   * @param {Signals} args.signals
   * @param {Colors} args.colors
   * @param {Index} args.index
   * @param {Utilities} args.utils
   * @param {DeepPartial<ChartOptions>} [args.options]
   */
  function createLightweightChart({
    element,
    signals,
    colors,
    index,
    utils,
    options: _options = {},
  }) {
    /** @satisfies {DeepPartial<ChartOptions>} */
    const options = {
      autoSize: true,
      layout: {
        fontFamily: "Satoshi",
        fontSize: 14,
        background: { color: "transparent" },
        attributionLogo: false,
        colorSpace: "display-p3",
        colorParsers: [oklchToRGBA],
      },
      grid: {
        vertLines: { visible: false },
        horzLines: { visible: false },
      },
      timeScale: {
        minBarSpacing: 2.1,
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
      scaleMargins: {
        top: 0.075,
        bottom: 0.05,
      },
      minimumWidth: 78,
    });

    signals.createEffect(
      () => ({
        defaultColor: colors.default(),
        offColor: colors.off(),
        borderColor: colors.border(),
      }),
      ({ defaultColor, offColor, borderColor }) => {
        console.log(
          defaultColor,
          offColor,
          borderColor,
          `rgba(${oklchToRGBA(borderColor).join(", ")})`,
        );

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
            timeVisible: index === /** @satisfies {Height} */ (2),
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
   * @type {DeepPartial<SeriesOptionsCommon>}
   */
  const defaultSeriesOptions = {
    // @ts-ignore
    lineWidth: 1.5,
    // priceLineVisible: false,
    // baseLineVisible: false,
    // baseLineColor: "",
  };

  /**
   * @param {Object} args
   * @param {string} args.id
   * @param {HTMLElement} args.parent
   * @param {Signals} args.signals
   * @param {Colors} args.colors
   * @param {"static" | "scrollable"} args.kind
   * @param {Utilities} args.utils
   * @param {VecsResources} args.vecsResources
   * @param {Owner | null} [args.owner]
   */
  function createChartElement({
    parent,
    signals,
    colors,
    kind,
    utils,
    vecsResources,
    owner: _owner,
  }) {
    let owner = _owner || signals.getOwner();

    const div = window.document.createElement("div");
    div.classList.add("chart");
    parent.append(div);

    const legend = createLegend({
      parent: div,
    });

    const chartDiv = window.document.createElement("div");
    chartDiv.classList.add("lightweight-chart");
    div.append(chartDiv);

    let ichart = /** @type {IChartApi | null} */ (null);
    let timeScaleSet = false;

    if (kind === "static") {
      new ResizeObserver(() => ichart?.timeScale().fitContent()).observe(
        chartDiv,
      );
    }

    /** @type {Index} */
    let index = 0;

    let timeResource = /** @type {VecResource| null} */ (null);

    /**
     * @param {ISeriesApi<any>} series
     * @param {VecResource} valuesResource
     */
    function createSetDataEffect(series, valuesResource) {
      signals.createEffect(
        () => [timeResource?.fetched(), valuesResource.fetched()],
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
            if (prevTime && prevTime === time) {
              offset += 1;
            }
            const v = ohlcs[i];
            if (typeof v === "number") {
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
          if (
            !timeScaleSet &&
            (index === /** @satisfies {Yearindex} */ (15) ||
              index === /** @satisfies {Decadeindex} */ (16))
          ) {
            ichart
              .timeScale()
              .setVisibleLogicalRange({ from: -1, to: data.length });
          }
          timeScaleSet = true;
        },
      );
    }

    return {
      /**
       * @param {Index} _index
       */
      create(_index) {
        index = _index;
        if (ichart) throw Error("IChart shouldn't be initialized");

        timeResource = vecsResources.getOrCreate(
          index,
          index === /** @satisfies {Height} */ (2)
            ? "fixed-timestamp"
            : "timestamp",
        );
        timeResource.fetch();

        ichart = createLightweightChart({
          index,
          element: chartDiv,
          signals,
          colors,
          utils,
        });
      },
      /**
       * @param {VecId} id
       * @param {number} [paneNumber]
       */
      addCandlestickSeries(id, paneNumber) {
        if (!ichart || !timeResource) throw Error("Chart not fully set");

        const valuesResource = vecsResources.getOrCreate(index, id);
        valuesResource.fetch();

        const green = colors.green();
        const red = colors.red();
        const series = ichart.addSeries(
          /** @type {SeriesDefinition<'Candlestick'>} */ (lc.CandlestickSeries),
          {
            upColor: green,
            downColor: red,
            wickUpColor: green,
            wickDownColor: red,
            borderVisible: false,
          },
          paneNumber,
        );

        createSetDataEffect(series, valuesResource);

        return series;
      },
      /**
       * @param {VecId} id
       * @param {number} [paneNumber]
       */
      addLineSeries(id, paneNumber) {
        if (!ichart || !timeResource) throw Error("Chart not fully set");

        const valuesResource = vecsResources.getOrCreate(index, id);
        valuesResource.fetch();

        const series = ichart.addSeries(
          /** @type {SeriesDefinition<'Line'>} */ (lc.LineSeries),
          {
            lineWidth: /** @type {any} */ (1.5),
          },
          paneNumber,
        );

        createSetDataEffect(series, valuesResource);

        return series;
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
        legend.reset();
      },
    };
  }

  return {
    inner: lc,
    createChartElement,
  };
});

/**
 * @param {Object} args
 * @param {Element} args.parent
 */
function createLegend({ parent }) {
  const legendElement = window.document.createElement("legend");
  parent.append(legendElement);

  return {
    reset() {
      legendElement.innerHTML = "";
    },
  };
}

const oklchToRGBA = (() => {
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
      const lch = oklch.split(" ").map((v, i) => {
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
      return [...rgb, 1];
    };
  }
})();
