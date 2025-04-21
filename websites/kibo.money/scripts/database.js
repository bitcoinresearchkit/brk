// @ts-check

/**
 * @param {Object} args
 * @param {VecIdToIndexes} args.vecIdToIndexes
 * @param {Utilities} args.utils
 * @param {Signals} args.signals
 * @param {VecsResources} args.vecsResources
 */
function createTable({ utils, vecIdToIndexes, signals, vecsResources }) {
  const indexToVecIds = createIndexToVecIds(vecIdToIndexes);

  const serializedIndexes = createSerializedIndexes();
  /** @type {SerializedIndex} */
  const defaultSerializedIndex = "height";
  const serializedIndex = signals.createSignal({
    name: /** @type {SerializedIndex} */ (defaultSerializedIndex),
    value: String(serializedIndexToIndex(defaultSerializedIndex)),
  });
  const index = signals.createMemo(() =>
    serializedIndexToIndex(serializedIndex().name),
  );

  const table = window.document.createElement("table");
  const obj = {
    element: table,
    /** @type {VoidFunction | undefined} */
    addRandomCol: undefined,
  };

  signals.createEffect(index, (index) => {
    table.innerHTML = "";

    const thead = window.document.createElement("thead");
    table.append(thead);
    const trHead = window.document.createElement("tr");
    const selects = signals.createSignal(
      /** @type {HTMLSelectElement[]} */ ([]),
      {
        equals: false,
      },
    );
    thead.append(trHead);
    const tbody = window.document.createElement("tbody");
    table.append(tbody);
    const rowElements = signals.createSignal(
      /** @type {HTMLTableRowElement[]} */ ([]),
    );

    /**
     * @param {Object} args
     * @param {HTMLSelectElement} args.select
     * @param {VoidFunction} [args.onLeft]
     * @param {VoidFunction} [args.onRight]
     * @param {VoidFunction} [args.onRemove]
     */
    function addThCol({ select, onLeft, onRight, onRemove }) {
      const th = window.document.createElement("th");
      th.scope = "col";
      trHead.append(th);
      const div = window.document.createElement("div");
      div.append(select);
      const strip = window.document.createElement("div");
      const unit = window.document.createElement("span");
      const moveLeft = utils.dom.createButtonElement({
        inside: "←",
        title: "Move column to the left",
        onClick: onLeft || (() => {}),
      });
      const moveRight = utils.dom.createButtonElement({
        inside: "→",
        title: "Move column to the right",
        onClick: onRight || (() => {}),
      });
      const remove = utils.dom.createButtonElement({
        inside: "×",
        title: "Remove column",
        onClick: onRemove || (() => {}),
      });
      strip.append(unit);
      strip.append(moveLeft);
      strip.append(moveRight);
      strip.append(remove);
      div.append(strip);
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

    const { select } = utils.dom.createSelect({
      id: "col-index",
      list: serializedIndexes.map((serializedIndex) => ({
        name: serializedIndex,
        value: String(serializedIndexToIndex(serializedIndex)),
      })),
      signal: serializedIndex,
    });
    const th = addThCol({ select });
    th.setUnit("Index");

    vecsResources
      .getOrCreate(index, serializedIndex().name)
      .fetch()
      .then((vec) => {
        if (!vec) return;
        const trs = /** @type {HTMLTableRowElement[]} */ ([]);
        for (let i = vec.length - 1; i >= 0; i--) {
          const value = vec[i];
          const tr = window.document.createElement("tr");
          trs.push(tr);
          tbody.append(tr);
          const th = window.document.createElement("th");
          th.innerHTML = String(value);
          th.scope = "row";
          tr.append(th);
        }
        rowElements.set(() => trs);
      });

    const columnIds = signals.createSignal(/** @type {VecId[]} */ ([]), {
      equals: false,
    });

    const owner = signals.getOwner();

    const possibleVecids = indexToVecIds[index];

    obj.addRandomCol = function () {
      signals.runWithOwner(owner, () => {
        const vecId =
          possibleVecids[Math.round(Math.random() * possibleVecids.length)];
        const colIndex = signals.createSignal(columnIds().length);

        const vecIdOption = signals.createSignal({
          name: vecId,
          value: vecId,
        });
        const { select } = utils.dom.createSelect({
          id: `col-${colIndex() + 1}`,
          list: possibleVecids.map((vecId) => ({
            name: vecId,
            value: vecId,
          })),
          signal: vecIdOption,
        });
        selects.set((l) => {
          l.push(select);
          return l;
        });
        /**
         * @param {boolean} right
         */
        function createMoveColumnFunction(right) {
          return () =>
            colIndex.set((oldI) => {
              const newI = oldI + (right ? 1 : -1);
              const currentSelect = selects()[oldI];
              const currentSelectSibling = currentSelect.nextSibling;
              const newSelect = selects()[newI];
              [selects()[oldI], selects()[newI]] = [
                selects()[newI],
                selects()[oldI],
              ];
              console.log(oldI, newI, selects());
              const newSelectSibling = newSelect.nextSibling;
              newSelectSibling?.before(currentSelect);
              currentSelectSibling?.before(newSelect);
              return newI;
            });
        }
        const th = addThCol({
          select,
          onLeft: createMoveColumnFunction(false),
          onRight: createMoveColumnFunction(true),
        });

        signals.createEffect(rowElements, (rowElements) => {
          if (!rowElements.length) return;
          for (let i = 0; i < rowElements.length; i++) {
            const td = window.document.createElement("td");
            rowElements[i].append(td);
          }

          signals.createEffect(vecIdOption, ({ name: vecId }) => {
            const unit = utils.vecidToUnit(vecId);
            th.setUnit(unit);

            const valuesPromise = vecsResources
              .getOrCreate(index, vecId)
              .fetch();

            columnIds.set((l) => {
              if (columnIds().length === colIndex()) {
                l.push(vecId);
              } else {
                l[colIndex()] = vecId;
              }
              console.log(l);
              return l;
            });

            valuesPromise.then((vec) => {
              if (!vec) return;
              // const diff = vec.length - rowElements.length;
              for (let i = 0; i < rowElements.length; i++) {
                const iRev = vec.length - 1 - i;
                const value = vec[iRev];

                /** @type {string | number | undefined} */
                let serialized;

                if (typeof value !== "number") {
                  serialized = value;
                } else if (value !== 18446744073709552000) {
                  if (
                    unit === "USD" ||
                    unit === "Difficulty" ||
                    unit === "sat/vB"
                  ) {
                    serialized = value.toLocaleString("en-us", {
                      minimumFractionDigits: 2,
                      maximumFractionDigits: 2,
                    });
                  } else if (unit === "BTC") {
                    serialized = value.toLocaleString("en-us", {
                      minimumFractionDigits: 8,
                      maximumFractionDigits: 8,
                    });
                  } else {
                    serialized = value.toLocaleString("en-us");
                  }
                }

                signals.runWithOwner(owner, () => {
                  signals.createEffect(colIndex, (colIndex) => {
                    // @ts-ignore
                    rowElements[i].childNodes[colIndex + 1].innerHTML =
                      serialized;
                  });
                });
              }
            });
          });
        });
      });

      // signals.runWithOwner(owner, () => {
      //   const thIndex = thHead.length;
      //   const possibleVecids = indexToVecIds[index()];
      //   const i = Math.round(Math.random() * possibleVecids.length);
      //   const vecId = signals.createSignal({
      //     name: possibleVecids[i],
      //     value: possibleVecids[i],
      //   });
      //   const th = addThCol();
      //   const { select } = utils.dom.createSelect({
      //     id: `col-${vecId}`,
      //     list: possibleVecids.map((vecId) => ({
      //       name: vecId,
      //       value: vecId,
      //     })),
      //     signal: vecId,
      //   });
      //   th.append(select);
      //   signals.createEffect(
      //     () => /** @type {const} */ ([index(), vecId(), rowElements()]),
      //     ([index, vecId, trsBody]) => {
      //       if (!trsBody.length) return;
      //       vecsResources
      //         .getOrCreate(index, vecId.name)
      //         .fetch()
      //         .then((vec) => {
      //           if (!vec) return;
      //           console.log({ vec, trsBody, index });
      //           for (let i = 0; i < vec.length; i++) {
      //             const iRev = vec.length - 1 - i;
      //             const value = vec[iRev];
      //             const td = window.document.createElement("td");
      //             td.innerHTML = String(value);
      //             trsBody[i].append(td);
      //           }
      //         });
      //     },
      //   );
      // });
    };
    // setTimeout(addCol, 2000);
    // addRandomCol();
    // addRandomCol();
    // addRandomCol();
  });

  return obj;
}

/**
 * @param {Object} args
 * @param {Colors} args.colors
 * @param {Signals} args.signals
 * @param {Utilities} args.utils
 * @param {Elements} args.elements
 * @param {VecsResources} args.vecsResources
 * @param {VecIdToIndexes} args.vecIdToIndexes
 */
export function init({
  colors,
  elements,
  signals,
  utils,
  vecsResources,
  vecIdToIndexes,
}) {
  const parent = elements.database;
  const { headerElement } = utils.dom.createHeader({
    title: "Database",
  });
  parent.append(headerElement);

  const div = window.document.createElement("div");
  parent.append(div);

  const table = createTable({
    signals,
    utils,
    vecIdToIndexes,
    vecsResources,
  });
  div.append(table.element);

  const span = window.document.createElement("span");
  span.innerHTML = "Add column";
  div.append(
    utils.dom.createButtonElement({
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
    /** @satisfies {VecId} */ ("height"),
    /** @satisfies {VecId} */ ("dateindex"),
    /** @satisfies {VecId} */ ("weekindex"),
    /** @satisfies {VecId} */ ("difficultyepoch"),
    /** @satisfies {VecId} */ ("monthindex"),
    /** @satisfies {VecId} */ ("quarterindex"),
    /** @satisfies {VecId} */ ("yearindex"),
    /** @satisfies {VecId} */ ("decadeindex"),
    /** @satisfies {VecId} */ ("halvingepoch"),
    /** @satisfies {VecId} */ ("addressindex"),
    /** @satisfies {VecId} */ ("p2pk33index"),
    /** @satisfies {VecId} */ ("p2pk65index"),
    /** @satisfies {VecId} */ ("p2pkhindex"),
    /** @satisfies {VecId} */ ("p2shindex"),
    /** @satisfies {VecId} */ ("p2trindex"),
    /** @satisfies {VecId} */ ("p2wpkhindex"),
    /** @satisfies {VecId} */ ("p2wshindex"),
    /** @satisfies {VecId} */ ("txindex"),
    /** @satisfies {VecId} */ ("txinindex"),
    /** @satisfies {VecId} */ ("txoutindex"),
    /** @satisfies {VecId} */ ("emptyindex"),
    /** @satisfies {VecId} */ ("multisigindex"),
    /** @satisfies {VecId} */ ("opreturnindex"),
    /** @satisfies {VecId} */ ("pushonlyindex"),
    /** @satisfies {VecId} */ ("unknownindex"),
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
      return /** @satisfies {Height} */ (0);
    case "dateindex":
      return /** @satisfies {Dateindex} */ (1);
    case "weekindex":
      return /** @satisfies {Weekindex} */ (2);
    case "difficultyepoch":
      return /** @satisfies {Difficultyepoch} */ (3);
    case "monthindex":
      return /** @satisfies {Monthindex} */ (4);
    case "quarterindex":
      return /** @satisfies {Quarterindex} */ (5);
    case "yearindex":
      return /** @satisfies {Yearindex} */ (6);
    case "decadeindex":
      return /** @satisfies {Decadeindex} */ (7);
    case "halvingepoch":
      return /** @satisfies {Halvingepoch} */ (8);
    case "addressindex":
      return /** @satisfies {Addressindex} */ (9);
    case "p2pk33index":
      return /** @satisfies {P2PK33index} */ (10);
    case "p2pk65index":
      return /** @satisfies {P2PK65index} */ (11);
    case "p2pkhindex":
      return /** @satisfies {P2PKHindex} */ (12);
    case "p2shindex":
      return /** @satisfies {P2SHindex} */ (13);
    case "p2trindex":
      return /** @satisfies {P2TRindex} */ (14);
    case "p2wpkhindex":
      return /** @satisfies {P2WPKHindex} */ (15);
    case "p2wshindex":
      return /** @satisfies {P2WSHindex} */ (16);
    case "txindex":
      return /** @satisfies {Txindex} */ (17);
    case "txinindex":
      return /** @satisfies {Txinindex} */ (18);
    case "txoutindex":
      return /** @satisfies {Txoutindex} */ (19);
    case "emptyindex":
      return /** @satisfies {Emptyindex} */ (20);
    case "multisigindex":
      return /** @satisfies {Multisigindex} */ (21);
    case "opreturnindex":
      return /** @satisfies {Opreturnindex} */ (22);
    case "pushonlyindex":
      return /** @satisfies {Pushonlyindex} */ (23);
    case "unknownindex":
      return /** @satisfies {Unknownindex} */ (24);
  }
}

/**
 * @param {VecIdToIndexes} vecIdToIndexes
 */
function createIndexToVecIds(vecIdToIndexes) {
  const indexToVecIds = Object.entries(vecIdToIndexes).reduce(
    (arr, [_id, indexes]) => {
      const id = /** @type {VecId} */ (_id);
      indexes.forEach((i) => {
        arr[i] ??= [];
        arr[i].push(id);
      });
      return arr;
    },
    /** @type {VecId[][]} */ (new Array(24)),
  );
  indexToVecIds.forEach((arr) => {
    arr.sort();
  });
  return indexToVecIds;
}
