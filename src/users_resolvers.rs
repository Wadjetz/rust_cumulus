use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use diesel::PgConnection;
use validator::Validate;

use errors::*;
use users_repository;
use user::{User, verify_password};
use token;
use config::Config;

pub fn signup_resolver(pool: &Pool<ConnectionManager<PgConnection>>, config: &Config, user: User) -> Result<String> {
    let connection = pool.get()?;
    let _ = user.validate()?;
    let _ = users_repository::insert(&connection, &user)?;
    let token = token::create_token(user.uuid, user.email, config.secret_key.as_ref())?;
    Ok(token)
}

pub fn login_resolver(pool: &Pool<ConnectionManager<PgConnection>>, config: &Config, email: String, password: String) -> Result<String> {
    let connection = pool.get()?;
    let user = users_repository::find_by_email(&connection, &email)?;
    if let Ok(true) = verify_password(&password, &user.password) {
        Ok(token::create_token(user.uuid, email.to_owned(), config.secret_key.as_ref())?)
    } else {
        Err(ErrorKind::WrongCredentials.into())
    }
}

pub fn auth_resolver<E>(pool: &Pool<ConnectionManager<PgConnection>>, config: &Config, token: String) -> Result<E> where E: From<User> {
    let connection = pool.get()?;
    let auth_data = token::decode_auth(&token, config.secret_key.as_ref())?;
    let user = users_repository::find_by_email(&connection, &auth_data.email)?;
    Ok(user.into())
}
