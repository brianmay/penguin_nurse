-- Your SQL goes here
ALTER TABLE symptoms
ADD COLUMN feeling_thirsty INTEGER NOT NULL DEFAULT 0 CHECK (
        feeling_thirsty BETWEEN 0 AND 10
    );