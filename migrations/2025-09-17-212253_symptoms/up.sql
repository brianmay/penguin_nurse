-- Your SQL goes here
ALTER TABLE symptoms
ADD COLUMN nasal_symptom INTEGER NOT NULL DEFAULT 0 CHECK (
        nasal_symptom BETWEEN 0 AND 10
    ),
    ADD COLUMN nasal_symptom_description TEXT CHECK (
        (
            nasal_symptom = 0
            AND nasal_symptom_description IS NULL
        )
        OR (
            nasal_symptom > 0
            AND nasal_symptom_description IS NOT NULL
        )
    );
UPDATE symptoms
SET nasal_symptom = runny_nose,
    nasal_symptom_description = 'Runny Nose'
WHERE runny_nose > 0;
ALTER TABLE symptoms DROP COLUMN IF EXISTS runny_nose;
ALTER TABLE wees
ADD COLUMN leakage INTEGER NOT NULL DEFAULT 0 CHECK (
        leakage BETWEEN 0 AND 10
    );