import { domToBlob } from "./4.6.6/dist/index.mjs";

/**
 * @param {Element} element
 */
export async function screenshot(element) {
  const blob = await domToBlob(element, {
    scale: 2,
  });
  const url = URL.createObjectURL(blob);
  window.open(url, "_blank");
  setTimeout(() => URL.revokeObjectURL(url), 100);
}
