import React, { Fragment, useEffect } from 'react';
import ReactCountryFlag from 'react-country-flag';

import { Redirect, withRouter } from 'react-router-dom';
import { connect } from 'react-redux';
import { fetchTrip, deleteTrip } from '../actions';

import OverlappingFriends from '../components/trips/OverlappingFriends.react';
import Spinner from '../components/Spinner.react';

const TripDetail = ({
  activeUser,
  match,
  trips,
  cities,
  users,
  history,
  fetchTrip,
  deleteTrip
}) => {
  const tripId = parseInt(match.params.id, 10);

  // Clone the trip
  let trip = trips.all[tripId];
  if (trip) {
    trip = {
      ...trip,
      city: cities[trip.city],
      matches: trip.matches.map(match => {
        const [uId, start, end] = match.split('::');
        return {
          ...users.all[uId],
          start_date: start,
          end_date: end
        };
      })
    };
    if (trip.user !== activeUser) {
      return <Redirect to="/404" />;
    }
  }

  useEffect(() => {
    // Get it from the API if:
    //   - it is not currently fetching trips
    //   - we don't have the trip in the store already
    if (!trips.isFetching && !trip) {
      fetchTrip(tripId);
    }
  }, []);

  const onEditTrip = () => history.push(`/trips/${tripId}/edit`);
  const onDeleteTrip = () => {
    if (window.confirm('Are you sure you want to delete this trip?')) {
      deleteTrip({
        id: tripId,
        redirectOnSuccess: '/'
      });
    }
  };

  const user = users.all[activeUser];

  return (
    <Fragment>
      <MainPanel
        isFetching={trips.isFetching}
        user={user}
        trip={trip}
        onEditTrip={onEditTrip}
        onDeleteTrip={onDeleteTrip}
      />
    </Fragment>
  );
};

const MainPanel = ({ isFetching, user, trip, onEditTrip, onDeleteTrip }) => {
  if (isFetching || !trip) {
    return <Spinner />;
  }

  return (
    <Fragment>
      <h2 className="uk-flex uk-flex-middle uk-flex-center">
        <ReactCountryFlag code={trip.city.country_code} svg />
        <span style={{ paddingLeft: 6 }}>{trip.city.name.toUpperCase()}</span>
      </h2>

      <div className="uk-flex trip-actions uk-flex-around">
        <button onClick={onEditTrip} className="uk-button uk-button-default">
          <span className="icono-sliders icon-sm" /> EDIT
        </button>
        <button onClick={onDeleteTrip} className="uk-button uk-button-default">
          <span className="icono-trash icon-sm" /> DELETE
        </button>
      </div>

      <br />

      <h5>YOUR DETAILS</h5>
      <OverlappingFriends {...trip} user={user} />
    </Fragment>
  );
};

const mapStateToProps = ({ activeUser, trips, users, cities }) => ({
  activeUser,
  trips,
  users,
  cities: cities.all
});

export default withRouter(
  connect(
    mapStateToProps,
    {
      fetchTrip,
      deleteTrip
    }
  )(TripDetail)
);
