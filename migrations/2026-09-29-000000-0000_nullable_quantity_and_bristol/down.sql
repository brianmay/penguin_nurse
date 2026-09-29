-- Reverse: make quantity and bristol required again
ALTER TABLE poos DROP CONSTRAINT poos_complete_quantity_bristol_check;
ALTER TABLE poos DROP CONSTRAINT check_bristol_quantity;
ALTER TABLE poos DROP CONSTRAINT check_quantity;
ALTER TABLE poos DROP CONSTRAINT check_bristol;

-- Set default values before making NOT NULL
UPDATE poos SET quantity = 0 WHERE quantity IS NULL;
UPDATE poos SET bristol = 0 WHERE bristol IS NULL;

ALTER TABLE poos ALTER COLUMN quantity SET NOT NULL;
ALTER TABLE poos ALTER COLUMN bristol SET NOT NULL;

-- Re-add original constraints
ALTER TABLE poos ADD CONSTRAINT check_bristol CHECK (
    bristol >= 0 AND bristol <= 7
);
ALTER TABLE poos ADD CONSTRAINT check_quantity CHECK (
    quantity >= 0 AND quantity <= 10
);
ALTER TABLE poos ADD CONSTRAINT check_bristol_quantity CHECK (
    (
        bristol = 0
        AND quantity = 0
    )
    OR (
        bristol > 0
        AND quantity > 0
    )
);
