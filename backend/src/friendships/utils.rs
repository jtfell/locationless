use friendships::models::FriendshipPopulated;
use users::models::UserPopulated;

// Convert a friendship struct into a populated user struct
pub fn gen_populated_user(active_user_id: i32, f: FriendshipPopulated) -> UserPopulated {
    let is_pending_friend;
    let have_requested_friend;

    let user = if f.user_b.id == active_user_id {
        // If user A is me:
        is_pending_friend = f.user_a_accepted && !f.user_b_accepted;
        have_requested_friend = !f.user_a_accepted && f.user_b_accepted;

        f.user_a
    } else {
        // If user B is me:
        is_pending_friend = !f.user_a_accepted && f.user_b_accepted;
        have_requested_friend = f.user_a_accepted && !f.user_b_accepted;

        f.user_b
    };

    UserPopulated {
        id: user.id,
        first_name: user.first_name,
        last_name: user.last_name,
        username: user.username,
        photo_url: user.photo_url,
        created: user.created,
        updated: user.updated,
        is_friend: (f.user_a_accepted && f.user_b_accepted),
        is_pending_friend,
        have_requested_friend,
        is_you: false,
    }
}

// TODO: Where are these in the STD lib?
pub fn get_min(a: i32, b: i32) -> i32 {
    if a < b {
        a
    } else {
        b
    }
}

pub fn get_max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use users::models::User;

    #[test]
    fn pending_friend_a() {
        let user_a = User {
            id: 5555,
            first_name: None,
            last_name: None,
            username: None,
            photo_url: None,
            created: chrono::NaiveDateTime::from_timestamp(1_000_000_000, 0),
            updated: chrono::NaiveDateTime::from_timestamp(1_000_000_000, 0),
        };
        let user_b = User {
            id: 7777,
            first_name: None,
            last_name: None,
            username: None,
            photo_url: None,
            created: chrono::NaiveDateTime::from_timestamp(1_000_000_000, 0),
            updated: chrono::NaiveDateTime::from_timestamp(1_000_000_000, 0),
        };
        let friendship = FriendshipPopulated {
            user_a,
            user_b,
            user_a_accepted: false,
            user_b_accepted: true,
        };
        // As User A, I have received a request and not responded yet
        let user_populated = gen_populated_user(5555, friendship);
        assert_eq!(user_populated.is_pending_friend, true);
        assert_eq!(user_populated.id, 7777);
    }

    #[test]
    fn requested_friend_a() {
        let user_a = User {
            id: 5555,
            first_name: None,
            last_name: None,
            username: None,
            photo_url: None,
            created: chrono::NaiveDateTime::from_timestamp(1_000_000_000, 0),
            updated: chrono::NaiveDateTime::from_timestamp(1_000_000_000, 0),
        };
        let user_b = User {
            id: 7777,
            first_name: None,
            last_name: None,
            username: None,
            photo_url: None,
            created: chrono::NaiveDateTime::from_timestamp(1_000_000_000, 0),
            updated: chrono::NaiveDateTime::from_timestamp(1_000_000_000, 0),
        };
        let friendship = FriendshipPopulated {
            user_a,
            user_b,
            user_a_accepted: true,
            user_b_accepted: false,
        };
        // As User A, I have made a request and they have not responded yet
        let user_populated = gen_populated_user(5555, friendship);
        assert_eq!(user_populated.have_requested_friend, true);
        assert_eq!(user_populated.id, 7777);
    }
}
