import React, { useState, useEffect } from 'react';
import { connect } from 'react-redux';
import axios from 'axios';

import { apiUrl } from '../../config';

import FriendCard from './FriendCard.react';

function FriendSearch({
  users,
  onRequestFriend,
  onAcceptFriend,
  fetchFriends
}) {
  const [results, setResults] = useState([]);
  const [phrase, setPhrase] = useState('');

  // Each time the user types, we send a search query.
  useEffect(() => {
    const CancelToken = axios.CancelToken;
    const source = CancelToken.source();

    if (phrase) {
      axios
        .get(`${apiUrl}/u/search?q=${phrase}`, {
          cancelToken: source.token,
          withCredentials: true
        })
        .then(({ data }) => setResults(data.data))
        .catch(err => {
          // All good.
          if (err && err.message === 'unmount') {
            return;
          }

          // TODO: Handle this!
          console.log(err);
        });
    } else {
      setResults([]);
    }

    // Cancel the request if the component is unmounted
    return () => source.cancel('unmount');
  }, [phrase]);

  return (
    <div className="uk-padding-small">
      <input
        className="uk-input"
        placeholder="Search for Telegram Username / Full Name"
        value={phrase}
        onChange={e => setPhrase(e.target.value)}
      />
      <hr />
      <div className="uk-padding uk-flex uk-flex-wrap">
        {results.map(r => {
          // Get friend object so we can tell if we're friends with each search result
          const friendObj = users.all[r.id] || r;

          // Don't show yourself in search results
          if (friendObj.is_you) {
            return null;
          }

          return (
            <FriendCard
              {...friendObj}
              variant="small"
              onAccept={() => onAcceptFriend(r)}
              onRequest={() => onRequestFriend(r)}
            />
          );
        })}
      </div>
    </div>
  );
}

const mapStateToProps = ({ users }) => ({
  users
});

export default connect(
  mapStateToProps,
  {}
)(FriendSearch);
