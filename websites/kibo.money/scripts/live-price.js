// @ts-check

/**
 * @import {Options} from './options';
 */

/**
 * @param {Object} args
 * @param {Colors} args.colors
 * @param {Consts} args.consts
 * @param {LightweightCharts} args.lightweightCharts
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
  lightweightCharts,
  options,
  signals,
  utils,
  webSockets,
}) {
  const livePriceElement = elements.livePrice;

  const price = window.document.createElement("h1");
  livePriceElement.append(price);

  signals.createEffect(webSockets.kraken1dCandle.latest, (candle) => {
    if (!candle) return;
    price.innerHTML = utils.formatters.dollars.format(candle.close);
  });
}
