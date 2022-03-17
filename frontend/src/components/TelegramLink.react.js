import React from 'react';

import './TelegramLink.scss';

const TelegramLink = ({ username }) => {
  if (!username) {
    return null;
  }
  return (
    <a
      className="uk-button uk-button-default telegram-link"
      href={`https://t.me/${username}`}
      target="_blank"
      rel="no-opener"
    >
      <img src="/telegram.svg" alt="telegram-icon" />
      <span>Telegram</span>
    </a>
  );
};

export default TelegramLink;
