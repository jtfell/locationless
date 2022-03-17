//  Endpoint for logging a user out. Nothing particularly special,
//  just obliterates their session entry + cookie.

use actix_session::Session;
use actix_web::{HttpResponse};

pub fn logout(session: Session) -> HttpResponse {
    session.clear();
    HttpResponse::Ok().finish()
}
