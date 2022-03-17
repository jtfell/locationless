use actix_session::Session;
use actix_web::{web, HttpResponse};
use futures::future::Future;

use trips::models::{lookup, TripLookup, TripPopulated};
use users::middleware::{session_is_authenticated, session_user};
use util::errors::AppError;
use util::responses::err_responder;
use State;

#[derive(Serialize, Deserialize, Debug)]
pub struct Lookup {
    data: Option<TripPopulated>,
}

// Handler for /trips/{id}
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

        Box::new(session_user(&session, data.clone()).and_then(move |user| {
            let trip_lookup = TripLookup {
                id: path.into_inner(),
                active_user_id: user.id,
            };

            // Get the trip from the DB
            web::block(move || lookup(trip_lookup, data.clone()))
                .then(move |res| match res {
                    Ok(trip) => Ok(HttpResponse::Ok().json(&Self { data: Some(trip) })),

                    Err(_e) => Err(AppError::NotFound {
                        // TODO: Check for DB NotFound err
                        msg: "Invalid Trip ID".to_string(),
                    }),
                })
        }))
    }
}
