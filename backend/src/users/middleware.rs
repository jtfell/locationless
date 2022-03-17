//  src/users/middleware.rs
//
//  Middleware that handles loading current User for a given
//  request, along with any Session data they may have. This
//  specifically enables Anonymous users with Session data, ala
//  Django's approach.

use actix_session::Session;
use actix_web::web;
use futures::{Future, future};
use State;

use util::errors::AppError;
use users::models::{lookup, UserLookup, UserPopulated};

pub type UserAuthenticationResult = Box<dyn Future<Item = UserPopulated, Error = AppError>>;

#[inline(always)]
pub fn session_is_authenticated(session: &Session) -> bool {
    match &session.get::<i32>("uid") {
        Ok(session_res) => match session_res {
            Some(_session_id) => true,
            None => false,
        },

        Err(e) => {
            error!("Error'd when attempting to fetch session data: {:?}", e);
            false
        }
    }
}

pub fn session_user(session: &Session, data: web::Data<State>) -> UserAuthenticationResult {
    match &session.get::<i32>("uid") {
        Ok(session_res) => match session_res.clone() {
            Some(session_id) => Box::new(
                // TODO
                web::block(move || {
                    lookup(
                        UserLookup {
                            id: session_id,
                            active_user_id: session_id,
                        },
                        data,
                    )
                })
                .then(|res| match res {
                    Ok(user) => Ok(user),
                    Err(_err) => Err(AppError::NotFound {
                        msg: "Unable to find user".to_string(),
                    }),
                }),
            ),

            None => Box::new(future::err(AppError::NotFound {
                msg: "User has no session data".to_string(),
            })),
        },

        Err(e) => {
            error!("Error'd when attempting to fetch session data: {:?}", e);
            Box::new(future::err(AppError::Internal))
        }
    }
}
