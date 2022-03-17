import { stringify } from 'query-string';
import {
  API,
  LOGOUT,
  FETCH_EXPLORE_SUGGESTIONS,
  SET_EXPLORE_SUGGESTIONS,
  FETCH_TRIPS,
  FETCH_TRIP,
  SET_TRIPS,
  CREATE_TRIP,
  UPDATE_TRIP,
  DELETE_TRIP,
  REMOVE_TRIP,
  FETCH_FRIENDS,
  CREATE_FRIEND_REQ,
  ACCEPT_FRIEND_REQ,
  FETCH_USER,
  SET_USERS,
  FETCH_CITIES,
  SET_CITIES,
  REDIRECT_TO
} from './types';

import format from 'date-fns/format';

import { apiUrl } from '../config';

export function logout() {
  return {
    type: LOGOUT
  };
}

export function redirectToLater(url) {
  return {
    type: REDIRECT_TO,
    payload: { url }
  };
}

export function setSuggestions(data) {
  return {
    type: SET_EXPLORE_SUGGESTIONS,
    payload: data
  };
}

export function fetchExploreSuggestions(data) {
  return apiAction({
    url: `${apiUrl}/explore/?${stringify(data)}`,
    onSuccess: setSuggestions,
    onFailure: () => console.log('Error occured loading explore suggestions'),
    label: FETCH_EXPLORE_SUGGESTIONS
  });
}

export function fetchFriends() {
  return apiAction({
    url: `${apiUrl}/u/friends`,
    onSuccess: setUsers,
    onFailure: () => console.log('Error occured loading friends'),
    label: FETCH_FRIENDS
  });
}

export function acceptFriendRequest(data) {
  return apiAction({
    url: `${apiUrl}/u/${data.id}/friends`,
    method: 'PUT',
    // HACK: Need the API to return the updated user object
    onSuccess: () =>
      setUsers({
        data: {
          ...data,
          is_friend: true,
          is_pending_friend: false
        }
      }),
    nextAction: fetchTrips,
    onFailure: () => console.log('Error occured accepting friendship request'),
    data: data,
    label: ACCEPT_FRIEND_REQ
  });
}

export function requestFriendship(data) {
  return apiAction({
    url: `${apiUrl}/u/${data.id}/friends`,
    method: 'POST',
    // HACK: Need the API to return the updated user object
    onSuccess: () =>
      setUsers({
        data: {
          ...data,
          have_requested_friend: true
        }
      }),
    onFailure: () => console.log('Error occured requesting friendship'),
    data: data,
    label: CREATE_FRIEND_REQ
  });
}

export function fetchUser(id) {
  return apiAction({
    url: `${apiUrl}/u/${id}`,
    onSuccess: setUsers,
    onFailure: () => console.log('Error occured loading user', id),
    label: FETCH_USER
  });
}

export function setUsers(data) {
  return {
    type: SET_USERS,
    payload: data
  };
}

export function fetchCities() {
  return apiAction({
    url: `${apiUrl}/cities/`,
    onSuccess: setCities,
    onFailure: () => console.log('Error occured loading cities'),
    label: FETCH_CITIES
  });
}

function setCities(data) {
  return {
    type: SET_CITIES,
    payload: data
  };
}

export function fetchTrip(id) {
  return apiAction({
    url: `${apiUrl}/trips/${id}`,
    onSuccess: setTrips,
    onFailure: () => console.log('Error occured loading trips'),
    label: FETCH_TRIP
  });
}

export function fetchTrips(data) {
  // Optional timeframe filter
  const qs =
    data && data.end_date_after
      ? `end_date_after=${format(data.end_date_after, 'YYYY-MM-DD')}`
      : '';

  return apiAction({
    url: `${apiUrl}/u/${data.id}/trips?${qs}`,
    onSuccess: setTrips,
    onFailure: () => console.log('Error occured loading trips'),
    label: FETCH_TRIPS
  });
}

function setTrips(data) {
  return {
    type: SET_TRIPS,
    payload: data
  };
}

function removeTrip(data) {
  return {
    type: REMOVE_TRIP,
    payload: data
  };
}

export function createTrip({ redirectOnSuccess, ...data }) {
  return apiAction({
    url: `${apiUrl}/trips`,
    method: 'POST',
    redirectOnSuccess,
    onSuccess: setTrips,
    onFailure: () => console.log('Error occured creating trip'),
    data: data,
    label: CREATE_TRIP
  });
}

export function updateTrip({ redirectOnSuccess, ...data }) {
  return apiAction({
    url: `${apiUrl}/trips/${data.id}`,
    method: 'PUT',
    redirectOnSuccess,
    onSuccess: setTrips,
    onFailure: () => console.log('Error occured updating trip'),
    data: data,
    label: UPDATE_TRIP
  });
}

export function deleteTrip({ redirectOnSuccess, ...data }) {
  return apiAction({
    url: `${apiUrl}/trips/${data.id}`,
    method: 'DELETE',
    redirectOnSuccess,
    onFailure: () => console.log('Error occured deleting trip'),
    onSuccess: () => removeTrip(data),
    label: DELETE_TRIP
  });
}

function apiAction({
  url = '',
  method = 'GET',
  data = null,
  accessToken = null,
  redirectOnSuccess = null,
  onSuccess = () => {},
  onFailure = () => {},
  nextAction = () => {},
  label = '',
  headersOverride = null
}) {
  return {
    type: API,
    payload: {
      url,
      method,
      data,
      accessToken,
      redirectOnSuccess,
      onSuccess,
      onFailure,
      label,
      headersOverride,
      nextAction
    }
  };
}
