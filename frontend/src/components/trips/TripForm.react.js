import React, { useState, useEffect } from 'react';
import { connect } from 'react-redux';

import List from 'react-virtualized/dist/commonjs/List';
import ReactCountryFlag from 'react-country-flag';
import Select from 'react-select';
import format from 'date-fns/format';

import DatePicker from '../DatePicker.react';

import {
  determineNextFreeDate,
  determineAlreadySelectedTripDates
} from '../../helpers/dateUtils';
import { fetchCities } from '../../actions';

//
// Virtualized Select Component
//
const MenuList = props => {
  const rows = props.children;
  const rowRenderer = ({ key, index, isScrolling, isVisible, style }) => (
    <div key={key} style={style}>
      {rows[index]}
    </div>
  );

  return (
    <List
      style={{ width: '100%', flexGrow: true }}
      width={props.width}
      height={300}
      rowHeight={35}
      rowCount={rows.length || 0}
      rowRenderer={rowRenderer}
    />
  );
};

const convertToSelectOpts = i =>
  i.map(i => ({
    label: (
      <div>
        <ReactCountryFlag code={i.country_code} svg /> {i.name}
      </div>
    ),
    value: i.id,
    name: i.name
  }));

const filterOption = (opt, str) => {
  return (
    (opt.data.name || '').toUpperCase().indexOf((str || '').toUpperCase()) > -1
  );
};

function TripForm({
  cities,
  trips,
  onSubmit,
  fetchCities,
  initialDates,
  initialCity
}) {
  // Default to the next free day and no city selected
  const [dates, setDates] = useState(
    initialDates || determineNextFreeDate(trips, 4)
  );
  const [city, setCity] = useState(initialCity);

  // Load cities list
  useEffect(() => {
    if (!cities.isFetching && !cities.hasFetched) {
      fetchCities();
    }
  }, []);

  //
  // Package up the state as a trip object and pass it back up
  //
  const onSubmitClick = e => {
    e.preventDefault();
    onSubmit({
      city: city,
      start_date: format(dates[0], 'YYYY-MM-DD'),
      end_date: format(dates[1], 'YYYY-MM-DD')
    });
  };

  // Order cities by population for now
  const citiesList = Object.keys(cities.all).map(cId => cities.all[cId]);
  citiesList.sort((a, b) => b.population - a.population);

  const opts = convertToSelectOpts(citiesList);
  const value = opts.find(a => a.value === city);

  return (
    <form className="uk-form-horizontal new-trip-form">
      <div className="uk-margin">
        <label className="uk-form-label">CITY</label>
        <div className="uk-form-controls">
          <Select
            styles={{
              container: () => ({
                width: 250
              }),
              menu: provided => ({
                ...provided,
                zIndex: 9999,
                top: null
              })
            }}
            filterOption={filterOption}
            components={{
              MenuList: props => (
                <MenuList {...props} width={window.innerWidth} />
              )
            }}
            placeholder=""
            className="country-picker"
            isSearchAble
            isLoading={cities.isFetching}
            value={value}
            onChange={e => setCity(e.value)}
            options={opts}
          />
        </div>
      </div>
      <div className="uk-margin">
        <label className="uk-form-label">DATE</label>
        <div className="uk-form-controls">
          <DatePicker
            className="uk-width-1-3@m"
            rangeClassName="uk-text-center margin-unset"
            disabledDates={determineAlreadySelectedTripDates(trips)}
            value={dates}
            onChange={setDates}
            hideDaysCount
          />
        </div>
      </div>
      <button
        className="uk-button uk-button-default"
        onClick={onSubmitClick}
        disabled={city === null || city === undefined || dates.length !== 2}
      >
        SUBMIT
      </button>
    </form>
  );
}

const mapStateToProps = ({ cities }) => ({
  cities
});

export default connect(
  mapStateToProps,
  {
    fetchCities
  }
)(TripForm);
