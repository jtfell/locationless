use actix_session::Session;
use actix_web::{web, HttpResponse};
use futures::future::Future;

use friendships::models::{lookup as friendship_lookup, UserFriendshipLookup};
use friendships::utils::gen_populated_user;
use users::middleware::{session_is_authenticated, session_user};
use users::models::{lookup, UserLookup, UserPopulated};
use util::errors::AppError;
use util::responses::err_responder;
use State;

#[derive(Serialize, Deserialize, Debug)]
pub struct Lookup {
    data: Option<UserPopulated>,
}

// Handler for /u/{id}
impl Lookup {
    pub fn get(
        session: Session,
        data: web::Data<State>,
        path: web::Path<i32>,
    ) -> Box<dyn Future<Item = HttpResponse, Error = AppError>> {
        if !session_is_authenticated(&session) {
            return err_responder(AppError::Unauthorised {
                msg: "You need to be logged in to access this endpoint".to_string(),
            });
        }

        Box::new(
            session_user(&session, data.clone()).and_then(move |active_user| {
                let f_lookup = UserFriendshipLookup {
                    active_user_id: active_user.id,
                    user_id: Some(path.into_inner()),
                };

                // Get the user from the DB
                web::block(move || friendship_lookup(f_lookup, data.clone())).then(move |res| {
                    match res {
                        Ok(mut friendships) => match friendships.pop() {
                            Some(f) => {
                                let user = gen_populated_user(active_user.id, f);
                                Ok(HttpResponse::Ok().json(&Self { data: Some(user) }))
                            }
                            None => Err(AppError::NotFound {
                                msg: "Invalid User ID".to_string(),
                            }),
                        },

                        Err(_e) => Err(AppError::NotFound {
                            // TODO: Check for DB NotFound err
                            msg: "Invalid User ID".to_string(),
                        }),
                    }
                })
            }),
        )
    }

    pub fn get_self(
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
                let user_lookup = UserLookup {
                    id: active_user.id,
                    active_user_id: active_user.id,
                };

                // Get the user from the DB
                web::block(move || lookup(user_lookup, data.clone())).then(move |res| match res {
                    Ok(user) => Ok(HttpResponse::Ok().json(&Self { data: Some(user) })),
                    Err(_e) => Err(AppError::Internal),
                })
            }),
        )
    }
}
