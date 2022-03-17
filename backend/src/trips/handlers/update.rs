use actix_session::Session;
use actix_web::{web, HttpResponse};
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use futures::future::Future;
use validator::Validate;

use trips::models::{update, Trip, UpdateTrip};
use users::middleware::{session_is_authenticated, session_user};
use util::errors::AppError;
use util::responses::err_responder;
use State;

#[derive(Serialize, Deserialize, Debug)]
pub struct Update {
    data: Trip,
}

#[derive(Validate, Deserialize, Serialize, Debug)]
pub struct UpdateTripBody {
    pub id: i32,
    pub city: i32,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
}

// Handler for PUT /trips/:id
impl Update {
    pub fn update(
        session: Session,
        data: web::Data<State>,
        trip: web::Json<UpdateTripBody>,
        path: web::Path<i32>,
    ) -> Box<dyn Future<Item = HttpResponse, Error = AppError>> {
        if !session_is_authenticated(&session) {
            return err_responder(AppError::Unauthorised {
                msg: "You need to be logged in to access this endpoint".to_string(),
            });
        }

        let trip_id = path.into_inner();
        let trip_body = trip.into_inner();
        if let Err(e) = trip_body.validate() {
            return err_responder(AppError::BadClientData { msg: e.to_string() });
        }

        Box::new(session_user(&session, data.clone()).and_then(move |user| {
            // User ID from session, rest of the data from the request body
            let update_trip = UpdateTrip {
                id: trip_id,
                user: user.id,
                city: trip_body.city,
                start_date: trip_body.start_date,
                end_date: trip_body.end_date,
            };

            web::block(move || update(update_trip, data.clone()))
                .then(move |res| match res {
                    Ok(created_trip) => Ok(HttpResponse::Ok().json(&Self { data: created_trip })),
                    Err(e) => match e {
                        // DieselError::DatabaseError(kind, info) => {
                        //     if let DatabaseErrorKind::ForeignKeyViolation = kind {
                        //         println!("{:?}", kind);
                        //         println!("{:?}", info);
                        //         return Err(AppError::BadClientData {
                        //             msg: format!("No city/trip with ID {}", trip_body.city),
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
        }))
    }
}
