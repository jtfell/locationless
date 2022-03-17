import React, { Fragment, useState, useEffect } from 'react';
import axios from 'axios';
import { connect } from 'react-redux';
import { withRouter, Redirect, Link } from 'react-router-dom';
// import TelegramLoginButton from 'react-telegram-login';
import { apiUrl } from '../config';
import { genInitialsAvatar } from '../helpers/avatarUtils';
import { redirectToLater } from '../actions';

import Spinner from '../components/Spinner.react';

const UserPreview = ({ match, redirectToLater }) => {
  const { username } = match.params;

  const [userId, setUserId] = useState(null);
  const [profilePic, setProfilePic] = useState(null);
  const [activeUser, setActiveUser] = useState(null);

  useEffect(
    () => {
      // Lookup the user info from public URL
      axios
        .get(`${apiUrl}/u/${username}/preview`, { withCredentials: true })
        .then(({ data }) => {
          setUserId(data.data.id);
          setProfilePic(
            data.data.photo_url || genInitialsAvatar({ name: username })
          );

          //
          // Set state so when user logs in, they are redirected back here.
          //
          redirectToLater(`/users/${data.data.id}`);
        })
        .catch(err => {
          // Handle the 404 to show that the user doesn't exist
          if (err.response && err.response.status === 404) {
            setUserId('NOT FOUND');
            setProfilePic(null);
          }

          // TODO: Proper error page pls
          setUserId('NOT FOUND');
        });
    },
    [userId]
  );

  // Check if we're logged in already
  useEffect(
    () => {
      axios
        .get(`${apiUrl}/u/`, { withCredentials: true })
        .then(({ data }) => setActiveUser(data.data.id))
        .catch(() => null); // This means we're not logged in
    },
    [1]
  );

  if (userId && activeUser) {
    return <Redirect to={`/users/${userId}`} />;
  }

  let screenContent = <Spinner />;
  // Little bit hacky but does the job. Display the 404 screen.
  if (userId === 'NOT FOUND') {
    screenContent = (
      <p>
        Oops! Looks like <b>{username}</b> doesn't have a public profile yet.
      </p>
    );
  } else if (userId) {
    screenContent = (
      <PreviewPanel username={username} profilePic={profilePic} />
    );
  }

  const logoStyles = {
    height: 55,
    paddingRight: 15,
    paddingLeft: 15,
    marginBottom: 40
  };
  return (
    <section className="uk-flex uk-flex-center">
      <div className="uk-panel uk-panel-box uk-text-center">
        <img src="/logo.png" alt="logo" style={logoStyles} />
        {screenContent}
      </div>
    </section>
  );
};

const PreviewPanel = ({ username, profilePic }) => (
  <Fragment>
    <div className="uk-flex uk-flex-center">
      <img
        className="avatar-big uk-border-circle"
        alt="avatar"
        src={profilePic}
      />
    </div>

    <hr />
    <p>
      <Link to="/login">Sign in</Link> to explore the world with{' '}
      <b>{username}</b>
    </p>
  </Fragment>
);

const mapStateToProps = () => ({});

export default connect(
  mapStateToProps,
  {
    redirectToLater
  }
)(withRouter(UserPreview));
