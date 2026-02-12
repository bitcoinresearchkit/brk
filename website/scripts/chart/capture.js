import { ios, canShare } from "../utils/env.js";
import { style } from "../utils/elements.js";
import { colors } from "../utils/colors.js";

export const canCapture = !ios || canShare;

/**
 * @param {Object} args
 * @param {HTMLCanvasElement} args.screenshot
 * @param {number} args.chartWidth
 * @param {HTMLElement} args.parent
 * @param {{ element: HTMLElement }[]} args.legends
 */
export function capture({ screenshot, chartWidth, parent, legends }) {
  const dpr = screenshot.width / chartWidth;
  const pad = Math.round(16 * dpr);
  const fontSize = Math.round(14 * dpr);
  const titleFontSize = Math.round(20 * dpr);
  const circleRadius = Math.round(5 * dpr);
  const legendHeight = Math.round(28 * dpr);
  const titleHeight = Math.round(36 * dpr);

  const title = (parent.querySelector("h1")?.textContent ?? "").toUpperCase();
  const hasTitle = title.length > 0;
  const hasTopLegend = legends[0].element.children.length > 0;
  const hasBottomLegend = legends[1].element.children.length > 0;
  const titleOffset = hasTitle ? titleHeight : 0;
  const topLegendOffset = hasTopLegend ? legendHeight : 0;
  const bottomOffset = hasBottomLegend ? legendHeight : 0;

  const canvas = document.createElement("canvas");
  canvas.width = screenshot.width + pad * 2;
  canvas.height =
    screenshot.height + pad * 2 + titleOffset + topLegendOffset + bottomOffset;
  const ctx = canvas.getContext("2d");
  if (!ctx) return;

  // Background
  const bodyBg = getComputedStyle(document.body).backgroundColor;
  const htmlBg = getComputedStyle(document.documentElement).backgroundColor;
  ctx.fillStyle = bodyBg === "rgba(0, 0, 0, 0)" ? htmlBg : bodyBg;
  ctx.fillRect(0, 0, canvas.width, canvas.height);

  /** @param {HTMLElement} legendEl @param {number} y */
  const drawLegend = (legendEl, y) => {
    ctx.font = `${fontSize}px ${style.fontFamily}`;
    ctx.textAlign = "left";
    ctx.textBaseline = "middle";
    let x = pad;
    for (const div of legendEl.children) {
      const label = div.querySelector("label");
      if (!label) continue;
      const input = label.querySelector("input");
      if (input && !input.checked) continue;
      // Draw color circles
      const colorSpans = label.querySelectorAll(".colors span");
      for (const span of colorSpans) {
        ctx.fillStyle = /** @type {HTMLElement} */ (span).style.backgroundColor;
        ctx.beginPath();
        ctx.arc(x + circleRadius, y, circleRadius, 0, Math.PI * 2);
        ctx.fill();
        x += circleRadius * 2 + Math.round(2 * dpr);
      }
      // Draw name
      const name = label.querySelector(".name")?.textContent ?? "";
      ctx.fillStyle = colors.default();
      ctx.fillText(name, x + Math.round(4 * dpr), y);
      x += ctx.measureText(name).width + Math.round(20 * dpr);
    }
  };

  // Title
  if (hasTitle) {
    ctx.font = `${titleFontSize}px ${style.fontFamily}`;
    ctx.fillStyle = colors.default();
    ctx.textAlign = "left";
    ctx.textBaseline = "middle";
    ctx.fillText(title, pad, pad + titleHeight / 2);
  }

  // Top legend
  if (hasTopLegend) {
    drawLegend(legends[0].element, pad + titleOffset + topLegendOffset / 2);
  }

  // Chart
  ctx.drawImage(screenshot, pad, pad + titleOffset + topLegendOffset);

  // Bottom legend
  if (hasBottomLegend) {
    drawLegend(
      legends[1].element,
      pad +
        titleOffset +
        topLegendOffset +
        screenshot.height +
        legendHeight / 2,
    );
  }

  // Watermark
  ctx.fillStyle = colors.gray();
  ctx.font = `${fontSize}px ${style.fontFamily}`;
  ctx.textAlign = "right";
  ctx.textBaseline = "bottom";
  ctx.fillText(
    window.location.host,
    canvas.width - pad,
    canvas.height - pad / 2,
  );

  // Open in new tab
  canvas.toBlob((blob) => {
    if (!blob) return;
    const url = URL.createObjectURL(blob);
    window.open(url, "_blank");
    setTimeout(() => URL.revokeObjectURL(url), 100);
  }, "image/png");
}
