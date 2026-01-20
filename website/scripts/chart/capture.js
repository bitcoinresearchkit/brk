import { ios, canShare } from "../utils/env.js";
import { domToBlob } from "../modules/modern-screenshot/4.6.7/dist/index.mjs";

export const canCapture = !ios || canShare;

/**
 * @param {Object} args
 * @param {Element} args.element
 * @param {string} args.name
 */
export async function capture({ element, name }) {
  const blob = await domToBlob(element, {
    scale: 2,
  });

  if (ios) {
    const file = new File(
      [blob],
      `bitview-${name}-${new Date().toJSON().split(".")[0]}.png`,
      { type: "image/png" },
    );

    try {
      await navigator.share({
        files: [file],
        title: `${name} on ${window.document.location.hostname}`,
      });
      return;
    } catch (err) {
      console.log(err);
    }
  }

  const url = URL.createObjectURL(blob);
  window.open(url, "_blank");
  setTimeout(() => URL.revokeObjectURL(url), 100);
}
