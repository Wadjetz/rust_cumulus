use jsonwebtoken::{encode, decode, Header, Validation};
use uuid::Uuid;
use rocket::request::{self, Request, FromRequest};
use rocket::outcome::Outcome;
use rocket::http::Status;

use errors::*;

const SECRET: &'static str = "secret"; // TODO move to conf

#[derive(Debug)]
pub struct AuthData {
    pub uuid: Uuid,
    pub email: String,
}

impl AuthData {
    pub fn new(uuid: Uuid, email: String) -> Self {
        AuthData {
            uuid: uuid,
            email: email
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claime {
    pub uuid: String,
    pub email: String,
}

#[allow(dead_code)]
impl Claime {
    pub fn new(uuid: String, email: String) -> Self {
        Claime {
            uuid: uuid,
            email: email
        }
    }
    pub fn to_auth(self) -> Result<AuthData> {
        let auth_data = Uuid::parse_str(&self.uuid).map(move |uuid| AuthData::new(uuid, self.email))?;
        Ok(auth_data)
    }
}

#[allow(dead_code)]
pub fn create_token(uuid: Uuid, email: String) -> Result<String> {
    let claims = Claime::new(uuid.hyphenated().to_string(), email.clone());
    let token = encode(&Header::default(), &claims, SECRET.as_ref())?;
    Ok(token)
}

pub fn decode_auth(token: &str) -> Result<AuthData> {
    let claims = decode::<Claime>(token, SECRET.as_ref(), &Validation::default())?;
    let claims = claims.claims;
    let auth = claims.to_auth()?;
    Ok(auth)
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthData {
    type Error = String;
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, String> {
        match request.headers().get("Authorization").next() {
            Some(token) => {
                match decode_auth(token) {
                    Ok(auth) => {
                        Outcome::Success(auth)
                    },
                    Err(error) => {
                        println!("decode = {:?}", error);
                        Outcome::Failure((Status::Unauthorized, "Wrong token".to_string()))
                    }
                }
            },
            None => {
                println!("No Authorization token");
                Outcome::Failure((Status::Unauthorized, "No Authorization token".to_string()))
            }
        }
    }
}
