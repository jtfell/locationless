import React from 'react';

const FriendshipStatus = ({ user, onRequest, onAccept }) => {
  if (user && user.is_you) {
    return null;
    // (
    //   <button className="uk-button uk-flex" disabled>
    //     <div className="icon-sm">
    //       <span className="icono-smile" />
    //     </div>
    //     <div>You</div>
    //   </button>
    // );
  }
  if (user && user.is_friend) {
    return (
      <button className="uk-button uk-flex uk-button-default" disabled>
        <div className="icon-sm">
          <span className="icono-check" />
        </div>
        <div>Friends</div>
      </button>
    );
  }
  if (user && user.is_pending_friend) {
    return (
      <button
        onClick={onAccept}
        className="uk-button uk-flex uk-button-default"
      >
        <div className="icon-sm">
          <span className="icono-check" />
        </div>
        <div>Accept</div>
      </button>
    );
  }
  if (user && user.have_requested_friend) {
    return (
      <button className="uk-button uk-flex uk-button-default" disabled>
        <div className="icon-sm">
          <span className="icono-clock" />
        </div>
        <div>Waiting</div>
      </button>
    );
  }

  return (
    <button className="uk-button uk-flex uk-button-default" onClick={onRequest}>
      <div className="icon-sm">
        <span className="icono-plus" />
      </div>
      <div>Request</div>
    </button>
  );
};

export default FriendshipStatus;
