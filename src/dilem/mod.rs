pub mod messages;
pub mod conversations;
pub mod conversations_users;

pub const CHAT_EVOLUTIONS_UP: &'static str = r#"
    CREATE TABLE conversations (
        uuid UUID PRIMARY KEY,
        level INT,
        created TIMESTAMP,
        updated TIMESTAMP
    );

    CREATE TABLE messages (
        uuid UUID PRIMARY KEY,
        content TEXT NOT NULL,
        created TIMESTAMP,
        updated TIMESTAMP,
        conversation_uuid UUID NOT NULL REFERENCES conversations(uuid),
        user_uuid UUID NOT NULL REFERENCES users(uuid)
    );

    CREATE TABLE conversations_users (
        uuid UUID PRIMARY KEY,
        conversation_uuid UUID NOT NULL REFERENCES conversations(uuid),
        user_uuid UUID NOT NULL REFERENCES users(uuid)
    );
"#;

pub const CHAT_EVOLUTIONS_DOWN: &'static str = r#"
    DROP TABLE conversations_users;
    DROP TABLE conversations;
    DROP TABLE messages;
"#;

pub const PROFILE_EVOLUTION_UP: &'static str = r#"
    CREATE TYPE Gender AS ENUM (
        'Male', 'Female'
    );
    CREATE TYPE Sexuality AS ENUM (
        'Straigth', 'Gay', 'Bisexual'
    );
    CREATE TABLE profiles (
        uuid UUID PRIMARY KEY,
        gender Gender NOT NULL,
        birthday_date DATE NOT NULL,
        description TEXT,
        sexuality Sexuality,
        location POINT,
        address TEXT
        height INT,
        ethnicity TEXT,
        religion TEXT,
        education TEXT,
        position TEXT,
        employer TEXT,
        speaks TEXT,
        user_uuid UUID NOT NULL REFERENCES users(uuid)
    );
"#;

pub const PROFILE_EVOLUTION_DOWN: &'static str = r#"
    DROP TABLE profiles;

    DROP TYPE "gender";
    DROP TYPE "sexuality";
"#;
