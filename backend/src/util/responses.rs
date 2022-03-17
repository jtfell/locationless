use actix_web::HttpResponse;
use futures::future::{err, ok, Future};
use serde::ser::Serialize;

use util::errors::AppError;

pub fn err_responder(e: AppError) -> Box<dyn Future<Item = HttpResponse, Error = AppError>> {
    Box::new(err::<_, AppError>(e))
}

pub fn ok_responder<T>(data: T) -> Box<dyn Future<Item = HttpResponse, Error = AppError>>
where
    T: Serialize,
{
    Box::new(ok::<_, AppError>(HttpResponse::Ok().json(data)))
}
