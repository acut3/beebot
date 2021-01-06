use crate::grants::*;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};

use jsonwebtoken::{
    encode,
    decode,
    Header,
    Algorithm,
    EncodingKey,
    DecodingKey,
    Validation,
    errors::{Error, ErrorKind},
};

#[derive(Serialize)]
struct ClaimsIn<'a> {
    iat: u64,
    exp: u64,
    uid: u64,
    grants: &'a UserGrants,
}

#[derive(Deserialize)]
#[allow(dead_code)]    // Because iat and exp are deserialized but never used
struct ClaimsOut {
    iat: u64,
    exp: u64,
    uid: u64,
    grants: UserGrants,
}


pub fn jwt_encode(uid: u64, grants: &UserGrants, key: &[u8], lifetime: u64) -> Result<String, String> {
    let iat = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(val) => val.as_secs(),
        Err(e) => return Err(format!("Erreur interne: {}", e)),
    };
    let exp = iat + (60 * lifetime);
    let claims = ClaimsIn {
        iat,
        exp,
        uid, 
        grants,
    };
    match encode(&Header::new(Algorithm::HS512), &claims, &EncodingKey::from_secret(key)) {
        Ok(val) => Ok(val),
        Err(e) => Err(format!("Erreur interne: {}", e)),
    }
}


pub fn jwt_decode(uid: u64, jwt: &str, key: &[u8]) -> std::result::Result<UserGrants, String> {
    let claims = match decode::<ClaimsOut>(jwt, &DecodingKey::from_secret(key), &Validation::new(Algorithm::HS512)) {
        Ok(val) => val.claims,
        Err(e) => return Err(errmsg(e)),
    };

    if claims.uid != uid {
        return Err(format!("Ce token est destiné à un autre utilisateur (<@{}>)", claims.uid));
    }
    Ok(claims.grants)
}


fn errmsg(e: Error) -> String {
    match e.kind() {
        ErrorKind::ExpiredSignature => format!("Ce token a expiré. Tu peux en demander un nouveau avec la commande `!iam`"),
        _ => format!("Ce token n'est pas valide: {}", e),
    }
}
