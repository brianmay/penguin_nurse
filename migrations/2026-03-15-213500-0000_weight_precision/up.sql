-- Change weight column from NUMERIC(4, 1) to NUMERIC(5, 2) for 0.01 kg precision
ALTER TABLE health_metrics
ALTER COLUMN weight TYPE NUMERIC(5, 2);