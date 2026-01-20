/**
 * @param {readonly [number, number, number, number, number, number, number, number, number]} A
 * @param {readonly [number, number, number]} B
 */
function multiplyMatrices(A, B) {
  return /** @type {const} */ ([
    A[0] * B[0] + A[1] * B[1] + A[2] * B[2],
    A[3] * B[0] + A[4] * B[1] + A[5] * B[2],
    A[6] * B[0] + A[7] * B[1] + A[8] * B[2],
  ]);
}

/** @param {readonly [number, number, number]} param0 */
function oklch2oklab([l, c, h]) {
  return /** @type {const} */ ([
    l,
    isNaN(h) ? 0 : c * Math.cos((h * Math.PI) / 180),
    isNaN(h) ? 0 : c * Math.sin((h * Math.PI) / 180),
  ]);
}

/** @param {readonly [number, number, number]} rgb */
function srgbLinear2rgb(rgb) {
  return rgb.map((c) =>
    Math.abs(c) > 0.0031308
      ? (c < 0 ? -1 : 1) * (1.055 * Math.abs(c) ** (1 / 2.4) - 0.055)
      : 12.92 * c,
  );
}

/** @param {readonly [number, number, number]} lab */
function oklab2xyz(lab) {
  const LMSg = multiplyMatrices(
    [1, 0.3963377773761749, 0.2158037573099136, 1, -0.1055613458156586,
     -0.0638541728258133, 1, -0.0894841775298119, -1.2914855480194092],
    lab,
  );
  const LMS = /** @type {[number, number, number]} */ (LMSg.map((val) => val ** 3));
  return multiplyMatrices(
    [1.2268798758459243, -0.5578149944602171, 0.2813910456659647,
     -0.0405757452148008, 1.112286803280317, -0.0717110580655164,
     -0.0763729366746601, -0.4214933324022432, 1.5869240198367816],
    LMS,
  );
}

/** @param {readonly [number, number, number]} xyz */
function xyz2rgbLinear(xyz) {
  return multiplyMatrices(
    [3.2409699419045226, -1.537383177570094, -0.4986107602930034,
     -0.9692436362808796, 1.8759675015077202, 0.04155505740717559,
     0.05563007969699366, -0.20397695888897652, 1.0569715142428786],
    xyz,
  );
}

/** @type {Map<string, [number, number, number, number]>} */
const conversionCache = new Map();

/**
 * Parse oklch string and return rgba tuple
 * @param {string} oklch
 * @returns {[number, number, number, number] | null}
 */
function parseOklch(oklch) {
  if (!oklch.startsWith("oklch(")) return null;

  const cached = conversionCache.get(oklch);
  if (cached) return cached;

  let str = oklch.slice(6, -1); // remove "oklch(" and ")"
  let alpha = 1;

  const slashIdx = str.indexOf(" / ");
  if (slashIdx !== -1) {
    const alphaPart = str.slice(slashIdx + 3);
    alpha = alphaPart.includes("%")
      ? Number(alphaPart.replace("%", "")) / 100
      : Number(alphaPart);
    str = str.slice(0, slashIdx);
  }

  const parts = str.split(" ");
  const l = parts[0].includes("%") ? Number(parts[0].replace("%", "")) / 100 : Number(parts[0]);
  const c = Number(parts[1]);
  const h = Number(parts[2]);

  const rgb = srgbLinear2rgb(xyz2rgbLinear(oklab2xyz(oklch2oklab([l, c, h]))))
    .map((v) => Math.max(Math.min(Math.round(v * 255), 255), 0));

  const result = /** @type {[number, number, number, number]} */ ([...rgb, alpha]);
  conversionCache.set(oklch, result);
  return result;
}

/**
 * Convert oklch string to rgba string
 * @param {string} oklch
 * @returns {string}
 */
export function oklchToRgba(oklch) {
  const result = parseOklch(oklch);
  if (!result) return oklch;
  const [r, g, b, a] = result;
  return a === 1 ? `rgb(${r}, ${g}, ${b})` : `rgba(${r}, ${g}, ${b}, ${a})`;
}

