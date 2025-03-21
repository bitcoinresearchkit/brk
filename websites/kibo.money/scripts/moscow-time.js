// @ts-check

/**
 * @import {Options} from './options';
 */

/**
 * @param {Object} args
 * @param {Colors} args.colors
 * @param {Constants} args.consts
 * @param {Signals} args.signals
 * @param {Utilities} args.utils
 * @param {Options} args.options
 * @param {Datasets} args.datasets
 * @param {WebSockets} args.webSockets
 * @param {Elements} args.elements
 * @param {Ids} args.ids
 * @param {Accessor<boolean>} args.dark
 */
export function init({
  colors,
  consts,
  dark,
  datasets,
  elements,
  ids,
  options,
  signals,
  utils,
  webSockets,
}) {
  const moscowTimeElement = elements.moscowTime;

  const satsPerDollar = signals.createMemo(
    () =>
      100_000_000 /
      // webSockets.kraken5mnCandle.latest()?.close ||
      (webSockets.kraken1dCandle.latest()?.close || 0),
  );

  const p = window.document.createElement("h1");
  moscowTimeElement.append(p);

  signals.createEffect(satsPerDollar, (satsPerDollar) => {
    p.innerHTML = utils.formatters.dollars.format(satsPerDollar);
  });
}
