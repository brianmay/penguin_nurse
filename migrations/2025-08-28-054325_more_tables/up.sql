CREATE TABLE wee_urges(
    id BIGSERIAL PRIMARY KEY,
    user_id BIGSERIAL NOT NULL,
    time TIMESTAMPTZ NOT NULL,
    utc_offset INTEGER NOT NULL,
    urgency INT NOT NULL,
    comments TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    FOREIGN KEY (user_id) REFERENCES users (id),
    CONSTRAINT check_urgency CHECK (
        urgency >= 0
        AND urgency <= 5
    )
);
CREATE INDEX idx_wee_urges_user_id ON wee_urges(user_id, time);
SELECT diesel_manage_updated_at('wee_urges');
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
CREATE TABLE exercises(
    id BIGSERIAL PRIMARY KEY,
    user_id BIGSERIAL NOT NULL,
    time TIMESTAMPTZ NOT NULL,
    utc_offset INTEGER NOT NULL,
    duration INTERVAL NOT NULL,
    location TEXT,
    distance NUMERIC(6, 2) CHECK (
        distance >= 0
        AND distance <= 1000
    ),
    calories INTEGER CHECK (
        calories >= 0
        AND calories <= 10000
    ),
    rpe INT CHECK (
        rpe BETWEEN 1 AND 10
    ),
    exercise_type exercise_type NOT NULL,
    comments TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    FOREIGN KEY (user_id) REFERENCES users (id)
);
CREATE INDEX idx_exercises_user_id ON exercises(user_id, time);
SELECT diesel_manage_updated_at('exercises');
CREATE TABLE health_metrics (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGSERIAL NOT NULL,
    time TIMESTAMPTZ NOT NULL,
    utc_offset INTEGER NOT NULL,
    -- Pulse (beats per minute, usually 40–220 in adults)
    pulse INTEGER CHECK (
        pulse BETWEEN 20 AND 300
    ),
    -- Blood glucose (mmol/L, typical human range 2–30; use NUMERIC for precision)
    blood_glucose NUMERIC(3, 1) CHECK (
        blood_glucose BETWEEN 0 AND 50
    ),
    -- Blood pressure (mmHg: systolic 70–250, diastolic 40–150)
    systolic_bp INTEGER CHECK (
        systolic_bp BETWEEN 50 AND 300
    ),
    diastolic_bp INTEGER CHECK (
        diastolic_bp BETWEEN 30 AND 200
    ),
    CHECK (systolic_bp > diastolic_bp),
    CHECK (
        (
            systolic_bp IS NULL
            AND diastolic_bp IS NULL
        )
        OR (
            systolic_bp IS NOT NULL
            AND diastolic_bp IS NOT NULL
        )
    ),
    -- Weight (kg: newborns ~1–6, adults commonly 30–300, but allow up to 500)
    weight NUMERIC(4, 1) CHECK (
        weight BETWEEN 0 and 500
    ),
    -- Height (cm: typical 30–250, but allow a wider safe bound)
    height INTEGER CHECK (
        height BETWEEN 20 AND 300
    ),
    comments TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    FOREIGN KEY (user_id) REFERENCES users (id)
);
CREATE INDEX idx_health_metrics_user_id ON health_metrics(user_id, time);
SELECT diesel_manage_updated_at('health_metrics');
CREATE TABLE symptoms (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGSERIAL NOT NULL,
    time TIMESTAMPTZ NOT NULL,
    utc_offset INTEGER NOT NULL,
    appetite_loss INTEGER NOT NULL CHECK (
        appetite_loss BETWEEN 0 AND 10
    ),
    fever INTEGER NOT NULL CHECK (
        fever BETWEEN 0 AND 10
    ),
    cough INTEGER NOT NULL CHECK (
        cough BETWEEN 0 AND 10
    ),
    sore_throat INTEGER NOT NULL CHECK (
        sore_throat BETWEEN 0 AND 10
    ),
    runny_nose INTEGER NOT NULL CHECK (
        runny_nose BETWEEN 0 AND 10
    ),
    sneezing INTEGER NOT NULL CHECK (
        sneezing BETWEEN 0 AND 10
    ),
    heart_burn INTEGER NOT NULL CHECK (
        heart_burn BETWEEN 0 AND 10
    ),
    abdominal_pain INTEGER NOT NULL CHECK (
        abdominal_pain BETWEEN 0 AND 10
    ),
    abdominal_pain_location TEXT CHECK (
        (
            abdominal_pain = 0
            AND abdominal_pain_location IS NULL
        )
        OR (
            abdominal_pain > 0
            AND abdominal_pain_location IS NOT NULL
        )
    ),
    diarrhea INTEGER NOT NULL CHECK (
        diarrhea BETWEEN 0 AND 10
    ),
    constipation INTEGER NOT NULL CHECK (
        constipation BETWEEN 0 AND 10
    ),
    lower_back_pain INTEGER NOT NULL CHECK (
        lower_back_pain BETWEEN 0 AND 10
    ),
    upper_back_pain INTEGER NOT NULL CHECK (
        upper_back_pain BETWEEN 0 AND 10
    ),
    neck_pain INTEGER NOT NULL CHECK (
        neck_pain BETWEEN 0 AND 10
    ),
    joint_pain INTEGER NOT NULL CHECK (
        joint_pain BETWEEN 0 AND 10
    ),
    headache INTEGER NOT NULL CHECK (
        headache BETWEEN 0 AND 10
    ),
    nausea INTEGER NOT NULL CHECK (
        nausea BETWEEN 0 AND 10
    ),
    dizziness INTEGER NOT NULL CHECK (
        dizziness BETWEEN 0 AND 10
    ),
    stomach_ache INTEGER NOT NULL CHECK (
        stomach_ache BETWEEN 0 AND 10
    ),
    chest_pain INTEGER NOT NULL CHECK (
        chest_pain BETWEEN 0 AND 10
    ),
    shortness_of_breath INTEGER NOT NULL CHECK (
        shortness_of_breath BETWEEN 0 AND 10
    ),
    fatigue INTEGER NOT NULL CHECK (
        fatigue BETWEEN 0 AND 10
    ),
    anxiety INTEGER NOT NULL CHECK (
        anxiety BETWEEN 0 AND 10
    ),
    depression INTEGER NOT NULL CHECK (
        depression BETWEEN 0 AND 10
    ),
    insomnia INTEGER NOT NULL CHECK (
        insomnia BETWEEN 0 AND 10
    ),
    comments TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    FOREIGN KEY (user_id) REFERENCES users (id)
);
CREATE INDEX idx_symptoms_user_id ON symptoms(user_id, time);
SELECT diesel_manage_updated_at('symptoms');
CREATE TABLE refluxs(
    id BIGSERIAL PRIMARY KEY,
    user_id BIGSERIAL NOT NULL,
    time TIMESTAMPTZ NOT NULL,
    utc_offset INTEGER NOT NULL,
    duration INTERVAL NOT NULL,
    location TEXT,
    severity INT NOT NULL CHECK (
        severity BETWEEN 0 AND 10
    ),
    comments TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    FOREIGN KEY (user_id) REFERENCES users (id)
);
CREATE INDEX idx_refluxs_user_id ON refluxs(user_id, time);
CREATE TABLE notes(
    id BIGSERIAL PRIMARY KEY,
    user_id BIGSERIAL NOT NULL,
    time TIMESTAMPTZ NOT NULL,
    utc_offset INTEGER NOT NULL,
    comments TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    FOREIGN KEY (user_id) REFERENCES users (id)
);
CREATE INDEX idx_notes_user_id ON refluxs(user_id, time);