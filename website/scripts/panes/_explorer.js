// import { randomFromArray } from "../utils/array.js";
// import { explorerElement } from "../utils/elements.js";

// export function init() {
//   const chain = window.document.createElement("div");
//   chain.id = "chain";
//   explorerElement.append(chain);

//   // vecsResources.getOrCreate(/** @satisfies {Height}*/ (5), "height");
//   //
//   const miners = [
//     { name: "Foundry USA", color: "orange" },
//     { name: "Via BTC", color: "teal" },
//     { name: "Ant Pool", color: "emerald" },
//     { name: "F2Pool", color: "indigo" },
//     { name: "Spider Pool", color: "yellow" },
//     { name: "Mara Pool", color: "amber" },
//     { name: "SEC Pool", color: "violet" },
//     { name: "Luxor", color: "orange" },
//     { name: "Brains Pool", color: "cyan" },
//   ];

//   for (let i = 0; i <= 10; i++) {
//     const { name, color: _color } = randomFromArray(miners);
//     const { cubeElement, leftFaceElement, rightFaceElement, topFaceElement } =
//       createCube();

//     // cubeElement.style.setProperty("--color", `var(--${color})`);

//     const heightElement = window.document.createElement("p");
//     const height = (1_000_002 - i).toString();
//     const prefixLength = 7 - height.length;
//     const spanPrefix = window.document.createElement("span");
//     spanPrefix.style.opacity = "0.5";
//     spanPrefix.style.userSelect = "none";
//     heightElement.append(spanPrefix);
//     spanPrefix.innerHTML = "#" + "0".repeat(prefixLength);
//     const spanHeight = window.document.createElement("span");
//     heightElement.append(spanHeight);
//     spanHeight.innerHTML = height;
//     rightFaceElement.append(heightElement);

//     const feesElement = window.document.createElement("div");
//     feesElement.classList.add("fees");
//     leftFaceElement.append(feesElement);
//     const averageFeeElement = window.document.createElement("p");
//     feesElement.append(averageFeeElement);
//     averageFeeElement.innerHTML = `~1.41`;
//     const feeRangeElement = window.document.createElement("p");
//     feesElement.append(feeRangeElement);
//     const minFeeElement = window.document.createElement("span");
//     minFeeElement.innerHTML = `0.11`;
//     feeRangeElement.append(minFeeElement);
//     const dashElement = window.document.createElement("span");
//     dashElement.style.opacity = "0.5";
//     dashElement.innerHTML = `-`;
//     feeRangeElement.append(dashElement);
//     const maxFeeElement = window.document.createElement("span");
//     maxFeeElement.innerHTML = `12.1`;
//     feeRangeElement.append(maxFeeElement);
//     const feeUnitElement = window.document.createElement("p");
//     feesElement.append(feeUnitElement);
//     feeUnitElement.style.opacity = "0.5";
//     feeUnitElement.innerHTML = `sat/vB`;

//     const spanMiner = window.document.createElement("span");
//     spanMiner.innerHTML = name;
//     topFaceElement.append(spanMiner);

//     chain.prepend(cubeElement);
//   }
// }

// function createCube() {
//   const cubeElement = window.document.createElement("div");
//   cubeElement.classList.add("cube");

//   const rightFaceElement = window.document.createElement("div");
//   rightFaceElement.classList.add("face");
//   rightFaceElement.classList.add("right");
//   cubeElement.append(rightFaceElement);

//   const leftFaceElement = window.document.createElement("div");
//   leftFaceElement.classList.add("face");
//   leftFaceElement.classList.add("left");
//   cubeElement.append(leftFaceElement);

//   const topFaceElement = window.document.createElement("div");
//   topFaceElement.classList.add("face");
//   topFaceElement.classList.add("top");
//   cubeElement.append(topFaceElement);

//   return {
//     cubeElement,
//     leftFaceElement,
//     rightFaceElement,
//     topFaceElement,
//   };
// }
