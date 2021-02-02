use crate::guards;
use rocket::{http::hyper::header::Location, response::Redirect};

#[derive(Responder)]
#[response(status = 303)]
pub struct RawRedirect((), Location);

#[derive(FromForm)]
pub struct InstallQuery {
    pub hmac: String,
    pub shop: String,
    pub timestamp: u32,
}

#[derive(FromForm)]
pub struct InstallCallbackQuery {
    pub code: String,
    pub hmac: String,
    pub shop: String,
    pub state: String,
    pub timestamp: u32,
}

#[get("/install")]
pub fn install(install: guards::Install) -> RawRedirect {
    RawRedirect((), Location(install.redirect_uri))
}

#[get("/install/callback")]
pub fn install_callback(_install_callback_guard: guards::InstallCallback) -> Redirect {
    Redirect::to("/")
}
