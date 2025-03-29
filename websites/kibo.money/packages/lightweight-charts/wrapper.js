// @ts-check

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
        fontSize: 13.5,
        background: { color: "transparent" },
        attributionLogo: false,
        colorSpace: "display-p3",
        colorParsers: [oklchToRGBA],
        panes: {},
      },
      grid: {
        vertLines: { visible: false },
        horzLines: { visible: false },
      },
      timeScale: {
        minBarSpacing: 0.1,
        shiftVisibleRangeOnNewBar: false,
        allowShiftVisibleRangeOnWhitespaceReplacement: false,
      },
      handleScale: {
        axisDoubleClickReset: {
          time: false,
        },
      },
      localization: {
        priceFormatter: utils.locale.numberToShortUSFormat,
        locale: "en-us",
      },
      ..._options,
    };

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
      }),
      ({ defaultColor, offColor }) => {
        console.log(defaultColor, offColor);

        chart.applyOptions({
          layout: {
            textColor: offColor,
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
              color: defaultColor,
              labelBackgroundColor: defaultColor,
            },
            vertLine: {
              color: defaultColor,
              labelBackgroundColor: defaultColor,
            },
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
    priceLineVisible: false,
    baseLineVisible: false,
    baseLineColor: "",
  };

  /**
   * @param {Object} args
   * @param {string} args.id
   * @param {HTMLElement} args.parent
   * @param {Signals} args.signals
   * @param {Colors} args.colors
   * @param {"static" | "scrollable"} args.kind
   * @param {Utilities} args.utils
   * @param {Owner | null} [args.owner]
   */
  function createChartElement({
    parent,
    signals,
    colors,
    id: chartId,
    kind,
    utils,
    owner: _owner,
  }) {
    let owner = _owner || signals.getOwner();

    const div = window.document.createElement("div");
    div.classList.add("chart");
    parent.append(div);

    const legendElement = window.document.createElement("legend");
    div.append(legendElement);

    const chartDiv = window.document.createElement("div");
    chartDiv.classList.add("lightweight-chart");
    div.append(chartDiv);

    let ichart = /** @type {IChartApi | null} */ (null);

    if (kind === "static") {
      new ResizeObserver(() => ichart?.timeScale().fitContent()).observe(
        chartDiv,
      );
    }

    return {
      chart() {
        return ichart;
      },
      /**
       * @param {Index} index
       */
      createChart(index) {
        if (ichart) throw Error("IChart shouldn't be initialized");

        createLightweightChart({
          index,
          element: chartDiv,
          signals,
          colors,
          utils,
        });
      },
      /**
       *
       * @param {Object} args
       * @param {Owner | null} args.owner
       */
      reset({ owner: _owner }) {
        owner = _owner;
        if (ichart !== null) {
          ichart.remove();
        } else {
          throw Error("IChart should be initialized");
        }
        legendElement.innerHTML = "";
      },
    };
  }

  return {
    createChartElement,
  };
});

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
