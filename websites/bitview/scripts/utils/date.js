const ONE_DAY_IN_MS = 1000 * 60 * 60 * 24;

export function todayUTC() {
  const today = new Date();
  return new Date(
    Date.UTC(
      today.getUTCFullYear(),
      today.getUTCMonth(),
      today.getUTCDate(),
      0,
      0,
      0,
    ),
  );
}

/**
 * @param {Date} date
 */
export function dateToDateIndex(date) {
  if (
    date.getUTCFullYear() === 2009 &&
    date.getUTCMonth() === 0 &&
    date.getUTCDate() === 3
  )
    return 0;
  return differenceBetweenDates(date, new Date("2009-01-09"));
}

/**
 * @param {Date} start
 * @param {Date} end
 */
export function createDateRange(start, end) {
  const dates = /** @type {Date[]} */ ([]);
  let currentDate = new Date(start);
  while (currentDate <= end) {
    dates.push(new Date(currentDate));
    currentDate.setUTCDate(currentDate.getUTCDate() + 1);
  }
  return dates;
}

/**
 * @param {Date} date1
 * @param {Date} date2
 */
export function differenceBetweenDates(date1, date2) {
  return Math.abs(date1.valueOf() - date2.valueOf()) / ONE_DAY_IN_MS;
}

/**
 * @param {Date} date1
 * @param {Date} date2
 */
export function roundedDifferenceBetweenDates(date1, date2) {
  return Math.round(differenceBetweenDates(date1, date2));
}
