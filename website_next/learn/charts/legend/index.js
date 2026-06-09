/**
 * @param {Chart} chart
 * @returns {{ legend: HTMLElement, menu: HTMLElement, items: HTMLElement[], readout: LegendReadout }}
 */
export function createLegend(chart) {
  const legend = document.createElement("figcaption");
  const header = document.createElement("header");
  const title = document.createElement("h5");
  const separator = document.createElement("span");
  const unit = document.createElement("span");
  const time = document.createElement("time");
  const menu = document.createElement("menu");
  const rows = chart.series.map((series) => {
    const item = document.createElement("li");
    const button = document.createElement("button");
    const label = document.createElement("span");
    const value = document.createElement("output");

    button.type = "button";
    button.setAttribute("aria-label", `Highlight ${series.label}`);
    button.style.setProperty("--color", series.color());
    label.append(series.label);
    button.append(label, value);
    item.append(button);
    menu.append(item);

    return { button, value };
  });
  const items = rows.map(({ button }) => button);

  separator.dataset.chart = "separator";
  separator.setAttribute("aria-hidden", "true");
  separator.append("|");
  unit.dataset.chart = "unit";
  unit.setAttribute("aria-label", chart.unit.name);
  unit.append(chart.unit.id);
  title.append(chart.title, " ", separator, " ", unit);
  header.append(title);
  header.append(time);
  legend.append(header, menu);

  return { legend, menu, items, readout: { time, rows } };
}
