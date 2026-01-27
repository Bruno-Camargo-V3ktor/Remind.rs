use chrono::{DateTime, Duration, Utc};
use domain::models::UserId;
use hmac::{Hmac, Mac};
use jwt::{Header, SignWithKey, Token, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha384;
use std::{collections::BTreeMap, str::FromStr};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserToken(String);

impl UserToken {
    pub fn new(key: &str, duration: i64, user_id: UserId) -> Self {
        let id = user_id.0.to_string();
        let now = Utc::now();
        let exp = now + Duration::hours(duration);

        let key: Hmac<Sha384> = Hmac::new_from_slice(key.as_bytes()).unwrap();
        let header = Header {
            algorithm: jwt::AlgorithmType::Hs384,
            ..Default::default()
        };
        let mut claims = BTreeMap::new();

        claims.insert("sub", id);
        claims.insert("iat", now.timestamp().to_string());
        claims.insert("exp", exp.timestamp().to_string());

        let token = Token::new(header, claims).sign_with_key(&key).unwrap();
        Self(token.as_str().to_owned())
    }

    pub fn validate(&self, key: &str) -> Option<UserId> {
        let key: Hmac<Sha384> = Hmac::new_from_slice(key.as_bytes()).unwrap();
        let token: Token<Header, BTreeMap<String, String>, _> =
            self.0.verify_with_key(&key).unwrap();

        let _header = token.header();
        let claims = token.claims();

        let exp_str = claims["exp"].clone();
        let id_str = claims["sub"].clone();

        let now = Utc::now();
        let exp = DateTime::from_timestamp_nanos(exp_str.parse::<i64>().unwrap());

        if now >= exp {
            None
        } else {
            UserId::from_str(&id_str).ok()
        }
    }
}
