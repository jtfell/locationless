import React, { Fragment, useEffect, useState } from 'react';
import { connect } from 'react-redux';
import { Link } from 'react-router-dom';
import { fetchCities, fetchExploreSuggestions } from '../actions';

import addDays from 'date-fns/add_days';
import format from 'date-fns/format';

// import Select from 'react-select';
import ReactCountryFlag from 'react-country-flag';

import DatePicker from '../components/DatePicker.react';
import Spinner from '../components/Spinner.react';
import FriendChip from '../components/friends/FriendChip.react';

const CONTINENT_OPTIONS = [
  { label: 'Europe', value: 'EU' },
  { label: 'N America', value: 'NA' },
  { label: 'S America', value: 'SA' },
  { label: 'Africa', value: 'AF' },
  { label: 'Asia', value: 'AS' },
  { label: 'Oceania', value: 'OC' }
];

const Explore = ({
  cities,
  suggestions,
  users,
  fetchCities,
  activeUser,
  fetchExploreSuggestions
}) => {
  // Default to the next +60 days on any continent
  const [continent, setContinent] = useState(null);
  const [dates, setDates] = useState([new Date(), addDays(new Date(), 60)]);

  useEffect(() => {
    if (!cities.isFetching && cities.ids.length === 0) {
      fetchCities();
    }
  }, []);

  const onSelectContinent = c => {
    if (c === continent) {
      setContinent(null);
    } else {
      setContinent(c);
    }
  };

  useEffect(
    () => {
      fetchExploreSuggestions({
        continent: continent || undefined,
        start_date: dates[0] && format(dates[0], 'YYYY-MM-DD'),
        end_date: dates[1] && format(dates[1], 'YYYY-MM-DD')
      });
    },
    [continent, dates[0], dates[1]]
  );

  if (suggestions.isFetching) {
    return <Spinner />;
  }

  const populatedSuggestions = suggestions.ids.map(sId => {
    const s = suggestions.all[sId];
    const us = s.users.map(uId => users.all[uId]);
    return {
      city: cities.all[s.city],
      users: us
    };
  });

  return (
    <Fragment>
      <div className="uk-flex uk-flex-around">
        <DatePicker
          value={dates}
          onChange={setDates}
          hideDaysCount
          className="uk-flex-none"
        />
        <div className="uk-flex uk-flex-wrap uk-flex-wrap-top uk-flex-center">
          {CONTINENT_OPTIONS.map(c => (
            <button
              key={c.value}
              style={{
                width: 100,
                height: 40,
                fontSize: 12,
                padding: 0,
                margin: 2,
                lineHeight: '12px',
                fontWeight: 'bold'
              }}
              className={`uk-button uk-button-default ${continent === c.value &&
                'uk-button-secondary'}`}
              onClick={() => onSelectContinent(c.value)}
            >
              {c.label}
            </button>
          ))}
        </div>
      </div>
      <hr />
      <SuggestionList suggestions={populatedSuggestions} />
    </Fragment>
  );
};
// <Select
//   styles={{
//     container: () => ({
//       width: 250
//     }),
//     menu: provided => ({
//       ...provided,
//       top: null
//     })
//   }}
//   className="contient-picker"
//   isSearchAble
//   isLoading={suggestions.isFetching}
//   value={continent}
//   onChange={setContinent}
//   options={CONTINENT_OPTIONS}
// />

const SuggestionList = ({ suggestions }) => {
  if (suggestions.length === 0) {
    return (
      <div>
        No results... Try adjusting your filters or adding more{' '}
        <Link to="/friends">friends</Link>
      </div>
    );
  }
  return (
    <div>
      {suggestions.map(({ city, users }) => (
        <div
          key={city.id}
          className="uk-tile uk-padding-small uk-flex uk-flex-between uk-link-muted"
        >
          <h6>
            <ReactCountryFlag code={city.country_code} svg /> | {city.name}
          </h6>
          {users.map(u => (
            <FriendChip key={u.id} {...u} />
          ))}
        </div>
      ))}
    </div>
  );
};

const mapStateToProps = ({ cities, suggestions, users, activeUser }) => ({
  cities,
  suggestions,
  users,
  activeUser
});

export default connect(
  mapStateToProps,
  {
    fetchCities,
    fetchExploreSuggestions
  }
)(Explore);
