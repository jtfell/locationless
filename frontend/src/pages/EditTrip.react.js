import React, { useEffect } from 'react';
import { connect } from 'react-redux';
import { withRouter, Redirect } from 'react-router-dom';

import isBefore from 'date-fns/is_before';
import { fetchTrips, updateTrip } from '../actions';

import Spinner from '../components/Spinner.react';
import TripForm from '../components/trips/TripForm.react';

const EditTripForm = ({
  fetchTrips,
  updateTrip,
  trips,
  activeUser,
  history,
  match
}) => {
  const activeTrip = trips.all[match.params.id];

  useEffect(() => {
    fetchTrips({ id: activeUser });
  }, [activeUser]);

  if (trips.isFetching || !trips.hasFetched) {
    return <Spinner />;
  }

  if (!activeTrip) {
    return <Redirect to="/404" />;
  }

  const populatedTrips = trips.ids
    .map(tId => trips.all[tId])
    .filter(t => t.id !== parseInt(match.params.id, 10))
    .filter(t => t.user === activeUser);

  const updateTripAndRedirect = trip => {
    updateTrip({
      ...trip,
      id: activeTrip.id,
      redirectOnSuccess: `/trips/${activeTrip.id}`
    });
  };

  return (
    <div>
      <TripForm
        trips={populatedTrips}
        onSubmit={updateTripAndRedirect}
        initialDates={[activeTrip.start_date, activeTrip.end_date]}
        initialCity={activeTrip.city}
      />
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
      updateTrip
    }
  )(EditTripForm)
);
