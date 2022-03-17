import React from 'react';
import { Link } from 'react-router-dom';
import { genInitialsAvatar } from '../../helpers/avatarUtils';

import './FriendChip.scss';

const FriendChip = ({
  id,
  first_name,
  last_name,
  username,
  photo_url,
  photoOnly
}) => {
  const name = `${first_name} ${last_name || ''}`;
  const photoSrc = photo_url || genInitialsAvatar({ name });
  const tooltipText = username || name;

  if (photoOnly) {
    return (
      <div className="uk-border-circle friend-chip">
        <img
          className="uk-border-circle avatar-preview"
          alt="avatar"
          src={photoSrc}
        />
      </div>
    );
  }

  return (
    <Link className="uk-border-circle friend-chip" to={`/users/${id}`}>
      <img
        className="uk-border-circle avatar-preview"
        alt="avatar"
        src={photoSrc}
      />
      <div className="uk-badge tooltip">{tooltipText}</div>
    </Link>
  );
};

export default FriendChip;
