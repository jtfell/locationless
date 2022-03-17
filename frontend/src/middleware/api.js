import axios from 'axios';
import { API } from '../actions/types';
import { accessDenied, apiError, apiStart, apiEnd } from '../actions/api';
import { redirectToLater } from '../actions';

const apiMiddleware = ({ dispatch }) => next => action => {
  if (!action) {
    return;
  }
  next(action);

  if (action.type !== API) return;

  const {
    url,
    method,
    data,
    accessToken,
    redirectOnSuccess,
    onSuccess,
    onFailure,
    label,
    headers,
    nextAction
  } = action.payload;
  const dataOrParams = ['GET', 'DELETE'].includes(method) ? 'params' : 'data';

  // axios default configs
  axios.defaults.baseURL = process.env.REACT_APP_BASE_URL || '';
  axios.defaults.headers.common['Content-Type'] = 'application/json';
  axios.defaults.headers.common['Authorization'] = `Bearer ${accessToken}`;

  if (label) {
    dispatch(apiStart(label));
  }

  axios
    .request({
      url,
      method,
      headers,
      withCredentials: true,
      [dataOrParams]: data
    })
    .then(({ data }) => {
      dispatch(onSuccess(data));
      if (redirectOnSuccess) {
        dispatch(redirectToLater(redirectOnSuccess));
      }
      dispatch(nextAction(data));
      if (label) {
        dispatch(apiEnd(label));
      }
    })
    .catch(error => {
      dispatch(apiError(error));
      dispatch(onFailure(error));

      if (error.response && error.response.status === 401) {
        dispatch(accessDenied(window.location.pathname));
      }
      if (label) {
        dispatch(apiEnd(label));
      }
    });
};

export default apiMiddleware;
