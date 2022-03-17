import isWithinRange from 'date-fns/is_within_range';
import differenceInDays from 'date-fns/difference_in_days';
import differenceInCalendarMonths from 'date-fns/difference_in_calendar_months';
import isBefore from 'date-fns/is_before';
import addDays from 'date-fns/add_days';
import subDays from 'date-fns/sub_days';
import format from 'date-fns/format';

// Disable any dates that you already have trips on (allowing 1 day overlap)
export function determineAlreadySelectedTripDates(trips) {
  return trips.map(t => ({
    from: addDays(t.start_date, 1),
    to: subDays(t.end_date, 1)
  }));
}

// Determine the next gap in the dates of n days
export function determineNextFreeDate(trips, n) {
  const nextFreeDate = trips.reduce((lastChecked, nextTrip) => {

    // Ignore any dates before now
    if (isBefore(nextTrip.end_date, newDate())) {
      return newDate();
    }

    // Only allow a gap to be auto-selected if there are n days available
    if (isBefore(addDays(lastChecked, n - 1), nextTrip.start_date)) {
      return lastChecked;
    }

    return nextTrip.end_date;
  }, newDate());
  return [nextFreeDate, newDate(addDays(nextFreeDate, n))];
}

export const newDate = (d = Date.now()) => format(d, 'YYYY-MM-DD');

export const isFutureTrip = t => differenceInDays(t.end_date, newDate()) >= 0;
export const isPastTrip = t => differenceInDays(t.end_date, newDate()) < 0;
export const isCurrentTrip = t =>
  isWithinRange(newDate(), t.start_date, t.end_date);

// Determine the number of months between the sorted list of locations
export function getMonthsInRange(locations) {
  if (locations.length === 0) {
    return 0;
  }

  const diff = differenceInCalendarMonths(
    locations[locations.length - 1].end_date,
    locations[0].start_date
  );
  return Math.abs(diff);
}
