import { domToBlob } from "./4.6.6/dist/index.mjs";

/**
 * @param {Object} args
 * @param {Element} args.element
 * @param {string} args.name
 * @param {string} args.title
 * @param {Env} args.env
 */
export async function screenshot({ element, name, title, env }) {
  const blob = await domToBlob(element, {
    scale: 2,
  });

  if (env.ios) {
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
        title: `Bitview screenshot: ${title}`,
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
