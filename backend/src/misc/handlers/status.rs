use actix_web::{HttpResponse, HttpRequest};

use util::errors::AppError;
use futures::future::Future;

use util::responses::ok_responder;

#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    data: Option<bool>,
}

// Handler for /x/status/
impl Status {
    pub fn status(_request: HttpRequest) -> impl Future<Item = HttpResponse, Error = AppError> {
        // TODO: Check DB connection

        ok_responder(Status { data: Some(true) })
    }
}
