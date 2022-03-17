use actix_session::Session;
use actix_web::{web, HttpResponse};
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use futures::future::Future;
use validator::Validate;

use trips::models::{new, NewTrip, Trip};
use users::middleware::{session_is_authenticated, session_user};
use util::errors::AppError;
use util::responses::err_responder;
use State;

// TODO: Populate overlaps???
#[derive(Serialize, Deserialize, Debug)]
pub struct Create {
    data: Trip,
}

#[derive(Validate, Deserialize, Serialize, Debug)]
pub struct NewTripBody {
    pub city: i32,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
}

// Handler for POST /trips/
impl Create {
    pub fn create(
        session: Session,
        trip: web::Json<NewTripBody>,
        data: web::Data<State>,
    ) -> Box<dyn Future<Item = HttpResponse, Error = AppError>> {
        if !session_is_authenticated(&session) {
            return err_responder(AppError::Unauthorised {
                msg: "You need to be logged in to access this endpoint".to_string(),
            });
        }

        let trip_body = trip.into_inner();
        if let Err(e) = trip_body.validate() {
            // TODO: Error message?
            return err_responder(AppError::BadClientData { msg: e.to_string() });
        }

        Box::new(
            session_user(&session, data.clone()).and_then(move |user| {
                // User ID from session, rest of the data from the request body
                let new_trip = NewTrip {
                    user: user.id,
                    city: trip_body.city,
                    start_date: trip_body.start_date,
                    end_date: trip_body.end_date,
                };

                web::block(move || new(new_trip, data.clone())).then(move |res| match res {
                    Ok(created_trip) => Ok(HttpResponse::Ok().json(&Self { data: created_trip })),
                    Err(e) => match e {
                        // TODO: Re-add this error handling
                        // DieselError::DatabaseError(kind, info) => {
                        //     if let DatabaseErrorKind::ForeignKeyViolation = kind {
                        //         return Err(AppError::BadClientData {
                        //             msg: format!("No city with ID {}", trip_body.city),
                        //         });
                        //     }
                        //     if let DatabaseErrorKind::UniqueViolation = kind {
                        //         let message =
                        //             info.details().unwrap_or_else(|| info.message()).to_string();
                        //         return Err(AppError::BadClientData { msg: message });
                        //     }
                        //     println!("{:?}", kind);
                        //     println!("{:?}", info);
                        //     Err(AppError::Internal)
                        // }
                        _ => Err(AppError::Internal),
                    },
                })
            }),
        )
    }
}
