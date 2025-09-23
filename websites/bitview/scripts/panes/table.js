import { randomFromArray } from "../core/array";
import { createButtonElement, createHeader, createSelect } from "../core/dom";
import { serdeMetrics, serdeString, serdeUnit } from "../core/serde";
import { resetParams } from "../core/url";

/**
 * @param {Object} args
 * @param {MetricToIndexes} args.metricToIndexes
 * @param {Option} args.option
 * @param {Utilities} args.utils
 * @param {Signals} args.signals
 * @param {VecsResources} args.vecsResources
 */
function createTable({
  utils,
  metricToIndexes,
  signals,
  option,
  vecsResources,
}) {
  const indexToMetrics = createIndexToMetrics(metricToIndexes);

  const serializedIndexes = createSerializedIndexes();
  /** @type {SerializedIndex} */
  const defaultSerializedIndex = "height";
  const serializedIndex = /** @type {Signal<SerializedIndex>} */ (
    signals.createSignal(
      /** @type {SerializedIndex} */ (defaultSerializedIndex),
      {
        save: {
          ...serdeString,
          keyPrefix: "table",
          key: "index",
        },
      },
    )
  );
  const index = signals.createMemo(() =>
    serializedIndexToIndex(serializedIndex()),
  );

  const table = window.document.createElement("table");
  const obj = {
    element: table,
    /** @type {VoidFunction | undefined} */
    addRandomCol: undefined,
  };

  signals.createEffect(index, (index, prevIndex) => {
    if (prevIndex !== undefined) {
      resetParams(option);
    }

    const possibleMetrics = indexToMetrics[index];

    const columns = signals.createSignal(/** @type {Metric[]} */ ([]), {
      equals: false,
      save: {
        ...serdeMetrics,
        keyPrefix: `table-${serializedIndex()}`,
        key: `columns`,
      },
    });
    columns.set((l) => l.filter((id) => possibleMetrics.includes(id)));

    signals.createEffect(columns, (columns) => {
      console.log(columns);
    });

    table.innerHTML = "";
    const thead = window.document.createElement("thead");
    table.append(thead);
    const trHead = window.document.createElement("tr");
    thead.append(trHead);
    const tbody = window.document.createElement("tbody");
    table.append(tbody);

    const rowElements = signals.createSignal(
      /** @type {HTMLTableRowElement[]} */ ([]),
    );

    /**
     * @param {Object} args
     * @param {HTMLSelectElement} args.select
     * @param {Unit} [args.unit]
     * @param {(event: MouseEvent) => void} [args.onLeft]
     * @param {(event: MouseEvent) => void} [args.onRight]
     * @param {(event: MouseEvent) => void} [args.onRemove]
     */
    function addThCol({ select, onLeft, onRight, onRemove, unit: _unit }) {
      const th = window.document.createElement("th");
      th.scope = "col";
      trHead.append(th);
      const div = window.document.createElement("div");
      div.append(select);
      // const top = window.document.createElement("div");
      // div.append(top);
      // top.append(select);
      // top.append(
      //   createAnchorElement({
      //     href: "",
      //     blank: true,
      //   }),
      // );
      const bottom = window.document.createElement("div");
      const unit = window.document.createElement("span");
      if (_unit) {
        unit.innerHTML = _unit;
      }
      const moveLeft = createButtonElement({
        inside: "←",
        title: "Move column to the left",
        onClick: onLeft || (() => {}),
      });
      const moveRight = createButtonElement({
        inside: "→",
        title: "Move column to the right",
        onClick: onRight || (() => {}),
      });
      const remove = createButtonElement({
        inside: "×",
        title: "Remove column",
        onClick: onRemove || (() => {}),
      });
      bottom.append(unit);
      bottom.append(moveLeft);
      bottom.append(moveRight);
      bottom.append(remove);
      div.append(bottom);
      th.append(div);
      return {
        element: th,
        /**
         * @param {Unit} _unit
         */
        setUnit(_unit) {
          unit.innerHTML = _unit;
        },
      };
    }

    addThCol({
      ...createSelect({
        list: serializedIndexes,
        signal: serializedIndex,
      }),
      unit: "index",
    });

    let from = 0;
    let to = 0;

    vecsResources
      .getOrCreate(index, serializedIndex())
      .fetch()
      .then((vec) => {
        if (!vec) return;
        from = /** @type {number} */ (vec[0]);
        to = /** @type {number} */ (vec.at(-1)) + 1;
        const trs = /** @type {HTMLTableRowElement[]} */ ([]);
        for (let i = vec.length - 1; i >= 0; i--) {
          const value = vec[i];
          const tr = window.document.createElement("tr");
          trs.push(tr);
          tbody.append(tr);
          const th = window.document.createElement("th");
          th.innerHTML = serializeValue({
            value,
            unit: "index",
          });
          th.scope = "row";
          tr.append(th);
        }
        rowElements.set(() => trs);
      });

    const owner = signals.getOwner();

    /**
     * @param {Metric} metric
     * @param {number} [_colIndex]
     */
    function addCol(metric, _colIndex = columns().length) {
      signals.runWithOwner(owner, () => {
        /** @type {VoidFunction | undefined} */
        let dispose;
        signals.createRoot((_dispose) => {
          dispose = _dispose;

          const metricOption = signals.createSignal({
            name: metric,
            value: metric,
          });
          const { select } = createSelect({
            list: possibleMetrics.map((metric) => ({
              name: metric,
              value: metric,
            })),
            signal: metricOption,
          });

          signals.createEffect(metricOption, (metricOption) => {
            select.style.width = `${21 + 7.25 * metricOption.name.length}px`;
          });

          if (_colIndex === columns().length) {
            columns.set((l) => {
              l.push(metric);
              return l;
            });
          }

          const colIndex = signals.createSignal(_colIndex);

          /**
           * @param {boolean} right
           * @returns {(event: MouseEvent) => void}
           */
          function createMoveColumnFunction(right) {
            return () => {
              const oldColIndex = colIndex();
              const newColIndex = oldColIndex + (right ? 1 : -1);

              const currentTh = /** @type {HTMLTableCellElement} */ (
                trHead.childNodes[oldColIndex + 1]
              );
              const oterTh = /** @type {HTMLTableCellElement} */ (
                trHead.childNodes[newColIndex + 1]
              );

              if (right) {
                oterTh.after(currentTh);
              } else {
                oterTh.before(currentTh);
              }

              columns.set((l) => {
                [l[oldColIndex], l[newColIndex]] = [
                  l[newColIndex],
                  l[oldColIndex],
                ];
                return l;
              });

              const rows = rowElements();
              for (let i = 0; i < rows.length; i++) {
                const element = rows[i].childNodes[oldColIndex + 1];
                const sibling = rows[i].childNodes[newColIndex + 1];
                const temp = element.textContent;
                element.textContent = sibling.textContent;
                sibling.textContent = temp;
              }
            };
          }

          const th = addThCol({
            select,
            unit: serdeUnit.deserialize(metric),
            onLeft: createMoveColumnFunction(false),
            onRight: createMoveColumnFunction(true),
            onRemove: () => {
              const ci = colIndex();
              trHead.childNodes[ci + 1].remove();
              columns.set((l) => {
                l.splice(ci, 1);
                return l;
              });
              const rows = rowElements();
              for (let i = 0; i < rows.length; i++) {
                rows[i].childNodes[ci + 1].remove();
              }
              dispose?.();
            },
          });

          signals.createEffect(columns, () => {
            colIndex.set(Array.from(trHead.children).indexOf(th.element) - 1);
          });

          console.log(colIndex());

          signals.createEffect(rowElements, (rowElements) => {
            if (!rowElements.length) return;
            for (let i = 0; i < rowElements.length; i++) {
              const td = window.document.createElement("td");
              rowElements[i].append(td);
            }

            signals.createEffect(
              () => metricOption().name,
              (metric, prevMetric) => {
                const unit = serdeUnit.deserialize(metric);
                th.setUnit(unit);

                const vec = vecsResources.getOrCreate(index, metric);

                vec.fetch({ from, to });

                const fetchedKey = vecsResources.genFetchedKey({ from, to });

                columns.set((l) => {
                  const i = l.indexOf(prevMetric ?? metric);
                  if (i === -1) {
                    l.push(metric);
                  } else {
                    l[i] = metric;
                  }
                  return l;
                });

                signals.createEffect(
                  () => vec.fetched().get(fetchedKey)?.vec(),
                  (vec) => {
                    if (!vec?.length) return;

                    const thIndex = colIndex() + 1;

                    for (let i = 0; i < rowElements.length; i++) {
                      const iRev = vec.length - 1 - i;
                      const value = vec[iRev];
                      // @ts-ignore
                      rowElements[i].childNodes[thIndex].innerHTML =
                        serializeValue({
                          value,
                          unit,
                        });
                    }
                  },
                );

                return () => metric;
              },
            );
          });
        });

        signals.onCleanup(() => {
          dispose?.();
        });
      });
    }

    columns().forEach((metric, colIndex) => addCol(metric, colIndex));

    obj.addRandomCol = function () {
      addCol(randomFromArray(possibleMetrics));
    };

    return () => index;
  });

  return obj;
}

