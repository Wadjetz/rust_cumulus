error_chain! {
    errors {
        AlreadyExist
        NotFound
        NotInserted
    }

    foreign_links {
        Jwt(::jsonwebtoken::errors::Error);
        Uuid(::uuid::ParseError);
        Db(::postgres::error::Error);
    }
}
