# Cumulus

## Exemple .env
```
DATABASE_URL=postgres://cumulus:password@localhost/cumulus
SECRET_KEY=secret
MERCURY_API_KEY=
RSS_JOB_INTERVAL=10
```

## Dev docker-compose.yml
```yaml
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

```sql
INSERT INTO users_feeds (uuid, reaction, user_uuid, feed_uuid, created, updated)
    SELECT uuid_generate_v4() as uuid, 'Unreaded', users_sources.user_uuid, feeds.uuid as feed_uuid, now(), now()
    FROM feeds
    JOIN users_sources ON users_sources.source_uuid = feeds.source_uuid
    WHERE 0 = (
        SELECT COUNT(*)
        FROM users_feeds
        WHERE users_feeds.feed_uuid = feeds.uuid
    )
;
```

```sql
CREATE EXTENSION "uuid-ossp";
```

```sql
ALTER TABLE "users_sources"
ADD "unreaded_count" bigint NOT NULL DEFAULT '0';
```
