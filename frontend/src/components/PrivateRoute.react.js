import React from 'react';
import { Route, Redirect } from 'react-router-dom';
import { withCookies } from 'react-cookie';

const PrivateRoute = ({ component: Component, ...rest }) => (
  <Route
    {...rest}
    render={props =>
      rest.cookies.get('sessionid') ? (
        <Component {...props} />
      ) : (
        <Redirect
          to={{
            pathname: '/login'
          }}
        />
      )
    }
  />
);

export default withCookies(PrivateRoute);
