import React, { useEffect } from 'react';
import { connect } from 'react-redux';
import { Link } from 'react-router-dom';

import isBefore from 'date-fns/is_before';
import {
  fetchUser,
  fetchTrips,
  requestFriendship,
  acceptFriendRequest
} from '../actions';

import { genInitialsAvatar } from '../helpers/avatarUtils';

import Spinner from '../components/Spinner.react';
import TripList from '../components/trips/TripList.react';
import TelegramLink from '../components/TelegramLink.react';
import FriendStatusButton from '../components/friends/FriendStatusButton.react';

const UserProfile = ({
  match,
  users,
  fetchUser,
  fetchTrips,
  trips,
  cities,
  activeUser,
  requestFriendship,
  acceptFriendRequest
}) => {
  const userId = parseInt(match.params.id, 10);

  // Clone the trip
  let user = users.all[userId];

  useEffect(() => {
    // Get it from the API if:
    //   - it is not currently fetching users
    if (!users.isFetching && userId !== activeUser) {
      fetchUser(userId);
    }
    fetchTrips({ id: userId });
  }, [userId]);

  if (users.isFetching || !user) {
    return <Spinner />;
  }

  const populatedTrips = trips.ids
    .map(tId => {
      const trip = trips.all[tId];
      return {
        ...trip,
        city: cities.all[trip.city],
        matches: trip.matches.map(match => {
          const [uId, start, end] = match.split('::');
          return {
            ...users.all[uId],
            start_date: start,
            end_date: end
          };
        })
      };
    })
    .filter(t => t.user === userId);

  // Sort so the soonest trip is first in the list
  populatedTrips.sort((a, b) =>
    isBefore(a.start_date, b.start_date) ? -1 : 1
  );

  const name = `${user.first_name} ${user.last_name}`;

  const onRequest = () => requestFriendship(user);
  const onAccept = () => acceptFriendRequest(user);
  return (
    <div>
      <div className="centered-hero">
        <div className="uk-flex uk-flex-left profile-details">
          <div className="uk-flex uk-flex-left">
            <img
              className="avatar-big uk-border-circle"
              alt="avatar"
              src={user.photo_url || genInitialsAvatar({ name })}
            />

            <div className="uk-flex uk-flex-column uk-padding">
              {user.username && <h4>@{user.username}</h4>}
              <span>
                {user.first_name.toUpperCase()}{' '}
                {(user.last_name || '').toUpperCase()}
              </span>
            </div>
          </div>

          <div className="user-icons">
            <FriendStatusButton
              user={user}
              onRequest={onRequest}
              onAccept={onAccept}
            />
          </div>
          {!user.is_you && (
            <div className="user-icons">
              <TelegramLink username={user.username} />
            </div>
          )}
        </div>

        {user.id === activeUser && (
          <Link to="/trips/new" className="uk-button uk-button-default">
            <span className="icono-plus icon-sm" /> ADD NEW TRIP
          </Link>
        )}
        <hr />
      </div>
      <TripList
        user={user}
        locations={populatedTrips}
        activeUser={activeUser}
        isFetching={trips.isFetching}
      />
    </div>
  );
};

const mapStateToProps = ({ cities, trips, users, activeUser }) => ({
  users,
  trips,
  cities,
  activeUser
});

export default connect(
  mapStateToProps,
  {
    fetchUser,
    fetchTrips,
    requestFriendship,
    acceptFriendRequest
  }
)(UserProfile);
