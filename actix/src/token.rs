use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use std::sync::Arc;

use crate::errors::{MyError, MyResult};

#[derive(Clone)]
pub struct JwtEncoder<'a> {
    encoding_key: EncodingKey,
    header: &'a Header,
}

impl<'a> JwtEncoder<'a> {
    pub fn new(secret: &str, header: &'a Header) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_ref()),
            header,
        }
    }

    // generate jwt token
    pub fn encode(&self, data: &Claims) -> MyResult<String> {
        encode(&self.header, &data, &self.encoding_key).map_err(|_e| MyError::InternalError)
    }
}

#[derive(Clone)]
pub struct JwtDecoder<'a, 'v> {
    decoding_key: DecodingKey<'a>,
    validation: &'v Validation,
}

impl<'a, 'v> JwtDecoder<'a, 'v> {
    pub fn new(secret: &'static str, validation: &'v Validation) -> Self {
        Self {
            decoding_key: DecodingKey::from_secret(secret.as_ref()),
            validation, //: Validation::default(),
        }
    }

    // decode jwt token string
    pub fn decode(&self, token: &'a str) -> MyResult<Claims> {
        decode::<Claims>(token, &self.decoding_key, &self.validation)
            .map_err(|_e| MyError::TokenError)
            .map(|t| t.claims)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: i64,
}

#[derive(Serialize)]
pub struct ResToekn {
    pub token: String,
    pub expires: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // let exp = chrono::Local::now().naive_utc().timestamp_millis() + 999;
        // let cl = Claims { sub: 1, exp: exp };
        // let secret = "abc";
        // let jwt_encoding_key = EncodingKey::from_secret(secret.as_ref());
        // let token = encode(&Header::default(), &cl, &jwt_encoding_key).unwrap();
        // println!("token: {}", token);

        // let decoode_key = DecodingKey::from_secret(secret.as_ref());
        // let validation = Validation::default();
        // let c = decode::<Claims>(&token, &decoode_key, &validation).unwrap();
        // println!("{:?}", c);

        // assert!(cl.sub == c.claims.sub);
    }
}