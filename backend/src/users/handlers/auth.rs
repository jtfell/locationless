use actix_session::Session;
use actix_web::{web, HttpResponse};
use futures::future::Future;
use telegram_login::{check_signature, TelegramLogin};

use users::models::{new_or_existing, NewOrExistingUser};
use util::errors::AppError;
use util::responses::err_responder;
use State;

#[derive(Serialize, Deserialize, Debug)]
pub struct Auth {}

impl Auth {
    pub fn get(
        session: Session,
        query: web::Query<TelegramLogin>,
        data: web::Data<State>,
    ) -> Box<dyn Future<Item = HttpResponse, Error = AppError>> {
        let bot_token = std::env::var("BOT_TOKEN").expect("BOT_TOKEN not set!");

        let user = query.into_inner();
        if let Err(_e) = check_signature(bot_token, user.clone()) {
            return err_responder(AppError::TelegramAuth);
        };

        let user_details = NewOrExistingUser {
            id: user.id,
            first_name: user.first_name,
            last_name: user.last_name,
            username: user.username,
            photo_url: user.photo_url,
        };

        Box::new(web::block(move ||
            new_or_existing(user_details, data))
                .then(move |res| match res {
                    // Regardless of if they are new or existing, log them in
                    Ok(u) => {
                        if let Err(e) = session.set("uid", u.id) {
                            error!("Could not set UID for user session! {:?}", e);
                            return Err(AppError::Internal);
                        };

                        Ok(HttpResponse::Ok().json(&Self {}))
                    }
                    Err(e) => {
                        error!("Could not lookup user! {:?}", e);
                        Err(AppError::Internal)
                    }
                }),
        )
    }
}
