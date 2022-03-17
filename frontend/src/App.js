import React, { Component, Fragment } from 'react';
import { connect } from 'react-redux';
import { withCookies } from 'react-cookie';
import { withRouter, Route, Redirect, Switch } from 'react-router-dom';
import axios from 'axios';
import * as Sentry from '@sentry/browser';

import TripDetail from './pages/TripDetail.react';
import NewTrip from './pages/NewTrip.react';
import EditTrip from './pages/EditTrip.react';

import FriendsList from './pages/FriendsList.react';
import UserProfile from './pages/UserProfile.react';
import Explore from './pages/Explore.react';
import UserPreview from './pages/UserPreview.react';

import PrivateRoute from './components/PrivateRoute.react';
import Login from './components/Login.react';
import Header from './components/Header.react';
import Spinner from './components/Spinner.react';

import { setUsers, logout, redirectToLater } from './actions';
import { NOT_FOUND, SERVER_ERR } from './errorMessages';

import { apiUrl, gaEnabled, sentryDsn } from './config';

import './App.scss';

Sentry.init({ dsn: sentryDsn });

//
// Defines the publically accessible pages. (Should be defined
// as <Route /> not <PrivateRoute /> below)
//
const isPublicUrl = url => url === '/login' || url.indexOf('/u/') === 0;

class App extends Component {
  constructor(props) {
    super(props);

    this.state = {
      errorMsg: null
    };
  }

  onLogout() {
    this.props.cookies.remove('sessionid');
    this.props.logout();
    axios.post(`${apiUrl}/u/logout/`, { withCredentials: true });
    this.props.history.push('/login');
  }

  onLogin(data) {
    this.props.history.push('/');
    this.props.setUsers({
      data: {
        ...data,
        is_you: true
      }
    });
  }

  static getDerivedStateFromError(error) {
    // Update state so the next render will show the fallback UI.
    return { errorMsg: SERVER_ERR };
  }

  componentDidCatch(error, errorInfo) {
    Sentry.withScope(scope => {
      Object.keys(errorInfo).forEach(k => scope.setExtra(k, errorInfo[k]));
      Sentry.captureException(error);
    });
  }

  componentDidMount() {
    this.checkLoginStatus();
  }

  componentDidUpdate() {
    this.checkLoginStatus();
  }

  checkLoginStatus() {
    const { activeUser, redirectTo, location, history } = this.props;

    if (window.gtag && gaEnabled) {
      history.listen(location => {
        window.gtag('set', { user: activeUser });
        window.gtag('event', 'page_view', { pathname: location.pathname });
      });
    }

    if (!activeUser && !isPublicUrl(location.pathname)) {
      axios
        .get(`${apiUrl}/u/`, { withCredentials: true })
        .then(({ data }) => this.props.setUsers({ data: data.data }))
        .catch(err => {
          this.props.history.push('/login');
        });
    }

    if (redirectTo === location.pathname) {
      this.props.redirectToLater(null);
    }
  }

  renderAuthorTag() {
    return (
      <div className="author">
        <a href="https://jtfell.github.io" target="_blank" rel="noopener">
          by jtfell
        </a>
      </div>
    );
  }

  render() {
    const { activeUser, redirectTo, location } = this.props;
    if (this.props.error.message || this.state.errorMsg) {
      return (
        <ErrorPage msg={this.props.error.message || this.state.errorMsg} />
      );
    }

    // Show spinner while loading the current user
    if (!activeUser && !isPublicUrl(location.pathname)) {
      return (
        <Fragment>
          <Spinner />
          {this.renderAuthorTag()}
        </Fragment>
      );
    }

    if (activeUser && redirectTo && redirectTo !== location.pathname) {
      return <Redirect to={redirectTo} />;
    }

    //
    // NOTE: Public routes need to be accounted for in the `isPublicUrl` function
    //       at the top of this file.
    //
    return (
      <Fragment>
        <Header onLogout={this.onLogout.bind(this)} activeUser={activeUser} />
        <main className="uk-container uk-container-small uk-container-center uk-padding">
          <Switch>
            <Route
              path="/login"
              component={() => <Login onLogin={this.onLogin.bind(this)} />}
            />
            <Route path="/u/:username" component={UserPreview} />

            <PrivateRoute
              exact
              path="/"
              component={() => (
                <Redirect to={`/users/${this.props.activeUser}`} />
              )}
            />
            <PrivateRoute path="/trips/new" component={NewTrip} />
            <PrivateRoute path="/trips/:id/edit" component={EditTrip} />
            <PrivateRoute path="/trips/:id" component={TripDetail} />

            <PrivateRoute exact path="/friends" component={FriendsList} />
            <PrivateRoute exact path="/explore" component={Explore} />
            <PrivateRoute path="/users/:id" component={UserProfile} />
            <Route component={() => <ErrorPage msg={NOT_FOUND} hideLogo />} />
          </Switch>
        </main>

        {this.renderAuthorTag()}
      </Fragment>
    );
  }
}

const ErrorPage = ({ msg, hideLogo }) => (
  <main className="uk-container uk-container-small uk-container-center uk-padding">
    {!hideLogo && (
      <a href="/">
        <img src="/logo.png" alt="logo" style={{ height: 85, padding: 15 }} />
      </a>
    )}
    <hr />
    <h4>{msg}</h4>
    <a onClick={() => Sentry.showReportDialog()}>Report feedback</a>
  </main>
);

const mapStateToProps = ({ users, activeUser, error, redirectTo }) => ({
  users,
  activeUser,
  error,
  redirectTo
});

export default withRouter(
  withCookies(
    connect(
      mapStateToProps,
      {
        setUsers,
        logout,
        redirectToLater
      }
    )(App)
  )
);
