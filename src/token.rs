use actix_web::{guard::Head, web::Json};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::errors::{MyError, MyResult};

#[derive(Clone)]
pub struct TokenTool<'token, 'decode> {
    jwt_secret: &'token str,
    encoding_key: EncodingKey,
    decoding_key: DecodingKey<'decode>,
    header: Header,
    validation: Validation,
}

impl TokenTool<'_, '_> {
    pub fn new(secret: &'static str) -> Self {
        Self {
            jwt_secret: secret, //secret.to_string(),
            encoding_key: EncodingKey::from_secret(secret.as_ref()),
            decoding_key: DecodingKey::from_secret(secret.as_ref()),
            header: Header::default(),
            validation: Validation::default(),
        }
    }

    pub fn encode(&self, data: &Claims) -> MyResult<String> {
        encode(&self.header, &data, &self.encoding_key).map_err(|_e| MyError::InternalError)
    }

    pub fn decode<'a>(&self, token: &'a str) -> MyResult<Claims> {
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
    use actix_web::guard::{Head, Header};

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
