/** @param {number} value */
export function formatCoordinate(value) {
  return value.toFixed(2);
}

/**
 * @param {string} command
 * @param {number} x
 * @param {number} y
 */
function createPathCommand(command, x, y) {
  return `${command}${formatCoordinate(x)} ${formatCoordinate(y)}`;
}

/** @param {ChartPoint[]} points */
export function createLinePathData(points) {
  return points
    .map(({ x, y }, index) => createPathCommand(index ? "L" : "M", x, y))
    .join(" ");
}

/** @param {StackedPoint[]} points */
export function createAreaPathData(points) {
  const commands = points.map(({ x, y1 }, index) =>
    createPathCommand(index ? "L" : "M", x, y1),
  );

  for (let index = points.length - 1; index >= 0; index -= 1) {
    const { x, y0 } = points[index];

    commands.push(createPathCommand("L", x, y0));
  }

  return `${commands.join(" ")} Z`;
}
