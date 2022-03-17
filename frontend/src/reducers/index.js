import {
  API_START,
  API_END,
  API_ERROR,
  LOGOUT,
  SET_USERS,
  FETCH_USER,
  SET_TRIPS,
  FETCH_TRIPS,
  REMOVE_TRIP,
  FETCH_FRIENDS,
  SET_CITIES,
  FETCH_CITIES,
  SET_EXPLORE_SUGGESTIONS,
  REDIRECT_TO
} from '../actions/types';

import { reduceError } from './util';

import { normalize, schema } from 'normalizr';

const user = new schema.Entity('users');
const city = new schema.Entity('cities');
const match = new schema.Entity(
  'matches',
  {
    user
  },
  {
    idAttribute: v => `${v.user.id}::${v.start_date}::${v.end_date}`
  }
);

const trip = new schema.Entity(
  'trips',
  {
    user,
    city,
    matches: [match]
  },
  {
    processStrategy: (value, parent, key) => {
      const matches = value.matches || [];
      return { ...value, matches };
    }
  }
);

const suggestion = new schema.Entity(
  'suggestions',
  {
    city,
    users: [user]
  },
  {
    idAttribute: v => v.city.id
  }
);

const schemas = {
  users: user,
  cities: city,
  trips: trip,
  suggestions: suggestion
};

const initialState = {
  activeUser: null,
  redirectTo: null,

  error: {
    status: null,
    message: null
  },

  trips: {
    ids: [],
    all: {},
    isFetching: false,
    hasFetched: false
  },
  cities: {
    ids: [],
    all: {},
    isFetching: false,
    hasFetched: false
  },
  users: {
    ids: [],
    all: {},
    isFetching: false,
    hasFetched: false
  },
  suggestions: {
    ids: [],
    all: {},
    isFetching: false,
    hasFetched: false
  }
};

const ENTITIES = ['trips', 'cities', 'users', 'suggestions'];

const insertEntities = (state, data, entity, overwrite = false) => {
  // Make sure we have an array
  if (!Array.isArray(data)) {
    data = [data];
  }

  // Normalise the data
  const normData = normalize(data, [schemas[entity]]);

  const newState = { ...state };
  Object.keys(state).forEach(e => {
    if (!state[e] || ENTITIES.indexOf(e) === -1) {
      return;
    }

    newState[e] = {
      ...state[e]
    };

    // Merge in all the new objects for each entity
    const allIds = [
      ...new Set(
        Object.keys(state[e].all).concat(
          Object.keys(normData.entities[e] || {})
        )
      )
    ];
    allIds.forEach(id => {
      newState[e].all[id] = Object.assign(
        {},
        state[e].all[id],
        (normData.entities[e] || {})[id]
      );
    });
  });

  if (!overwrite) {
    // Add all the new IDs to the primary entity
    const existingIds = state[entity].ids;
    newState[entity].ids = [...new Set(existingIds.concat(normData.result))];
  } else {
    newState[entity].ids = normData.result;
  }

  return newState;
};

const TRIPS_LABELS = [FETCH_TRIPS];
const CITIES_LABELS = [FETCH_CITIES];
const USERS_LABELS = [FETCH_USER, FETCH_FRIENDS];

export default function(state = initialState, action) {
  switch (action.type) {
    case LOGOUT:
      return initialState;
    case API_ERROR:
      return reduceError(state, action);
    case REDIRECT_TO:
      return {
        ...state,
        redirectTo: action.payload.url
      };

    case REMOVE_TRIP: {
      const trips = Object.assign({}, state.trips);
      trips.ids = trips.ids.filter(tId => tId !== action.payload.id);
      delete trips.all[action.payload.id];

      return {
        ...state,
        trips
      };
    }

    case SET_EXPLORE_SUGGESTIONS:
      return insertEntities(
        state,
        action.payload.data,
        'suggestions',
        !!'overwrite'
      );

    case SET_TRIPS:
      return insertEntities(state, action.payload.data, 'trips');

    case SET_CITIES:
      return insertEntities(state, action.payload.data, 'cities');

    case SET_USERS: {
      const newState = insertEntities(state, action.payload.data, 'users');

      // Save the user ID of the logged in user
      if (action.payload && action.payload.data && action.payload.data.is_you) {
        newState.activeUser = action.payload.data.id;
      }

      return newState;
    }

    case API_START:
      if (TRIPS_LABELS.indexOf(action.payload) > -1) {
        return {
          ...state,
          trips: {
            ...state.trips,
            isFetching: true
          }
        };
      }
      if (USERS_LABELS.indexOf(action.payload) > -1) {
        return {
          ...state,
          users: {
            ...state.users,
            isFetching: true
          }
        };
      }
      if (CITIES_LABELS.indexOf(action.payload) > -1) {
        return {
          ...state,
          cities: {
            ...state.cities,
            isFetching: true
          }
        };
      }
      return state;

    case API_END:
      if (TRIPS_LABELS.indexOf(action.payload) > -1) {
        return {
          ...state,
          trips: {
            ...state.trips,
            isFetching: false,
            hasFetched: true
          }
        };
      }
      if (USERS_LABELS.indexOf(action.payload) > -1) {
        return {
          ...state,
          users: {
            ...state.users,
            isFetching: false,
            hasFetched: true
          }
        };
      }
      if (CITIES_LABELS.indexOf(action.payload) > -1) {
        return {
          ...state,
          cities: {
            ...state.cities,
            isFetching: false,
            hasFetched: true
          }
        };
      }
      return state;

    default:
      return state;
  }
}
