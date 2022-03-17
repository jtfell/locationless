use actix_session::Session;
use actix_web::{web, HttpResponse};
use futures::future::Future;

use users::middleware::{session_is_authenticated, session_user};
use users::models::{search, User, UserSearch};
use util::errors::AppError;
use util::responses::err_responder;
use State;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Search {
    data: Vec<User>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SearchReq {
    q: String,
}

impl Search {
    pub fn search(
        session: Session,
        data: web::Data<State>,
        query: web::Query<SearchReq>,
    ) -> Box<dyn Future<Item = HttpResponse, Error = AppError>> {
        if !session_is_authenticated(&session) {
            return err_responder(AppError::Unauthorised {
                msg: "You need to be logged in to access this endpoint".to_string(),
            });
        }

        // Check the structure of the request
        let search_q = query.into_inner();

        // TODO: This endpoint is returning 404s rather than empty results...
        Box::new(
            session_user(&session, data.clone()).and_then(move |active_user| {
                web::block(move || {
                    search(
                        UserSearch {
                            q: search_q.q,
                            active_user_id: active_user.id,
                        },
                        data.clone(),
                    )
                })
                .then(move |res| match res {
                    Ok(results) => Ok(HttpResponse::Ok().json(&Self { data: results })),
                    Err(e) => {
                        error!("User search failure: {:?}", e);
                        Err(AppError::Internal)
                    }
                })
            }),
        )
    }
}
