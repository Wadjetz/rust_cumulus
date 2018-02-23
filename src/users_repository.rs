use diesel;
use diesel::prelude::*;
use diesel::PgConnection;

use errors::*;
use schema::users;
use schema::users::dsl::*;
use user::User;

pub fn insert(connection: &PgConnection, user: &User) -> Result<User> {
    Ok(diesel::insert_into(users::table).values(user).get_result(connection)?)
}

pub fn find_by_email(connection: &PgConnection, searched_email: &str) -> Result<User> {
    Ok(users.filter(email.eq(searched_email)).first::<User>(&*connection)?)
}
