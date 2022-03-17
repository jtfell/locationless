import { SERVER_ERR, NOT_FOUND } from '../errorMessages';

export function reduceError(state, action) {
  const err = action.error && action.error.response;
  if (!err || err.status >= 500 || !err.status) {
    return {
      ...state,
      error: {
        status: 500,
        message: SERVER_ERR
      }
    };
  }

  if (err.status === 404) {
    return {
      ...state,
      error: {
        status: 404,
        message: NOT_FOUND
      }
    };
  }

  return {
    ...state,
    error: {
      status: err.status,
      message: (err.data && err.data.error) || SERVER_ERR
    }
  };
}
