use rocket::request::{self, Request};
use rocket::Outcome;
use rocket::{http::Cookie, request::FromRequest};

use super::valid_hmac;
use super::valid_install_nonce;
use super::valid_shop_uri;

pub struct InstallCallback;

impl<'a, 'r> FromRequest<'a, 'r> for InstallCallback {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        println!("One");

        match valid_hmac::ValidHmac::from_request(request) {
            Outcome::Failure((status, _)) => return Outcome::Failure((status, ())),
            _ => (),
        };

        println!("Two");

        match valid_shop_uri::ValidShopUri::from_request(request) {
            Outcome::Failure((status, _)) => return Outcome::Failure((status, ())),
            _ => (),
        };

        println!("Three");

        match valid_install_nonce::ValidInstallNonce::from_request(request) {
            Outcome::Failure((status, _)) => return Outcome::Failure((status, ())),
            _ => (),
        };

        println!("Four");

        request
            .cookies()
            .remove_private(Cookie::named("install_nonce"));

        Outcome::Success(Self)
    }
}
