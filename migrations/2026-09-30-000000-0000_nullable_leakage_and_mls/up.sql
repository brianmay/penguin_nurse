-- Make leakage and mls nullable when not complete
ALTER TABLE wees DROP CONSTRAINT IF EXISTS check_leakage;
ALTER TABLE wees DROP CONSTRAINT IF EXISTS check_mls;
ALTER TABLE wees DROP CONSTRAINT IF EXISTS wees_complete_leakage_mls_check;

ALTER TABLE wees ALTER COLUMN leakage DROP NOT NULL;
ALTER TABLE wees ALTER COLUMN mls DROP NOT NULL;

-- Set defaults for existing complete rows (so they satisfy the upcoming constraint)
UPDATE wees SET leakage = 0 WHERE complete = true AND leakage IS NULL;
UPDATE wees SET mls = 100 WHERE complete = true AND mls IS NULL;

-- Add constraints allowing NULL values
ALTER TABLE wees ADD CONSTRAINT check_leakage CHECK (
    leakage IS NULL OR (leakage >= 0 AND leakage <= 10)
);
ALTER TABLE wees ADD CONSTRAINT check_mls CHECK (
    mls IS NULL OR (mls >= 0 AND mls <= 10000)
);

-- Require leakage and mls when complete
ALTER TABLE wees ADD CONSTRAINT wees_complete_leakage_mls_check CHECK (
    complete = false OR (leakage IS NOT NULL AND mls IS NOT NULL)
);
