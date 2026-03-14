-- Revert wees table changes
ALTER TABLE wees DROP CONSTRAINT check_color;
-- Set any NULL colour values to default values before making NOT NULL
UPDATE wees
SET colour_hue = 0,
    colour_saturation = 0,
    colour_value = 0
WHERE colour_hue IS NULL
    OR colour_saturation IS NULL
    OR colour_value IS NULL;
ALTER TABLE wees
ALTER COLUMN colour_hue
SET NOT NULL,
    ALTER COLUMN colour_saturation
SET NOT NULL,
    ALTER COLUMN colour_value
SET NOT NULL;
ALTER TABLE wees
ADD CONSTRAINT check_color CHECK (
        colour_hue >= 0
        AND colour_hue <= 360
        AND colour_saturation >= 0
        AND colour_saturation <= 1
        AND colour_value >= 0
        AND colour_value <= 1
    );
-- Revert poos table changes
ALTER TABLE poos DROP CONSTRAINT check_color;
-- Set any NULL colour values to default values before making NOT NULL
UPDATE poos
SET colour_hue = 0,
    colour_saturation = 0,
    colour_value = 0
WHERE colour_hue IS NULL
    OR colour_saturation IS NULL
    OR colour_value IS NULL;
ALTER TABLE poos
ALTER COLUMN colour_hue
SET NOT NULL,
    ALTER COLUMN colour_saturation
SET NOT NULL,
    ALTER COLUMN colour_value
SET NOT NULL;
ALTER TABLE poos
ADD CONSTRAINT check_color CHECK (
        colour_hue >= 0
        AND colour_hue <= 360
        AND colour_saturation >= 0
        AND colour_saturation <= 1
        AND colour_value >= 0
        AND colour_value <= 1
    );