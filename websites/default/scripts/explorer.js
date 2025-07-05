// @ts-check

/**
 * @param {Object} args
 * @param {Colors} args.colors
 * @param {LightweightCharts} args.lightweightCharts
 * @param {Accessor<ChartOption>} args.option
 * @param {Signals} args.signals
 * @param {Utilities} args.utils
 * @param {WebSockets} args.webSockets
 * @param {Elements} args.elements
 * @param {VecsResources} args.vecsResources
 * @param {VecIdToIndexes} args.vecIdToIndexes
 */
export function init({
  colors,
  elements,
  lightweightCharts,
  option,
  signals,
  utils,
  webSockets,
  vecsResources,
  vecIdToIndexes,
}) {
  const chain = window.document.createElement("div");
  chain.id = "chain";
  elements.explorer.append(chain);

  chain.append(createCube());
  chain.append(createCube());
  chain.append(createCube());
  chain.append(createCube());
  chain.append(createCube());
}

function createCube() {
  const cubeElement = window.document.createElement("div");
  cubeElement.classList.add("cube");
  const faceFrontElement = window.document.createElement("div");
  faceFrontElement.classList.add("face");
  faceFrontElement.classList.add("front");
  cubeElement.append(faceFrontElement);
  const faceSideElement = window.document.createElement("div");
  faceSideElement.classList.add("face");
  faceSideElement.classList.add("side");
  cubeElement.append(faceSideElement);
  const faceTopElement = window.document.createElement("div");
  faceTopElement.classList.add("face");
  faceTopElement.classList.add("top");
  cubeElement.append(faceTopElement);
  return cubeElement;
}
