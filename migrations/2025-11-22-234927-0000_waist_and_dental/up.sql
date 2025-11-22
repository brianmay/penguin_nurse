ALTER TABLE symptoms
ADD COLUMN dental_pain_description TEXT;
UPDATE symptoms
SET dental_pain_description = 'No description'
WHERE dental_pain > 0;
ALTER TABLE symptoms
ADD CONSTRAINT dental_pain_description_check CHECK (
        (
            dental_pain = 0
            AND dental_pain_description IS NULL
        )
        OR (
            dental_pain > 0
            AND dental_pain_description IS NOT NULL
        )
    );
ALTER TABLE health_metrics
ADD COLUMN waist_circumference NUMERIC(4, 1) CHECK (
        waist_circumference BETWEEN 30 AND 300
    );