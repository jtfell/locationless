import React, { useState, Fragment } from 'react';
import { Link, withRouter } from 'react-router-dom';
import { withCookies } from 'react-cookie';

const Header = ({ cookies, onLogout, activeUser, history }) => {
  if (!activeUser || !cookies.get('sessionid')) {
    return null;
  }
  return (
    <div className="uk-navbar-container">
      <nav className="navbar-main uk-navbar uk-container">
        <div className="uk-navbar-left">
          <MobileNavbar
            activeUser={activeUser}
            onLogout={onLogout}
            history={history}
          />
          <Link className="uk-navbar-item uk-logo" to="/">
            <img
              src="/logo.png"
              alt="logo"
              style={{ height: 35, paddingRight: 15, paddingLeft: 15 }}
            />
            Locationless
          </Link>
        </div>

        <DesktopNavbar activeUser={activeUser} onLogout={onLogout} />
      </nav>
    </div>
  );
};

const MobileNavbar = ({ activeUser, onLogout, history }) => {
  const [isOpen, setIsOpen] = useState(false);

  const navigateTo = to => {
    setIsOpen(false);
    history.push(to);
  };
  const logout = () => {
    setIsOpen(false);
    onLogout();
  };

  return (
    <Fragment>
      <div
        className="uk-navbar-item uk-hidden@s"
        onClick={() => setIsOpen(!isOpen)}
      >
        <div className="icono-hamburger" onClick={() => setIsOpen(!isOpen)} />
      </div>
      {isOpen && (
        <div className="fullscreen-mobile-menu">
          <ul className="uk-list">
            <li className="uk-navbar-item" onClick={() => setIsOpen(!isOpen)}>
              <div className="icono-cross" />
            </li>
            <li
              className="uk-navbar-item"
              onClick={() => navigateTo('/explore')}
            >
              <h4>EXPLORE</h4>
            </li>
            <li
              className="uk-navbar-item"
              onClick={() => navigateTo('/friends')}
            >
              <h4>FRIENDS</h4>
            </li>
            <li
              className="uk-navbar-item"
              onClick={() => navigateTo(`/users/${activeUser}`)}
            >
              <h4>PROFILE</h4>
            </li>
            <li className="uk-navbar-item" onClick={() => logout()}>
              <h4>LOGOUT</h4>
            </li>
          </ul>
        </div>
      )}
    </Fragment>
  );
};

const DesktopNavbar = ({ activeUser, onLogout }) => (
  <div className="uk-navbar-right uk-visible@s">
    <ul className="uk-navbar-nav">
      <li className="uk-navbar-item">
        <Link to="/explore">Explore</Link>
      </li>
      <li className="uk-navbar-item">
        <Link to="/friends">Friends</Link>
      </li>
      <li className="uk-navbar-item">
        <Link to={`/users/${activeUser}`}>Profile</Link>
      </li>
      <li className="uk-navbar-item">
        <a onClick={onLogout}>Logout</a>
      </li>
    </ul>
  </div>
);

export default withRouter(withCookies(Header));