/**
 * @param {Object} args
 * @param {Signals} args.signals
 * @param {Utilities} args.utils
 * @param {Option} args.option
 * @param {Elements} args.elements
 * @param {VecsResources} args.vecsResources
 * @param {MetricToIndexes} args.metricToIndexes
 */
export function init({
  elements,
  signals,
  option,
  utils,
  vecsResources,
  metricToIndexes,
}) {
  const parent = elements.table;
  const { headerElement } = createHeader("Table");
  parent.append(headerElement);

  const div = window.document.createElement("div");
  parent.append(div);

  const table = createTable({
    signals,
    utils,
    metricToIndexes,
    vecsResources,
    option,
  });
  div.append(table.element);

  const span = window.document.createElement("span");
  span.innerHTML = "Add column";
  div.append(
    createButtonElement({
      onClick: () => {
        table.addRandomCol?.();
      },
      inside: span,
      title: "Click or tap to add a column to the table",
    }),
  );
}

function createSerializedIndexes() {
  return /** @type {const} */ ([
    /** @satisfies {Metric} */ ("dateindex"),
    /** @satisfies {Metric} */ ("decadeindex"),
    /** @satisfies {Metric} */ ("difficultyepoch"),
    /** @satisfies {Metric} */ ("emptyoutputindex"),
    /** @satisfies {Metric} */ ("halvingepoch"),
    /** @satisfies {Metric} */ ("height"),
    /** @satisfies {Metric} */ ("inputindex"),
    /** @satisfies {Metric} */ ("monthindex"),
    /** @satisfies {Metric} */ ("opreturnindex"),
    /** @satisfies {Metric} */ ("semesterindex"),
    /** @satisfies {Metric} */ ("outputindex"),
    /** @satisfies {Metric} */ ("p2aaddressindex"),
    /** @satisfies {Metric} */ ("p2msoutputindex"),
    /** @satisfies {Metric} */ ("p2pk33addressindex"),
    /** @satisfies {Metric} */ ("p2pk65addressindex"),
    /** @satisfies {Metric} */ ("p2pkhaddressindex"),
    /** @satisfies {Metric} */ ("p2shaddressindex"),
    /** @satisfies {Metric} */ ("p2traddressindex"),
    /** @satisfies {Metric} */ ("p2wpkhaddressindex"),
    /** @satisfies {Metric} */ ("p2wshaddressindex"),
    /** @satisfies {Metric} */ ("quarterindex"),
    /** @satisfies {Metric} */ ("txindex"),
    /** @satisfies {Metric} */ ("unknownoutputindex"),
    /** @satisfies {Metric} */ ("weekindex"),
    /** @satisfies {Metric} */ ("yearindex"),
    /** @satisfies {Metric} */ ("loadedaddressindex"),
    /** @satisfies {Metric} */ ("emptyaddressindex"),
  ]);
}
/** @typedef {ReturnType<typeof createSerializedIndexes>} SerializedIndexes */
/** @typedef {SerializedIndexes[number]} SerializedIndex */

