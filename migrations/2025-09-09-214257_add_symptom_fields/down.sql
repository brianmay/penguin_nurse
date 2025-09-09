-- This file should undo anything in `up.sql`
ALTER TABLE symptoms DROP COLUMN IF EXISTS shoulder_pain,
    DROP COLUMN IF EXISTS hand_pain,
    DROP COLUMN IF EXISTS foot_pain,
    DROP COLUMN IF EXISTS wrist_pain,
    DROP COLUMN IF EXISTS dental_pain,
    DROP COLUMN IF EXISTS eye_pain,
    DROP COLUMN IF EXISTS ear_pain,
    DROP COLUMN IF EXISTS feeling_hot,
    DROP COLUMN IF EXISTS feeling_cold;