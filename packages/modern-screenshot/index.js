import { domToBlob } from "./4.6.6/dist/index.mjs";

const userAgent = navigator.userAgent.toLowerCase();
const iphone = userAgent.includes("iphone");
const ipad = userAgent.includes("ipad");
const ios = iphone || ipad;

/**
 * @param {Object} args
 * @param {Element} args.element
 * @param {string} args.name
 * @param {string} args.title
 */
export async function screenshot({ element, name, title }) {
  const blob = await domToBlob(element, {
    scale: 2,
  });

  if (ios) {
    const file = new File(
      [blob],
      `bitview-${name}-${new Date().toJSON().split(".")[0]}.png`,
      {
        type: "image/png",
      },
    );

    try {
      await navigator.share({
        files: [file],
        title: `${title} on ${window.document.location.hostname}`,
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
