import React, { Fragment, useState } from 'react';
import { connect } from 'react-redux';
import { withRouter, Link } from 'react-router-dom';

import format from 'date-fns/format';
import addDays from 'date-fns/add_days';
import subDays from 'date-fns/sub_days';

import ReactCountryFlag from 'react-country-flag';
import FriendChip from './friends/FriendChip.react';
import DatePicker from './DatePicker.react';
import { updateTrip } from '../actions';

function TripCard({
  updateTrip,
  disabledDates,
  viewOnly,
  isHighlighted,
  ...trip
}) {
  const { id, city, start_date, end_date, matches } = trip;

  //
  // State for component
  //
  const [dates, setDates] = useState([start_date, end_date]);

  //
  // Update the dates in the redux store and send off an API request
  //
  const updateDates = d => {
    setDates(d);
    if (d.length !== 2) {
      return;
    }
    return updateTrip({
      id,
      city: city.id,
      start_date: format(d[0], 'YYYY-MM-DD'),
      end_date: format(d[1], 'YYYY-MM-DD')
    });
  };

  //
  // Empty card state
  //
  if (!id) {
    return <NoTripSelected />;
  }

  const disabledDatesMinusThoseForThisTrip = (disabledDates || []).filter(
    ({ from, to }) =>
      !(
        format(addDays(start_date, 1), 'YYYY-MM-DD') ===
          format(from, 'YYYY-MM-DD') &&
        format(subDays(end_date, 1), 'YYYY-MM-DD') === format(to, 'YYYY-MM-DD')
      )
  );

  const DivOrLink = viewOnly ? 'div' : Link;
  return (
    <Fragment>
      <div className="uk-tile uk-padding-small uk-flex uk-flex-between uk-link-muted">
        <div>
          <DivOrLink to={`/trips/${id}`}>
            <h6 className="trip-title">
              <ReactCountryFlag code={city.country_code} svg /> | {city.name}
            </h6>
          </DivOrLink>
          {/* DISBABLED FOR NOW, not sure if I still want this functionality */}
          <DatePicker
            disabledDates={disabledDatesMinusThoseForThisTrip}
            value={dates}
            onChange={updateDates}
            disabled={true}
          />
        </div>
        <div className="uk-flex">
          <span className="uk-flex">
            {matches.slice(0, 3).map(m => (
              <FriendChip key={m.id} {...m} />
            ))}
            <ExtraFriends allFriends={matches} numDisplayed={3} />
          </span>
        </div>
      </div>
    </Fragment>
  );
}

const ExtraFriends = ({ allFriends, numDisplayed }) => {
  if (allFriends.length === 0) {
    return (
      <div className="uk-border-circle avatar-preview uk-background-muted">
        <p className="uk-border-circle uk-dark">-</p>
      </div>
    );
  }
  if (allFriends.length > numDisplayed) {
    return (
      <div className="uk-border-circle avatar-preview uk-background-muted">
        <p className="uk-border-circle uk-dark">
          +{allFriends.length - numDisplayed}
        </p>
      </div>
    );
  }
  return null;
};

const NoTripSelected = () => (
  <Fragment>
    <div className="uk-tile uk-padding-small uk-flex uk-flex-between uk-link-muted">
      <div>
        <h6>No Trip Selected</h6>
      </div>
      <span className="uk-flex">
        <hr className="uk-divider-vertical uk-margin-right uk-margin-left" />
        <span className="uk-flex uk-flex-column" />
      </span>
    </div>
    <hr />
  </Fragment>
);

const mapStateToProps = () => ({});

export default connect(
  mapStateToProps,
  {
    updateTrip
  }
)(withRouter(TripCard));
