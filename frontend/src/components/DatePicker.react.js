import React from 'react';
import Flatpickr from 'react-flatpickr';
import DateRange from './DateRange.react';

import 'flatpickr/dist/themes/light.css';

function DatePicker({
  disabledDates,
  onChange,
  value,
  hideDaysCount,
  disabled,
  className,
  rangeClassName
}) {
  if (disabled) {
    return (
      <DateRange
        className={rangeClassName}
        start_date={value[0]}
        end_date={value[1]}
        hideDaysCount={hideDaysCount}
      />
    );
  }
  return (
    <Flatpickr
      className={className}
      options={{
        mode: 'range',
        disable: disabledDates || [],
        dateFormat: 'Y-m-d',
        wrap: true
      }}
      value={value}
      onChange={onChange}
    >
      <DateRange
        className={rangeClassName}
        start_date={value[0]}
        end_date={value[1]}
        hideDaysCount={hideDaysCount}
      />
      <input
        style={{ opacity: 0, margin: 0, width: 1, padding: 0 }}
        type="text"
        placeholder="I'm invisible and only here to anchor calendar to"
        data-input
      />
    </Flatpickr>
  );
}

export default DatePicker;
