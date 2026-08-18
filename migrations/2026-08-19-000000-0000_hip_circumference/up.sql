ALTER TABLE health_metrics
ADD COLUMN hip_circumference NUMERIC(4, 1) CHECK (
        hip_circumference BETWEEN 30 AND 300
    );
