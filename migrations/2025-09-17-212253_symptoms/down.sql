-- This file should undo anything in `up.sql`
ALTER TABLE symptoms
ADD COLUMN runny_nose INTEGER NOT NULL DEFAULT 0 CHECK (
        runny_nose BETWEEN 0 AND 10
    );
UPDATE symptoms
SET runny_nose = nasal_symptom
WHERE nasal_symptom > 0
    and nasal_symptom_description = 'Runny Nose';
ALTER TABLE symptoms DROP COLUMN IF EXISTS nasal_symptom,
    DROP COLUMN IF EXISTS nasal_symptom_description;
ALTER TABLE wees DROP COLUMN IF EXISTS leakage;