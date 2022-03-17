import React from 'react';
import ReactCountryFlag from 'react-country-flag';

import DatePicker from '../DatePicker.react';

const TripPanel = ({ city, start_date, end_date, matches, isActive }) => (
  <form className="uk-form-horizontal uk-margin-large">
    <div className="uk-margin">
      <label className="uk-form-label">
        <h4>LOCATION</h4>
      </label>
      <div className="uk-form-controls">
        <div className="outlined uk-flex uk-flex-middle">
          <ReactCountryFlag code={city.country_code} svg />
          <span>{city.name}</span>
        </div>
      </div>
    </div>

    <div className="uk-margin">
      <label className="uk-form-label">
        <h4>DATES</h4>
      </label>
      <div className="uk-form-controls">
        <DatePicker
          rangeClassName="outlined"
          value={[start_date, end_date]}
          hideDaysCount
        />
      </div>
    </div>
  </form>
);

export default TripPanel;
