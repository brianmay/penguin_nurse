-- Make colour fields nullable in wees table
ALTER TABLE wees
ALTER COLUMN colour_hue DROP NOT NULL,
    ALTER COLUMN colour_saturation DROP NOT NULL,
    ALTER COLUMN colour_value DROP NOT NULL;
-- Drop old color check constraint and add new ones for wees
ALTER TABLE wees DROP CONSTRAINT check_color;
-- Update existing entries: set colour to NULL where mls is 0
UPDATE wees SET colour_hue = NULL, colour_saturation = NULL, colour_value = NULL WHERE mls = 0;
-- If mls is 0, all colour fields must be NULL
-- If any colour field is NOT NULL, all must be NOT NULL and within valid ranges
ALTER TABLE wees
ADD CONSTRAINT check_color CHECK (
        (
            mls = 0
            AND colour_hue IS NULL
            AND colour_saturation IS NULL
            AND colour_value IS NULL
        )
        OR (
            mls > 0
            AND (
                (
                    colour_hue IS NULL
                    AND colour_saturation IS NULL
                    AND colour_value IS NULL
                )
                OR (
                    colour_hue IS NOT NULL
                    AND colour_saturation IS NOT NULL
                    AND colour_value IS NOT NULL
                    AND colour_hue >= 0
                    AND colour_hue <= 360
                    AND colour_saturation >= 0
                    AND colour_saturation <= 1
                    AND colour_value >= 0
                    AND colour_value <= 1
                )
            )
        )
    );
-- Make colour fields nullable in poos table
ALTER TABLE poos
ALTER COLUMN colour_hue DROP NOT NULL,
    ALTER COLUMN colour_saturation DROP NOT NULL,
    ALTER COLUMN colour_value DROP NOT NULL;
-- Drop old color check constraint and add new ones for poos
ALTER TABLE poos DROP CONSTRAINT check_color;
-- Update existing entries: set colour to NULL where quantity is 0
UPDATE poos SET colour_hue = NULL, colour_saturation = NULL, colour_value = NULL WHERE quantity = 0;
-- If quantity is 0, all colour fields must be NULL
-- If any colour field is NOT NULL, all must be NOT NULL and within valid ranges
ALTER TABLE poos
ADD CONSTRAINT check_color CHECK (
        (
            quantity = 0
            AND colour_hue IS NULL
            AND colour_saturation IS NULL
            AND colour_value IS NULL
        )
        OR (
            quantity > 0
            AND (
                (
                    colour_hue IS NULL
                    AND colour_saturation IS NULL
                    AND colour_value IS NULL
                )
                OR (
                    colour_hue IS NOT NULL
                    AND colour_saturation IS NOT NULL
                    AND colour_value IS NOT NULL
                    AND colour_hue >= 0
                    AND colour_hue <= 360
                    AND colour_saturation >= 0
                    AND colour_saturation <= 1
                    AND colour_value >= 0
                    AND colour_value <= 1
                )
            )
        )
    );