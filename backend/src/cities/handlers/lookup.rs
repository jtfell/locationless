use actix_session::Session;
use actix_web::{web, HttpResponse};
use futures::future::Future;

use cities::models::{lookup, lookup_all, City, CityLookup, CityLookupAll};
use users::middleware::session_is_authenticated;
use util::errors::AppError;
use util::responses::err_responder;
use State;

#[derive(Serialize, Deserialize, Debug)]
pub struct LookupAll {
    data: Option<Vec<City>>,
}

// Handler for /cities/
impl LookupAll {
    pub fn get_all(
        session: Session,
        data: web::Data<State>,
    ) -> Box<dyn Future<Item = HttpResponse, Error = AppError>> {
        if !session_is_authenticated(&session) {
            return err_responder(AppError::Unauthorised {
                msg: "You need to be logged in to access this endpoint".to_string(),
            });
        }

        let cities_lookup = CityLookupAll {};

        // Get the cities from the DB
        Box::new(
            web::block(move || lookup_all(cities_lookup, data.clone())).then(
                move |res| match res {
                    Ok(cities) => Ok(HttpResponse::Ok().json(&Self { data: Some(cities) })),
                    Err(e) => {
                        error!("{}", e);
                        Err(AppError::Internal)
                    }
                },
            ),
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Lookup {
    data: Option<City>,
}

// Handler for /cities/{id}
impl Lookup {
    pub fn get(
        session: Session,
        path: web::Path<i32>,
        data: web::Data<State>,
    ) -> Box<dyn Future<Item = HttpResponse, Error = AppError>> {
        if !session_is_authenticated(&session) {
            return err_responder(AppError::Unauthorised {
                msg: "You need to be logged in to access this endpoint".to_string(),
            });
        }

        let city_lookup = CityLookup {
            id: path.into_inner(),
        };

        // Get the trip from the DB
        Box::new(
            web::block(move || lookup(city_lookup, data.clone())).then(move |res| match res {
                Ok(trip) => Ok(HttpResponse::Ok().json(&Self { data: Some(trip) })),

                Err(_e) => Err(AppError::NotFound {
                    msg: "Invalid City ID".to_string(),
                }),
            }),
        )
    }
}