/**
 * @param {SerializedIndex} serializedIndex
 * @returns {Index}
 */
function serializedIndexToIndex(serializedIndex) {
  switch (serializedIndex) {
    case "height":
      return /** @satisfies {Height} */ (5);
    case "dateindex":
      return /** @satisfies {DateIndex} */ (0);
    case "weekindex":
      return /** @satisfies {WeekIndex} */ (23);
    case "difficultyepoch":
      return /** @satisfies {DifficultyEpoch} */ (2);
    case "monthindex":
      return /** @satisfies {MonthIndex} */ (7);
    case "quarterindex":
      return /** @satisfies {QuarterIndex} */ (19);
    case "semesterindex":
      return /** @satisfies {SemesterIndex} */ (20);
    case "yearindex":
      return /** @satisfies {YearIndex} */ (24);
    case "decadeindex":
      return /** @satisfies {DecadeIndex} */ (1);
    case "halvingepoch":
      return /** @satisfies {HalvingEpoch} */ (4);
    case "txindex":
      return /** @satisfies {TxIndex} */ (21);
    case "inputindex":
      return /** @satisfies {InputIndex} */ (6);
    case "outputindex":
      return /** @satisfies {OutputIndex} */ (9);
    case "p2pk33addressindex":
      return /** @satisfies {P2PK33AddressIndex} */ (12);
    case "p2pk65addressindex":
      return /** @satisfies {P2PK65AddressIndex} */ (13);
    case "p2pkhaddressindex":
      return /** @satisfies {P2PKHAddressIndex} */ (14);
    case "p2shaddressindex":
      return /** @satisfies {P2SHAddressIndex} */ (15);
    case "p2traddressindex":
      return /** @satisfies {P2TRAddressIndex} */ (16);
    case "p2wpkhaddressindex":
      return /** @satisfies {P2WPKHAddressIndex} */ (17);
    case "p2wshaddressindex":
      return /** @satisfies {P2WSHAddressIndex} */ (18);
    case "p2aaddressindex":
      return /** @satisfies {P2AAddressIndex} */ (10);
    case "p2msoutputindex":
      return /** @satisfies {P2MSOutputIndex} */ (11);
    case "opreturnindex":
      return /** @satisfies {OpReturnIndex} */ (8);
    case "emptyoutputindex":
      return /** @satisfies {EmptyOutputIndex} */ (3);
    case "unknownoutputindex":
      return /** @satisfies {UnknownOutputIndex} */ (22);
    case "emptyaddressindex":
      return /** @satisfies {EmptyAddressIndex} */ (26);
    case "loadedaddressindex":
      return /** @satisfies {LoadedAddressIndex} */ (25);
  }
}

