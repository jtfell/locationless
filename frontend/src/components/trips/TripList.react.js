//
// The CSS file for this component works very closely with the state here.
//
// It's mainly complicated by the different layouts on mobile and desktop, as
// some components (filters, buttons) are hidden on desktop.
//
import React, { useState } from 'react';

import addMonths from 'date-fns/add_months';
import isWithinRange from 'date-fns/is_within_range';

import {
  determineAlreadySelectedTripDates,
  newDate
} from '../../helpers/dateUtils';

import Spinner from '../Spinner.react';
import TimeSlicePicker from '../TimeSlicePicker.react';
import DatePicker from '../DatePicker.react';
import LocationMap from '../LocationMap.react';
import TripCard from '../TripCard.react';

import './TripList.scss';

const TripList = ({ user, activeUser, locations, isFetching }) => {
  //
  // State for component
  //
  const [dateSelection, setSelectedDates] = useState([
    newDate(),
    newDate(addMonths(newDate(), 3))
  ]);

  //
  // Only relevant on mobile:
  //
  // 'MAP' or 'LIST' (the CSS shows/hides the other panel)
  //
  const [activePanelOnMobile, switchPanelOnMobile] = useState('LIST');

  //
  // Loading states
  //
  if (!locations || isFetching) {
    return <Spinner />;
  }
  if (locations.length === 0) {
    return (
      <div>
        No trips yet...
        <br />
      </div>
    );
  }

  // Filter trips based on date selection
  const filteredLocations = locations.filter(isWithinMonth(dateSelection));

  // Disable adding dates that overlap with existing trips
  const disabledDates = determineAlreadySelectedTripDates(locations);

  const classes = [
    'trip-list-container',
    'uk-margin uk-flex uk-flex-between',

    // This class is used to hide one panel or the other
    activePanelOnMobile
  ];
  return (
    <div className={classes.join(' ')}>
      <div className="left-panel">
        <ListViewButton onClick={() => switchPanelOnMobile('LIST')} />
        <DateFilters
          setSelectedDates={setSelectedDates}
          dateSelection={dateSelection}
        />
        <LocationMap locations={filteredLocations} />
      </div>
      <div className="right-panel">
        <MapViewButton onClick={() => switchPanelOnMobile('MAP')} />
        <DateFilters
          setSelectedDates={setSelectedDates}
          dateSelection={dateSelection}
        />

        <div className="trip-list">
          {filteredLocations.map(trip => (
            <TripCard
              key={trip.id}
              disabledDates={disabledDates}
              {...trip}
              viewOnly={user.id !== activeUser}
            />
          ))}
        </div>
      </div>
    </div>
  );
};

const ListViewButton = ({ onClick }) => (
  <button
    className="active-panel-switch-btn uk-button uk-button-secondary"
    onClick={onClick}
  >
    Go To List View
  </button>
);

const MapViewButton = ({ onClick }) => (
  <button
    className="active-panel-switch-btn uk-button uk-button-secondary"
    onClick={onClick}
  >
    Go To Map View
  </button>
);

const DateFilters = ({ setSelectedDates, dateSelection }) => (
  <div className="date-filters uk-padding-small uk-flex uk-flex-around uk-flex-middle">
    <TimeSlicePicker value={null} onChange={setSelectedDates} />
    <DatePicker
      className="uk-width-1-3@m uk-flex uk-flex-middle uk-flex-center"
      value={dateSelection}
      onChange={setSelectedDates}
      hideDaysCount
    />
  </div>
);

const isWithinMonth = dateSelection => ({ start_date, end_date }) => {
  const [start, end] = dateSelection;

  return (
    isWithinRange(start_date, start, end) || isWithinRange(end_date, start, end)
  );
};

export default TripList;
