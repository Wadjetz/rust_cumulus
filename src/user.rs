use uuid::Uuid;

#[derive(Debug)]
pub struct User {
    pub uuid: Uuid,
    pub login: String,
    pub email: String,
    pub password: String,
    //pub created: String,
    //pub updated: String,
    //pub last_connection: String,
    //pub active: bool,
}

impl User {
    pub fn new(uuid: Uuid, login: String, email: String, password: String) -> Self {
        User {
            uuid, login, email, password,
        }
    }
}