/**
 * @param {MetricToIndexes} metricToIndexes
 */
function createIndexToMetrics(metricToIndexes) {
  const indexToMetrics = Object.entries(metricToIndexes).reduce(
    (arr, [_id, indexes]) => {
      const id = /** @type {Metric} */ (_id);
      indexes.forEach((i) => {
        arr[i] ??= [];
        arr[i].push(id);
      });
      return arr;
    },
    /** @type {Metric[][]} */ (Array.from({ length: 24 })),
  );
  indexToMetrics.forEach((arr) => {
    arr.sort();
  });
  return indexToMetrics;
}

/**
 * @param {Object} args
 * @param {number | string | Object | Array<any>} args.value
 * @param {Unit} args.unit
 */
function serializeValue({ value, unit }) {
  const t = typeof value;
  if (value === null) {
    return "null";
  } else if (typeof value === "string") {
    return value;
  } else if (t !== "number") {
    return JSON.stringify(value).replaceAll('"', "").slice(1, -1);
  } else if (value !== 18446744073709552000) {
    if (unit === "usd" || unit === "difficulty" || unit === "sat/vb") {
      return value.toLocaleString("en-us", {
        minimumFractionDigits: 2,
        maximumFractionDigits: 2,
      });
    } else if (unit === "btc") {
      return value.toLocaleString("en-us", {
        minimumFractionDigits: 8,
        maximumFractionDigits: 8,
      });
    } else {
      return value.toLocaleString("en-us");
    }
  } else {
    return "";
  }
}
