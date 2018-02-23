-- Your SQL goes here

ALTER TABLE sources ADD COLUMN url TEXT UNIQUE;
ALTER TABLE sources ADD COLUMN title TEXT;
ALTER TABLE sources ADD COLUMN website TEXT;

UPDATE
    sources
SET
    title = s2.t,
    url = s2.u,
    website = s2.w
FROM (
    SELECT
       uuid,
       sources.data ->> 'title' as t,
       sources.data ->> 'xml_url' as u,
       sources.data ->> 'html_url' as w
    FROM sources
) as s2
WHERE sources.uuid = s2.uuid;

ALTER TABLE sources DROP COLUMN source_type;
ALTER TABLE sources DROP COLUMN data;

ALTER TABLE sources ALTER COLUMN created SET NOT NULL;
ALTER TABLE sources ALTER COLUMN updated SET NOT NULL;

ALTER TABLE users ALTER COLUMN created SET NOT NULL;
ALTER TABLE users ALTER COLUMN updated SET NOT NULL;

ALTER TABLE sources ALTER COLUMN url SET NOT NULL;
ALTER TABLE sources ALTER COLUMN title SET NOT NULL;
ALTER TABLE sources ALTER COLUMN website SET NOT NULL;
