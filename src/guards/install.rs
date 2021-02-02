use crate::ShopifyApp;
use rocket::Outcome;
use rocket::State;
use rocket::{
    http::SameSite,
    request::{self, Request},
};
use rocket::{
    http::{Cookie, Status},
    request::FromRequest,
};

use super::valid_hmac;

pub struct Install {
    pub redirect_uri: String,
    pub nonce: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for Install {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        match valid_hmac::ValidHmac::from_request(request) {
            Outcome::Failure((status, _)) => return Outcome::Failure((status, ())),
            _ => (),
        };

        let shopify_app = request.guard::<State<ShopifyApp>>()?;
        let shop = match request.get_query_value::<String>("shop") {
            Some(Ok(state)) => state,
            _ => return Outcome::Failure((Status::BadRequest, ())),
        };

        let (redirect_uri, nonce) = shopify_app.install_uri(
            &shop,
            &format!("{host}/install/callback", host = shopify_app.host),
        );

        request.cookies().add_private(
            Cookie::build("install_nonce", nonce.clone())
                .same_site(SameSite::None)
                .finish(),
        );

        Outcome::Success(Self {
            redirect_uri,
            nonce,
        })
    }
}
