import React, { Fragment } from 'react';

import FriendChip from '../friends/FriendChip.react';
import DateRange from '../DateRange.react';

import differenceInDays from 'date-fns/difference_in_days';
import getOverlappingDaysInRanges from 'date-fns/get_overlapping_days_in_ranges';

import './OverlappingFriends.scss';

const OverlappingFriends = ({
  city,
  start_date,
  end_date,
  matches,
  isActive,
  user
}) => (
  <Fragment>
    <OverlapGraph
      {...user}
      start_date={start_date}
      end_date={end_date}
      to={end_date}
      from={start_date}
    />
    <h5>OVERLAPPING FRIENDS</h5>
    <FriendsBars
      matches={matches}
      start_date={start_date}
      end_date={end_date}
    />
  </Fragment>
);

const FriendsBars = ({ matches, start_date, end_date }) => {
  if (matches.length === 0) {
    return (
      <p>
        None of your friends trips are currently overlapping with this one.
        Maybe you need to add some people you know to improve your chances?
      </p>
    );
  }

  return (
    <Fragment>
      {matches.map(m => (
        <OverlapGraph key={m.id} {...m} from={start_date} to={end_date} />
      ))}
    </Fragment>
  );
};

// <Link to={`/users/${id}`} className="chip-label uk-link-muted">
//   @{username || `${first_name} ${last_name || ''}`}
// </Link>
const OverlapGraph = ({
  id,
  first_name,
  last_name,
  username,
  photo_url,
  start_date,
  end_date,
  from,
  to
}) => (
  <div className="uk-flex uk-flex-middle friend-container">
    <FriendChip
      id={id}
      username={username}
      first_name={first_name}
      last_name={last_name}
      photo_url={photo_url}
    />
    <OverlapBar
      start_date={start_date}
      end_date={end_date}
      to={to}
      from={from}
    />
  </div>
);

const OverlapBar = ({ id, start_date, end_date, from, to }) => {
  const totalLength = differenceInDays(to, from);
  const daysFromStart = differenceInDays(start_date, from);
  const daysOverlap = getOverlappingDaysInRanges(
    from,
    to,
    start_date,
    end_date
  );

  const STYLES = {
    width: `${Math.floor((daysOverlap * 100) / totalLength)}%`,
    marginLeft: `${Math.max(
      Math.floor((daysFromStart * 100) / totalLength) - 5,
      0
    )}%`
  };

  return (
    <div className="overlap-bar-container">
      <div className="overlap-bar" style={STYLES}>
        <DateRange
          style={{}}
          className="uk-flex uk-flex-between"
          start_date={start_date}
          end_date={end_date}
          hideDaysCount
        />
      </div>
    </div>
  );
};

export default OverlappingFriends;
