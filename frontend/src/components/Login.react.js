import React, { Component } from 'react';
import { withRouter } from 'react-router-dom';
import axios from 'axios';
import { stringify } from 'query-string';
import TelegramLoginButton from 'react-telegram-login';

import { apiUrl, botName } from '../config';

class Login extends Component {
  state = {
    errMsg: null
  };

  handleResponse = data => {
    return axios({
      url: `${apiUrl}/u/auth?${stringify(data)}`,
      method: 'get',
      withCredentials: true
    })
      .then(res => {
        // Pass the user data back to the main app
        this.props.onLogin(data);
      })
      .catch(err => {
        if (err.response) {
          const errMsg =
            (err.response.data &&
              err.response.data.errors &&
              err.response.data.errors[0]) ||
            err.response;

          this.setState({ errMsg });
        } else {
          this.setState({ errMsg: 'Unable to connect to the API' });
        }
      });
  };

  render() {
    return (
      <section className="uk-flex uk-flex-center">
        <div className="uk-panel uk-panel-box uk-text-center telegram-login-container">
          <img
            src="/logo.png"
            alt="logo"
            style={{ height: 55, paddingRight: 15, paddingLeft: 15 }}
          />
          <h1>LOCATIONLESS</h1>
          <hr />
          <TelegramLoginButton
            dataOnauth={this.handleResponse}
            botName={botName}
          />
          <p>
            You will need an account on{' '}
            <a target="_blank" href="https://telegram.org" rel="no-opener">
              Telegram
            </a>{' '}
            to get started
          </p>
        </div>
      </section>
    );
  }
}

export default withRouter(Login);
