import React, { Fragment, useEffect } from 'react';
import { connect } from 'react-redux';
import {
  fetchFriends,
  requestFriendship,
  acceptFriendRequest
} from '../actions';

import Spinner from '../components/Spinner.react';
import FriendCard from '../components/friends/FriendCard.react';
import FriendSearch from '../components/friends/FriendSearch.react';

const FriendsList = ({
  users,
  fetchFriends,
  requestFriendship,
  acceptFriendRequest,
  activeUser
}) => {
  useEffect(() => {
    if (!users.isFetching) {
      fetchFriends();
    }
  }, []);

  return (
    <Fragment>
      <MainPanel
        isFetching={users.isFetching && users.ids.length === 0}
        users={users}
        activeUser={activeUser}
        acceptFriendRequest={acceptFriendRequest}
        requestFriendship={requestFriendship}
      />
    </Fragment>
  );
};

const MainPanel = ({
  isFetching,
  users,
  activeUser,
  acceptFriendRequest,
  requestFriendship
}) => {
  if (isFetching) {
    return <Spinner />;
  }

  const friendList = Object.keys(users.all)
    .filter(uId => parseInt(uId, 10) !== activeUser)
    .map(uId => users.all[uId]);

  const accepted = friendList.filter(f => f.is_friend);
  const hasFriends = accepted.length > 0;
  const pending = friendList.filter(f => f.is_pending_friend);
  const hasPendingFriends = pending.length > 0;

  return (
    <Fragment>
      {hasPendingFriends && <h4>PENDING REQUESTS</h4>}
      <div className="uk-flex uk-flex-wrap">
        {pending.map(f => (
          <FriendCard
            key={f.id}
            {...f}
            onAccept={() => acceptFriendRequest(f)}
          />
        ))}
      </div>
      {hasFriends && <h4>FRIENDS</h4>}
      <div className="uk-flex uk-flex-wrap">
        {accepted.map(f => (
          <FriendCard key={f.id} {...f} />
        ))}
      </div>
      <h4>FIND FRIENDS</h4>
      <FriendSearch
        users={users}
        onRequestFriend={requestFriendship}
        onAcceptFriend={acceptFriendRequest}
      />
    </Fragment>
  );
};

const mapStateToProps = ({ users, activeUser }) => ({
  users,
  activeUser
});

export default connect(
  mapStateToProps,
  {
    fetchFriends,
    requestFriendship,
    acceptFriendRequest
  }
)(FriendsList);
