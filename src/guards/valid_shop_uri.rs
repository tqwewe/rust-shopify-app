use once_cell::sync::Lazy;
use regex::Regex;
use rocket::http::RawStr;
use rocket::request::{self, Request};
use rocket::Outcome;
use rocket::{http::Status, request::FromRequest};

static SHOP_EXP: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[a-zA-Z0-9][a-zA-Z0-9\-]*\.myshopify\.com$").unwrap());

pub struct ValidShopUri;

impl<'a, 'r> FromRequest<'a, 'r> for ValidShopUri {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let is_valid = match request.get_query_value::<&RawStr>("shop") {
            Some(shop) => SHOP_EXP.is_match(shop?),
            None => false,
        };

        if is_valid {
            Outcome::Success(ValidShopUri)
        } else {
            Outcome::Failure((Status::BadRequest, ()))
        }
    }
}
