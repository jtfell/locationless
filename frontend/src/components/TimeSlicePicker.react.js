import React from 'react';
import Select from 'react-select';

import addMonths from 'date-fns/add_months';
import addYears from 'date-fns/add_years';
import lastDayOfYear from 'date-fns/last_day_of_year';

function TimeSlicePicker({ onChange, value, disabled, className }) {
  const now = new Date();
  const options = [
    {
      label: 'Next 3 Months',
      value: [now, addMonths(now, 3)]
    },
    {
      label: 'Next 6 Months',
      value: [now, addMonths(now, 6)]
    },
    {
      label: 'This Year',
      value: [addYears(lastDayOfYear(now), -1), lastDayOfYear(now)]
    },
    {
      label: 'Last 3 Months',
      value: [addMonths(now, -3), now]
    },
    {
      label: 'Last 6 Months',
      value: [addMonths(now, -6), now]
    }
  ];

  const onSelection = label => {
    const { value } = options.find(o => o.label === label);
    onChange(value);
  };

  return (
    <Select
      styles={{
        container: () => ({ width: 200 }),
        menu: provided => ({ ...provided, zIndex: 9999 }),
        menuPortal: provided => ({ ...provided, zIndex: 9999 })
      }}
      className={className}
      onChange={onSelection}
      menuPosition="fixed"
      value={null}
      disabled={disabled}
      isSearchable={false}
      placeholder={'Preset'}
      options={options.map(o => o.label)}
      getOptionLabel={a => a}
    />
  );
}

export default TimeSlicePicker;
