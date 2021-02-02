use crate::ShopifyApp;
use hex;
use hmac::{Hmac, Mac, NewMac};
use rocket::request::{self, Request};
use rocket::Outcome;
use rocket::State;
use rocket::{http::Status, request::FromRequest};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub struct ValidHmac;

impl<'a, 'r> FromRequest<'a, 'r> for ValidHmac {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let shopify_app = request.guard::<State<ShopifyApp>>()?;

        let query_items_iter = match request.raw_query_items() {
            Some(query_items_iter) => query_items_iter,
            None => return Outcome::Failure((Status::BadRequest, ())),
        };

        let mut hmac = String::new();

        let mut message = query_items_iter.fold(
            Vec::<(String, String)>::with_capacity(2),
            |mut acc, query_item| {
                let (key, value) = query_item.key_value_decoded();

                if key == "hmac" {
                    hmac = value;
                    return acc;
                }

                acc.push((key, value));
                acc
            },
        );

        message.sort_by(|a, b| a.0.cmp(&b.0));

        let message_str = &message.iter().fold(String::new(), |acc, key_val| {
            acc + "&" + &key_val.0 + "=" + &key_val.1
        })[1..];

        let encoded = {
            let mut mac = HmacSha256::new_varkey(shopify_app.credentials.secret.as_bytes())
                .expect("Invalid store secret");
            mac.update(message_str.as_bytes());
            let result = mac.finalize();
            let code_bytes = result.into_bytes();

            hex::encode(code_bytes)
        };

        let matches = encoded.eq(&hmac);

        println!("our secret: {}", shopify_app.credentials.secret);
        println!("our hmac: {}", encoded);
        println!("their hmac: {}", hmac);
        println!("matches: {}", matches);

        if matches {
            Outcome::Success(ValidHmac)
        } else {
            Outcome::Failure((Status::Unauthorized, ()))
        }
    }
}
