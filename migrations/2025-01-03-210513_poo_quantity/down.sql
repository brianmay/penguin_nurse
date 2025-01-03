ALTER TABLE poos DROP CONSTRAINT check_quantity;
ALTER TABLE poos
ADD CONSTRAINT check_quantity CHECK (
        quantity >= 0
        AND quantity <= 5
    );