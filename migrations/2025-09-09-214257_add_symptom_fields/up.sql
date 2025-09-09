-- Your SQL goes here
ALTER TABLE symptoms
ADD COLUMN shoulder_pain INTEGER NOT NULL DEFAULT 0 CHECK (
        shoulder_pain BETWEEN 0 AND 10
    ),
    ADD COLUMN hand_pain INTEGER NOT NULL DEFAULT 0 CHECK (
        hand_pain BETWEEN 0 AND 10
    ),
    ADD COLUMN foot_pain INTEGER NOT NULL DEFAULT 0 CHECK (
        foot_pain BETWEEN 0 AND 10
    ),
    ADD COLUMN wrist_pain INTEGER NOT NULL DEFAULT 0 CHECK (
        wrist_pain BETWEEN 0 AND 10
    ),
    ADD COLUMN dental_pain INTEGER NOT NULL DEFAULT 0 CHECK (
        dental_pain BETWEEN 0 AND 10
    ),
    ADD COLUMN eye_pain INTEGER NOT NULL DEFAULT 0 CHECK (
        eye_pain BETWEEN 0 AND 10
    ),
    ADD COLUMN ear_pain INTEGER NOT NULL DEFAULT 0 CHECK (
        ear_pain BETWEEN 0 AND 10
    ),
    ADD COLUMN feeling_hot INTEGER NOT NULL DEFAULT 0 CHECK (
        feeling_hot BETWEEN 0 AND 10
    ),
    ADD COLUMN feeling_cold INTEGER NOT NULL DEFAULT 0 CHECK (
        feeling_cold BETWEEN 0 AND 10
    );