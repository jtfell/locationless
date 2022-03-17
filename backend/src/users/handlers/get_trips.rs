use actix_session::Session;
use actix_web::{web, HttpResponse};
use futures::future::Future;

use trips::models::{user_trip_lookup, TripPopulated, UserTripLookup};
use users::middleware::{session_is_authenticated, session_user};
use util::errors::AppError;
use util::responses::err_responder;
use State;

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTrips {
    data: Vec<TripPopulated>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TripFilter {
    end_date_after: Option<chrono::NaiveDate>,
}

// Handler for /u/{id}/trips
impl GetTrips {
    pub fn get(
        session: Session,
        data: web::Data<State>,
        path: web::Path<i32>,
        query: web::Query<TripFilter>,
    ) -> Box<dyn Future<Item = HttpResponse, Error = AppError>> {
        if !session_is_authenticated(&session) {
            return err_responder(AppError::Unauthorised {
                msg: "You need to be logged in to access this endpoint".to_string(),
            });
        }

        let user_id = path.into_inner();
        let q = query.into_inner();
        let end_date_after = q
            .end_date_after
            .unwrap_or(chrono::NaiveDate::from_ymd(2000, 1, 1));

        Box::new(
            session_user(&session, data.clone()).and_then(move |active_user| {
                let t_lookup = UserTripLookup {
                    user: user_id.clone(),
                    end_date_after,
                };

                // Get the trips from the DB
                web::block(move || user_trip_lookup(t_lookup, data.clone())).then(move |res| {
                    match res {
                        Ok(l) => {
                            // TODO: What data needs to be filtered?

                            let trips = l
                                .into_iter()
                                .map(|mut t| {
                                    // Remove the matches array (if not user's trips)
                                    if active_user.id != user_id {
                                        t.matches = vec![];
                                    }
                                    t
                                })
                                .collect::<Vec<_>>();

                            Ok(HttpResponse::Ok().json(&Self { data: trips }))
                        }

                        Err(_e) => Err(AppError::NotFound {
                            msg: "Invalid User ID".to_string(),
                        }),
                    }
                })
            }),
        )
    }
}
