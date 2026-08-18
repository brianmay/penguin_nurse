ALTER TABLE health_metrics
  ADD COLUMN body_fat_pct NUMERIC(4, 1),
  ADD COLUMN bia_details JSONB;
