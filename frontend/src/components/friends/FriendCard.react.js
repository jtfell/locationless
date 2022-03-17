import React from 'react';
import { Link } from 'react-router-dom';

import FriendChip from './FriendChip.react';
import FriendStatusButton from './FriendStatusButton.react';
import TelegramLink from '../TelegramLink.react';

import './FriendCard.scss';

const FriendCard = props => {
  const fullName = `${props.first_name || ''} ${props.last_name || ''}`;
  if (props.variant && props.variant !== 'small') {
    throw new Error(`Invalid variant prop: ${props.variant}`);
  }

  const Wrapper = ({ children, is_friend, id }) =>
    is_friend ? (
      <Link
        className="uk-card-title uk-flex uk-flex-middle"
        to={`/users/${id}`}
      >
        {children}
      </Link>
    ) : (
      <div className="uk-card-title uk-flex uk-flex-middle">{children}</div>
    );

  return (
    <div
      className={`friend-card uk-card uk-card-default uk-margin ${
        props.variant
      }`}
    >
      <Wrapper is_friend={props.is_friend} id={props.id}>
        <div className="uk-width-auto uk-first-column uk-padding">
          <FriendChip {...props} />
          <span className="uk-text-small uk-padding-left uk-margin-remove-bottom">
            {fullName.toUpperCase()}
          </span>
        </div>
      </Wrapper>
      <div className="uk-padding-small uk-button-group uk-flex uk-flex-wrap">
        <FriendStatusButton
          user={props}
          onAccept={props.onAccept}
          onRequest={props.onRequest}
        />
        {props.variant !== 'small' && <TelegramLink {...props} />}
      </div>
    </div>
  );
};

export default FriendCard;
