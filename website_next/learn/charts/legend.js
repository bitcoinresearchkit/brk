/**
 * @param {Chart} chart
 * @returns {{ legend: HTMLElement, menu: HTMLElement, items: HTMLElement[], readout: Readout }}
 */
export function createLegend(chart) {
  const legend = document.createElement("figcaption");
  const header = document.createElement("header");
  const title = document.createElement("span");
  const time = document.createElement("time");
  const menu = document.createElement("menu");
  const rows = chart.series.map((series) => {
    const item = document.createElement("li");
    const button = document.createElement("button");
    const label = document.createElement("span");
    const value = document.createElement("output");

    button.type = "button";
    button.style.setProperty("--color", series.color());
    label.append(series.label);
    button.append(label, value);
    item.append(button);
    menu.append(item);

    return { button, value };
  });
  const items = rows.map(({ button }) => button);

  title.append(chart.title);
  header.append(title);
  header.append(time);
  legend.append(header, menu);

  return { legend, menu, items, readout: { time, rows } };
}

/**
 * @typedef {Object} Readout
 * @property {HTMLTimeElement} time
 * @property {{ value: HTMLOutputElement }[]} rows
 */

/** @typedef {import("./index.js").Chart} Chart */
