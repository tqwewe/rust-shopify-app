use rocket::request::{self, Request};
use rocket::Outcome;
use rocket::{http::Status, request::FromRequest};

pub struct ValidInstallNonce;

impl<'a, 'r> FromRequest<'a, 'r> for ValidInstallNonce {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let cookie = request.cookies().get_private("install_nonce");
        let install_nonce = match &cookie {
            Some(cookie) => cookie.value(),
            None => return Outcome::Failure((Status::BadRequest, ())),
        };

        let state = match request.get_query_value::<String>("state") {
            Some(Ok(state)) => state,
            _ => return Outcome::Failure((Status::BadRequest, ())),
        };

        let is_valid = state == install_nonce;

        if is_valid {
            Outcome::Success(ValidInstallNonce)
        } else {
            Outcome::Failure((Status::BadRequest, ()))
        }
    }
}
