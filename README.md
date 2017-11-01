# Cumulus

## Exemple .env
```
DATABASE_URL=postgres://cumulus:password@localhost/cumulus
SECRET_KEY=secret
MERCURY_API_KEY=
RSS_JOB_INTERVAL=10
```

## Dev docker-compose.yml
```
version: '3'
services:
    postgres:
        image: postgres:10.0-alpine
        environment:
            POSTGRES_PASSWORD: password
            POSTGRES_USER: cumulus
            POSTGRES_DB: cumulus
        ports:
            - 5432:5432
        volumes:
            - .pg-data/mongodb:/var/lib/postgresql/data

    adminer:
        image: adminer
        ports:
            - 5555:8080
```

ALTER TABLE "users_sources"
ADD "unreaded_count" bigint NOT NULL DEFAULT '0';
