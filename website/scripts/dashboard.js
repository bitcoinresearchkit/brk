/**
 * @import {Options} from './options';
 */

/**
 * @param {Object} args
 * @param {Flexmasonry} args.flexmasonry
 * @param {Signal<Record<LastPath, number> | null>} args.lastValues
 * @param {Options} args.options
 * @param {Accessor<DashboardOption>} args.selected
 * @param {Signals} args.signals
 * @param {Elements} args.elements
 * @param {Utilities} args.utils
 */
export function initDashboardElement({
  elements,
  flexmasonry,
  lastValues,
  options,
  selected,
  signals,
  utils,
}) {
  flexmasonry.destroyAll();

  const element = elements.dashboard;

  element.innerHTML = "";

  /**
   * @param {HTMLElement} parent
   * @param {string} name
   * @param {boolean} [defaultOpen]
   */
  function createDetails(parent, name, defaultOpen) {
    const details = window.document.createElement("details");
    details.open = defaultOpen || false;
    parent.append(details);

    const summary = window.document.createElement("summary");
    summary.innerHTML = name;
    details.append(summary);

    const div = window.document.createElement("div");
    details.append(div);

    return div;
  }

  /**
   * @param {HTMLElement} parent
   * @param {string} name
   */
  function createTable(parent, name) {
    const table = window.document.createElement("table");
    parent.append(table);

    const tbody = window.document.createElement("tbody");
    table.append(tbody);

    return tbody;
  }

  /**
   * @param {Object} args
   * @param {HTMLTableSectionElement} args.tbody
   * @param {string} args.name
   * @param {Unit} [args.unit]
   * @param {string} args.path
   * @param {boolean} [args.formatNumber]
   */
  function createRow({ tbody, unit, name, path, formatNumber }) {
    path = path.replace("date-to-", "").replace("height-to-", "");

    const tr = window.document.createElement("tr");
    tbody.append(tr);

    const tdName = window.document.createElement("td");
    tr.append(tdName);

    const a = window.document.createElement("a");
    a.href = path;
    a.innerHTML = name;
    tdName.append(a);

    const tdValue = window.document.createElement("td");

    const preSmall = window.document.createElement("small");
    tdValue.append(preSmall);

    const valueSpan = window.document.createElement("span");
    tdValue.append(valueSpan);

    const postSmall = window.document.createElement("small");
    tdValue.append(postSmall);

    signals.createEffect(() => {
      const _lastValues = lastValues();

      if (!_lastValues || !(path in _lastValues)) return;

      const value = _lastValues[/** @type {LastPath} */ (path)] ?? 0;

      tdValue.title = `${utils.locale.numberToUSFormat(value ?? 0)}`;

      const formattedValue =
        formatNumber ?? unit !== "Count"
          ? utils.locale.numberToShortUSFormat(value)
          : utils.locale.numberToUSFormat(
              value,
              unit === "Count" ? 0 : undefined,
            );

      if (unit === "Date") {
        valueSpan.innerHTML = String(value);
        postSmall.innerHTML = ` UTC`;
        return;
      }

      valueSpan.innerHTML = formattedValue;

      switch (unit) {
        case "US Dollars": {
          preSmall.innerHTML = `$`;
          break;
        }
        case "Bitcoin": {
          preSmall.innerHTML = `₿`;
          break;
        }
        case "Percentage": {
          postSmall.innerHTML = `%`;
          break;
        }
        case "Seconds": {
          postSmall.innerHTML = ` sec`;
          break;
        }
        case "Megabytes": {
          postSmall.innerHTML = ` MB`;
          break;
        }
      }
    });

    tr.append(tdValue);
  }

  // selected().groups.forEach(({ name, values, unit: groupUnit }) => {
  //   const tbody = createTable(name);

  //   values.forEach(({ name, path, unit: valueUnit, formatNumber }) => {
  //     const unit = groupUnit ?? valueUnit;

  //     createRow({
  //       name,
  //       tbody,
  //       path,
  //       unit,
  //       formatNumber,
  //     });
  //   });
  // });
  //
  /** @type {HTMLTableSectionElement | null} */
  let currentTbody = null;

  const separator = " · ";
  /**
   * @param {Object} args
   * @param {OptionsTree} args.tree
   * @param {{group: OptionsGroup; element: HTMLElement} | null} args.parent
   * @param {string[]} [args.namePre]
   */
  function recursiveOptionConverter({ tree, parent, namePre }) {
    tree.forEach((anyOption) => {
      if ("tree" in anyOption) {
        currentTbody = null;

        const group = anyOption;

        if (!group.tree.length || group.dashboard?.skip) return;

        if (!parent || group.dashboard?.separate) {
          recursiveOptionConverter({
            tree: group.tree,
            parent: {
              group,
              element: createDetails(
                element,
                group.name,
                group.dashboard?.defaultOpen,
              ),
            },
            namePre,
          });
        } else {
          const pre = group.dashboard?.flatten
            ? [...(namePre || []), group.name]
            : [];

          let element = parent.element;

          if (!group.dashboard?.flatten) {
            element = createDetails(
              parent.element,
              group.name,
              group.dashboard?.defaultOpen,
            );
          }

          recursiveOptionConverter({
            tree: group.tree,
            parent: {
              group,
              element,
            },
            namePre: pre,
          });
        }
      } else if (anyOption.kind === "chart" && parent) {
        currentTbody ||= createTable(
          parent.element,
          (namePre || []).join(separator),
        );

        if (!currentTbody) throw "Shouldn't be possible";

        const tbody = currentTbody;

        const option = anyOption;

        if (option.dashboard?.skip) return;

        const { top, bottom } = option;

        const topLength = top?.length ?? 0;
        const bottomLength = bottom?.length ?? 0;

        /**
         * @param {SeriesBlueprint[]} array
         */
        function createRowFromBlueprint(array) {
          const searchArray = array.filter(
            (blueprint) =>
              blueprint.options?.lastValueVisible !== false &&
              /** @type {LineStyleOptions | undefined} */ (blueprint.options)
                ?.lineStyle === undefined,
          );

          const blueprint =
            searchArray.length === 1
              ? searchArray[0]
              : searchArray.find((blueprint) => blueprint.main);

          if (!blueprint) return;

          let name = namePre?.join(separator) || "";
          if (!option.dashboard?.ignoreName) {
            if (name) {
              name += separator;
            }
            name += option.name;
          }

          createRow({
            name,
            tbody,
            path: blueprint.datasetPath,
            unit: option.unit,
            formatNumber: blueprint.formatNumber,
          });
        }

        if (!topLength && !bottomLength) {
          createRow({
            name: option.name,
            tbody,
            path: "close",
            unit: option.unit,
            formatNumber: false,
          });
        } else if (top && bottomLength === 0) {
          createRowFromBlueprint(top);
        } else if (bottom) {
          createRowFromBlueprint(bottom);
        }
      } else if (parent && "unit" in anyOption) {
        createRow({
          name: anyOption.name,
          tbody: currentTbody,
          path: anyOption.path,
          unit: anyOption.unit,
          formatNumber: false,
        });
      }
    });
    currentTbody = null;
  }

  recursiveOptionConverter({
    tree: /** @type {OptionsGroup} */ (
      /** @type {OptionsGroup} */ (options.tree[1]).tree[0]
    ).tree,
    parent: null,
  });

  flexmasonry.init([element]);
}
