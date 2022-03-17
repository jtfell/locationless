use actix_session::Session;
use actix_web::{web, HttpResponse};
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use futures::future::result;
use futures::future::Future;
use validator::Validate;

use friendships::models::{
    lookup, new, update, FriendshipUpdate, NewFriendship, UserFriendshipLookup,
};
use friendships::utils::{gen_populated_user, get_max, get_min};
use users::middleware::{session_is_authenticated, session_user};
use users::models::UserPopulated;
use util::errors::AppError;
use util::responses::err_responder;
use util::telegram::{send_message, SendTelegramMessage};
use State;

#[derive(Serialize, Deserialize, Debug)]
pub struct Friendships {
    data: Option<Vec<UserPopulated>>,
}

// Handler for /friendships/ (implicit self user filter)
impl Friendships {
    pub fn get_all(
        session: Session,
        data: web::Data<State>,
    ) -> Box<dyn Future<Item = HttpResponse, Error = AppError>> {
        if !session_is_authenticated(&session) {
            return err_responder(AppError::Unauthorised {
                msg: "You need to be logged in to access this endpoint".to_string(),
            });
        }

        Box::new(
            session_user(&session, data.clone()).and_then(move |active_user| {
                let user_friendship_lookup = UserFriendshipLookup {
                    active_user_id: active_user.id,
                    user_id: None,
                };

                // Get the friendships from the DB
                web::block(move || lookup(user_friendship_lookup, data.clone())).then(move |res| {
                    match res {
                        Ok(friendships) => {
                            let users = friendships
                                .into_iter()
                                .map(|f| gen_populated_user(active_user.id, f))
                                .collect::<Vec<_>>();
                            Ok(HttpResponse::Ok().json(&Self { data: Some(users) }))
                        }
                        Err(e) => {
                            error!("{}", e);
                            Err(AppError::Internal)
                        }
                    }
                })
            }),
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FriendRequest {
    data: Option<UserPopulated>,
}

#[derive(Validate, Deserialize, Serialize, Debug)]
pub struct FriendshipRequest {}

impl FriendRequest {
    pub fn request(
        session: Session,
        data: web::Data<State>,
        path: web::Path<i32>,
    ) -> Box<dyn Future<Item = HttpResponse, Error = AppError>> {
        if !session_is_authenticated(&session) {
            return err_responder(AppError::Unauthorised {
                msg: "You need to be logged in to access this endpoint".to_string(),
            });
        }

        let user_id = path.into_inner();
        Box::new(
            session_user(&session, data.clone()).and_then(move |active_user| {
                // TODO: Fix the types here
                // if self_user.id == friendship_req.user {
                //     return Err(AppError::BadClientData { msg: "You can't add yourself as a friend".to_string() });
                // }

                // User ID from session, requested friend from path
                let new_friendship = NewFriendship {
                    user_a: get_min(active_user.id, user_id),
                    user_b: get_max(active_user.id, user_id),
                    user_a_accepted: active_user.id == get_min(active_user.id, user_id),
                    user_b_accepted: active_user.id == get_max(active_user.id, user_id),
                };

                web::block(move || new(new_friendship, data.clone())).then(move |res| match res {
                    Ok(_new_user) => {
                        // Send a notification to the other user on Telegram
                        // let tg_message = SendTelegramMessage {
                        //     user_id: user_id,
                        //     text: generate_friend_req_msg(active_user),
                        // };

                        // FIXME: can't figure out how to fix this pile of types
                        // send_message(tg_message, data.clone()).then(move |_r| {
                        // })
                        Ok(HttpResponse::Ok().json(&Self {
                            // TODO: Populate a new user object
                            data: None,
                        }))
                    }
                    Err(_e) => Err(AppError::Internal),
                })
                // Err(e) => match e {
                //     DieselError::DatabaseError(kind, info) => {
                //         if let DatabaseErrorKind::ForeignKeyViolation = kind {
                //             return err_responder(AppError::BadClientData {
                //                 msg: format!("No user with ID {}", user_id),
                //             });
                //         }
                //         if let DatabaseErrorKind::UniqueViolation = kind {
                //             return err_responder(AppError::BadClientData {
                //                 msg: format!(
                //                     "You are already friends with User {}",
                //                     user_id
                //                 ),
                //             });
                //         }
                //         error!("DB Error {:?}: {:?}", kind, info);
                //         err_responder(AppError::Internal)
                //     }
                //     _ => err::<_, AppError>(AppError::Internal),
                // },
            }),
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FriendResponse {
    data: Option<UserPopulated>,
}

#[derive(Validate, Deserialize, Serialize, Debug)]
pub struct FriendshipResponse {
    accept: bool,
}

impl FriendResponse {
    pub fn response(
        session: Session,
        data: web::Data<State>,
        path: web::Path<i32>,
        body: web::Json<FriendResponse>,
    ) -> Box<dyn Future<Item = HttpResponse, Error = AppError>> {
        if !session_is_authenticated(&session) {
            return err_responder(AppError::Unauthorised {
                msg: "You need to be logged in to access this endpoint".to_string(),
            });
        }

        let user_id = path.into_inner();
        let _action = body.into_inner();

        Box::new(
            session_user(&session, data.clone()).and_then(move |active_user| {
                // TODO: Fix the types here
                // if self_user.id == friendship_req.user {
                //     return Err(AppError::BadClientData { msg: "You can't add yourself as a friend".to_string() });
                // }

                // TODO: Implement deny

                // User ID from session, requested friend from path
                let friendship_update = FriendshipUpdate {
                    user_a: get_min(active_user.id, user_id),
                    user_b: get_max(active_user.id, user_id),
                    user_a_accepted: if active_user.id < user_id {
                        Some(true)
                    } else {
                        None
                    },
                    user_b_accepted: if active_user.id > user_id {
                        Some(true)
                    } else {
                        None
                    },
                };

                web::block(move || update(friendship_update, data.clone())).then(move |res| {
                    match res {
                        Ok(_updated_friendship) => Ok(HttpResponse::Ok().json(&Self {
                            // TODO: Populate a new user object
                            data: None,
                        })),
                        Err(e) => match e {
                            // TODO: Sort out error handling
                            // DieselError::DatabaseError(kind, info) => {
                            //     if let DatabaseErrorKind::ForeignKeyViolation = kind {
                            //         return Err(AppError::BadClientData {
                            //             msg: format!("No user with ID {}", user_id),
                            //         });
                            //     }
                            //     // if let DatabaseErrorKind::UniqueViolation = kind {
                            //     //     return Err(AppError::BadClientData {
                            //     //         msg: format!(
                            //     //             "You are already friends with User {}",
                            //     //             user_id
                            //     //         ),
                            //     //     });
                            //     // }
                            //     error!("DB Error {:?}: {:?}", kind, info);
                            //     Err(AppError::Internal)
                            // }
                            _ => Err(AppError::Internal),
                        },
                    }
                })
            }),
        )
    }
}

//
// Identify the user with as much info as their profile contains.
//
fn generate_friend_req_msg(user: UserPopulated) -> String {
    let l_name = user.last_name.unwrap_or("".to_string());
    let f_name = user.first_name.unwrap_or("".to_string());
    let u_name = match user.username {
        Some(username) => format!("(@{:?})", username),
        None => "".to_string(),
    };
    format!(
        "You have a new friend request from {} {} {}",
        f_name, l_name, u_name
    )
}
