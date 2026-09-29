-- Make quantity and bristol nullable when not complete
ALTER TABLE poos DROP CONSTRAINT check_bristol_quantity;
ALTER TABLE poos DROP CONSTRAINT check_quantity;
ALTER TABLE poos DROP CONSTRAINT check_bristol;

ALTER TABLE poos ALTER COLUMN quantity DROP NOT NULL;
ALTER TABLE poos ALTER COLUMN bristol DROP NOT NULL;

-- Set defaults for existing complete rows (so they satisfy the upcoming constraint)
UPDATE poos SET quantity = 1 WHERE complete = true AND quantity IS NULL;
UPDATE poos SET bristol = 4 WHERE complete = true AND bristol IS NULL;

-- Re-add constraints allowing NULL values
ALTER TABLE poos ADD CONSTRAINT check_bristol CHECK (
    bristol IS NULL OR (bristol >= 0 AND bristol <= 7)
);
ALTER TABLE poos ADD CONSTRAINT check_quantity CHECK (
    quantity IS NULL OR (quantity >= 0 AND quantity <= 10)
);
ALTER TABLE poos ADD CONSTRAINT check_bristol_quantity CHECK (
    (bristol IS NULL AND quantity IS NULL)
    OR (bristol = 0 AND quantity = 0)
    OR (bristol > 0 AND bristol <= 7 AND quantity > 0 AND quantity <= 10)
);

-- Require quantity and bristol when complete
ALTER TABLE poos ADD CONSTRAINT poos_complete_quantity_bristol_check CHECK (
    complete = false OR (quantity IS NOT NULL AND bristol IS NOT NULL)
);
