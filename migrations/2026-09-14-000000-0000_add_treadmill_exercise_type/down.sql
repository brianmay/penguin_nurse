ALTER TYPE exercise_type RENAME TO exercise_type_old;
CREATE TYPE exercise_type AS ENUM (
    'walking',
    'running',
    'cycling',
    'indoor_cycling',
    'jumping',
    'skipping',
    'flying',
    'other'
);
ALTER TABLE exercises
    ALTER COLUMN exercise_type TYPE exercise_type
    USING exercise_type::text::exercise_type;
DROP TYPE exercise_type_old;
