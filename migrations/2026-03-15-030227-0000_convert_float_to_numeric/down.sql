-- Remove check constraints
ALTER TABLE nested_consumables DROP CONSTRAINT IF EXISTS nested_consumables_liquid_mls_non_negative;
ALTER TABLE nested_consumables DROP CONSTRAINT IF EXISTS nested_consumables_quantity_non_negative;
ALTER TABLE consumption_consumables DROP CONSTRAINT IF EXISTS consumption_consumables_liquid_mls_non_negative;
ALTER TABLE consumption_consumables DROP CONSTRAINT IF EXISTS consumption_consumables_quantity_non_negative;
ALTER TABLE consumptions DROP CONSTRAINT IF EXISTS consumption_liquid_mls_non_negative;
-- Convert back to FLOAT
ALTER TABLE nested_consumables
ALTER COLUMN liquid_mls TYPE FLOAT8 USING liquid_mls::FLOAT8;
ALTER TABLE nested_consumables
ALTER COLUMN quantity TYPE FLOAT8 USING quantity::FLOAT8;
ALTER TABLE consumption_consumables
ALTER COLUMN liquid_mls TYPE FLOAT8 USING liquid_mls::FLOAT8;
ALTER TABLE consumption_consumables
ALTER COLUMN quantity TYPE FLOAT8 USING quantity::FLOAT8;
ALTER TABLE consumptions
ALTER COLUMN liquid_mls TYPE FLOAT8 USING liquid_mls::FLOAT8;