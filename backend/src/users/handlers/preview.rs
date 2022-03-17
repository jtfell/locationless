use actix_web::{web, HttpResponse};
use futures::future::Future;

use users::models::{username_lookup, UserPreview, UsernameLookup};
use util::errors::AppError;
use State;

#[derive(Serialize, Deserialize, Debug)]
pub struct PreviewLookup {
    data: Option<UserPreview>,
}

// Handler for /u/{username}/preview
//
// NOTE:
//   This is a public route and has no auth requirements.
//   Be careful what you expose here!
impl PreviewLookup {
    pub fn get(
        data: web::Data<State>,
        path: web::Path<String>,
    ) -> Box<dyn Future<Item = HttpResponse, Error = AppError>> {
        let u_lookup = UsernameLookup {
            username: path.into_inner(),
        };

        // Get the user from the DB
        Box::new(
            web::block(move || username_lookup(u_lookup, data.clone())).then(
                move |res| match res {
                    Ok(user) => Ok(HttpResponse::Ok().json(&Self { data: Some(user) })),
                    Err(e) => Err(AppError::Internal),
                },
            ),
        )
    }
}
