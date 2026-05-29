const INFERNO_STOPS = [
  [0, 0, 0, 0],
  [0.13, 40, 11, 84],
  [0.25, 101, 21, 110],
  [0.38, 159, 42, 99],
  [0.5, 212, 72, 66],
  [0.63, 245, 125, 21],
  [0.75, 250, 193, 39],
  [0.88, 252, 243, 105],
  [1, 252, 255, 164],
];

export const INFERNO_LUT = createColorLut(INFERNO_STOPS);

/**
 * @param {number[][]} stops - Tuples of [position, red, green, blue].
 */
export function createColorLut(stops) {
  const lut = new Uint32Array(256);
  for (let i = 0; i < lut.length; i++) {
    const t = i / 255;
    let a = stops[0];
    let b = stops[stops.length - 1];
    for (let j = 0; j < stops.length - 1; j++) {
      if (t >= stops[j][0] && t <= stops[j + 1][0]) {
        a = stops[j];
        b = stops[j + 1];
        break;
      }
    }
    const f = a[0] === b[0] ? 0 : (t - a[0]) / (b[0] - a[0]);
    const r = (a[1] + f * (b[1] - a[1]) + 0.5) | 0;
    const g = (a[2] + f * (b[2] - a[2]) + 0.5) | 0;
    const blue = (a[3] + f * (b[3] - a[3]) + 0.5) | 0;
    lut[i] = 0xff000000 | (blue << 16) | (g << 8) | r;
  }
  return lut;
}
