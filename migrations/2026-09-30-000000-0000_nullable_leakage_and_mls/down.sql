-- Reverse: make leakage and mls required again
ALTER TABLE wees DROP CONSTRAINT IF EXISTS wees_complete_leakage_mls_check;
ALTER TABLE wees DROP CONSTRAINT IF EXISTS check_mls;
ALTER TABLE wees DROP CONSTRAINT IF EXISTS check_leakage;

-- Set default values before making NOT NULL
UPDATE wees SET leakage = 0 WHERE leakage IS NULL;
UPDATE wees SET mls = 0 WHERE mls IS NULL;

ALTER TABLE wees ALTER COLUMN leakage SET NOT NULL;
ALTER TABLE wees ALTER COLUMN mls SET NOT NULL;
