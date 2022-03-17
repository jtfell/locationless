use actix_session::Session;
use actix_web::{web, HttpResponse};
use futures::future::Future;

use explore::models::{lookup, ContinentCode, ExploreLookup, Suggestion};
use users::middleware::{session_is_authenticated, session_user};
use util::errors::AppError;
use util::responses::{err_responder};
use State;

#[derive(Serialize, Deserialize, Debug)]
pub struct Lookup {
    data: Vec<Suggestion>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchFilter {
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
    pub continent: Option<ContinentCode>,
}

// Handler for /explore/?start_date=__&end_date=__&continent=__
impl Lookup {
    pub fn get(
        session: Session,
        data: web::Data<State>,
        query: web::Query<SearchFilter>,
    ) -> Box<dyn Future<Item = HttpResponse, Error = AppError>> {
        if !session_is_authenticated(&session) {
            return err_responder(AppError::Unauthorised {
                msg: "You need to be logged in to access this endpoint".to_string(),
            });
        }

        let q = query.into_inner();

        Box::new(session_user(&session, data.clone()).and_then(move |user| {
            let explore_lookup = ExploreLookup {
                active_user_id: user.id,
                continent: q.continent,
                start_date: q.start_date,
                end_date: q.end_date,
            };

            // Get the trip from the DB
            web::block(move || lookup(explore_lookup, data.clone())).then(move |res| match res {
                Ok(suggestions) => Ok(HttpResponse::Ok().json(&Self { data: suggestions })),

                Err(_e) => Err(AppError::Internal),
            })
        }))
    }
}
