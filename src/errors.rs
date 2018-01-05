error_chain! {
    errors {
        AlreadyExist
        NotFound
        NotInserted
        WrongCredentials
        Unauthorized
    }

    foreign_links {
        IO(::std::io::Error);
        Jwt(::jsonwebtoken::errors::Error);
        Uuid(::uuid::ParseError);
        Db(::postgres::error::Error);
        R2D2(::r2d2::Error);
        WS(::reqwest::Error);
        Json(::serde_json::Error);
        Strum(::strum::ParseError);
        Bcrypt(::bcrypt::BcryptError);
        Url(::url::ParseError);
        Diesel(::diesel::result::Error);
    }
}
