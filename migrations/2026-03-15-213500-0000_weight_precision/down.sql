-- Revert weight column from NUMERIC(5, 2) back to NUMERIC(4, 1)
ALTER TABLE health_metrics
ALTER COLUMN weight TYPE NUMERIC(4, 1);