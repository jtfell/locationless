//  telegram.rs
//
//  Handles setting up a telegram API actor to work within actix-web.
// Disabled until the lib supports multiple threads

use actix_web::web;
use futures::Future;
use std::env;
use telegram_bot_fork::*;
use State;

use util::errors::AppError;

#[derive(Clone)]
pub struct Telegram {
    token: String,
}

impl Telegram {
    pub fn new() -> Self {
        // Initialise the Telegram Api instance
        let token = env::var("BOT_TOKEN").expect("BOT_TOKEN not set");

        Self { token }
    }
}

// TODO: Implement handler for sending a message to a user

pub struct SendTelegramMessage {
    pub user_id: i32,
    pub text: String,
}

// TODO: Implement error when telegram lib switched to failure crate
pub fn send_message(
    msg: SendTelegramMessage,
    data: web::Data<State>,
) -> Box<dyn Future<Item = (), Error = AppError>> {
    // FIXME: As the Api type is not thread-safe, we need to create a new one
    // each time we use this module. Gross.
    let api = Api::new(data.telegram.token.clone()).unwrap();

    println!(
        "[Telegram Actor] Sending: {:?} to {:?}",
        msg.text, msg.user_id
    );
    let chat = Chat::Private(User {
        id: UserId(msg.user_id.into()),
        first_name: "".to_string(),
        last_name: None,
        username: None,
        language_code: None,
        is_bot: false,
    });
    let m = SendMessage::new(chat, msg.text);
    Box::new(api.send(m).then(|res| match res {
        Ok(_msg) => Ok(()),
        Err(_e) => Err(AppError::Internal),
    }))
}
