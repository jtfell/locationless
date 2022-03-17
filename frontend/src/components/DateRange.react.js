import React, { Fragment } from 'react';

import getMonth from 'date-fns/get_month';
import format from 'date-fns/format';
import differenceInDays from 'date-fns/difference_in_days';

const MONTHS = [
  'JAN',
  'FEB',
  'MAR',
  'APR',
  'MAY',
  'JUN',
  'JUL',
  'AUG',
  'SEP',
  'OCT',
  'NOV',
  'DEC'
];

const STYLES = {
  margin: 'auto',
  width: 'fit-content'
};

const DateRange = ({
  start_date,
  end_date,
  hideDaysCount,
  className,
  style
}) => (
  <div
    data-toggle
    className={`uk-flex date-range ${className || ''}`}
    style={style || STYLES}
  >
    <div>
      <b>
        {format(start_date, 'DD')} <br /> {MONTHS[getMonth(start_date)]}
      </b>
    </div>
    <div>
      <b>
        {end_date && (
          <Fragment>
            {format(end_date, 'DD')}
            <br />
            {MONTHS[getMonth(end_date)]}
          </Fragment>
        )}
      </b>
    </div>
    {!hideDaysCount && (
      <h6
        style={{
          width: 92,
          height: 42,
          textAlign: 'center',
          color: '#666',
          margin: 2,
          marginTop: 10
        }}
      >
        {end_date ? differenceInDays(end_date, start_date) : '?'} Day Trip
      </h6>
    )}
  </div>
);

export default DateRange;
