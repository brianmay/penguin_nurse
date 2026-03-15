-- Convert consumption.liquid_mls from FLOAT to NUMERIC
ALTER TABLE consumptions
ALTER COLUMN liquid_mls TYPE NUMERIC(10, 2) USING liquid_mls::NUMERIC(10, 2);
-- Add check constraint to ensure non-negative values
ALTER TABLE consumptions
ADD CONSTRAINT consumption_liquid_mls_non_negative CHECK (liquid_mls >= 0);
-- Convert consumption_consumables.quantity from FLOAT to NUMERIC
ALTER TABLE consumption_consumables
ALTER COLUMN quantity TYPE NUMERIC(10, 3) USING quantity::NUMERIC(10, 3);
-- Add check constraint to ensure non-negative values
ALTER TABLE consumption_consumables
ADD CONSTRAINT consumption_consumables_quantity_non_negative CHECK (quantity >= 0);
-- Convert consumption_consumables.liquid_mls from FLOAT to NUMERIC
ALTER TABLE consumption_consumables
ALTER COLUMN liquid_mls TYPE NUMERIC(10, 2) USING liquid_mls::NUMERIC(10, 2);
-- Add check constraint to ensure non-negative values
ALTER TABLE consumption_consumables
ADD CONSTRAINT consumption_consumables_liquid_mls_non_negative CHECK (liquid_mls >= 0);
-- Convert nested_consumables.quantity from FLOAT to NUMERIC
ALTER TABLE nested_consumables
ALTER COLUMN quantity TYPE NUMERIC(10, 3) USING quantity::NUMERIC(10, 3);
-- Add check constraint to ensure non-negative values
ALTER TABLE nested_consumables
ADD CONSTRAINT nested_consumables_quantity_non_negative CHECK (quantity >= 0);
-- Convert nested_consumables.liquid_mls from FLOAT to NUMERIC
ALTER TABLE nested_consumables
ALTER COLUMN liquid_mls TYPE NUMERIC(10, 2) USING liquid_mls::NUMERIC(10, 2);
-- Add check constraint to ensure non-negative values
ALTER TABLE nested_consumables
ADD CONSTRAINT nested_consumables_liquid_mls_non_negative CHECK (liquid_mls >= 0);