use actix_session::Session;
use actix_web::{web, HttpResponse};
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use futures::future::Future;

use trips::models::{delete, DeleteTrip};
use users::middleware::{session_is_authenticated, session_user};
use util::errors::AppError;
use util::responses::err_responder;
use State;

#[derive(Serialize, Deserialize, Debug)]
pub struct Delete {}

// Handler for DELETE /trips/:id
impl Delete {
    pub fn delete(
        session: Session,
        data: web::Data<State>,
        path: web::Path<i32>,
    ) -> Box<dyn Future<Item = HttpResponse, Error = AppError>> {
        if !session_is_authenticated(&session) {
            return err_responder(AppError::Unauthorised {
                msg: "You need to be logged in to access this endpoint".to_string(),
            });
        }

        let trip_id = path.into_inner();

        Box::new(session_user(&session, data.clone()).and_then(move |user| {
            let del_trip = DeleteTrip {
                id: trip_id,
                user: user.id,
            };

            web::block(move || delete(del_trip, data.clone()))
                .then(move |res| match res {
                    Ok(_) => Ok(HttpResponse::Ok().json(&Self {})),
                    Err(e) => match e {
                        // DieselError::DatabaseError(kind, info) => {
                        //     if let DatabaseErrorKind::ForeignKeyViolation = kind {
                        //         println!("{:?}", kind);
                        //         println!("{:?}", info);
                        //         return Err(AppError::BadClientData {
                        //             msg: format!("No trip with ID {}", trip_id),
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
