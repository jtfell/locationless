import React, { useEffect } from 'react';
import { connect } from 'react-redux';
import { withRouter } from 'react-router-dom';

import isBefore from 'date-fns/is_before';
import { fetchTrips, createTrip } from '../actions';

import Spinner from '../components/Spinner.react';
import TripForm from '../components/trips/TripForm.react';

const NewTripForm = ({
  fetchTrips,
  createTrip,
  trips,
  activeUser,
  history
}) => {
  useEffect(() => {
    fetchTrips({ id: activeUser });
  }, [activeUser]);

  if (trips.isFetching) {
    return <Spinner />;
  }

  const populatedTrips = trips.ids
    .map(tId => trips.all[tId])
    .filter(t => t.user === activeUser);

  const createTripAndRedirect = trip => {
    createTrip({
      ...trip,
      redirectOnSuccess: '/'
    });
  };

  // Sort so the soonest trip is first in the list
  populatedTrips.sort((a, b) =>
    isBefore(a.start_date, b.start_date) ? -1 : 1
  );

  return (
    <div>
      <TripForm trips={populatedTrips} onSubmit={createTripAndRedirect} />
    </div>
  );
};

const mapStateToProps = ({ trips, activeUser }) => ({
  trips,
  activeUser
});

export default withRouter(
  connect(
    mapStateToProps,
    {
      fetchTrips,
      createTrip
    }
  )(NewTripForm)
);
