CREATE TYPE consumable_unit AS ENUM (
    'millilitres',
    'grams',
    'international_units',
    'number'
);
CREATE TABLE consumables (
    id BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    brand TEXT,
    barcode TEXT,
    is_organic BOOLEAN NOT NULL,
    unit consumable_unit NOT NULL,
    comments TEXT,
    created TIMESTAMPTZ,
    destroyed TIMESTAMPTZ,
    created_at timestamptz NOT NULL DEFAULT NOW(),
    updated_at timestamptz NOT NULL DEFAULT NOW()
);
CREATE index idx_consumable_barcode on consumables(barcode);
CREATE index idx_created on consumables(created, destroyed, name);
CREATE TABLE consumptions (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGSERIAL NOT NULL,
    time TIMESTAMPTZ NOT NULL,
    duration INTERVAL NOT NULL,
    liquid_mls FLOAT,
    comments TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    FOREIGN KEY (user_id) REFERENCES users (id)
);
CREATE index idx_consumptions_user_id on consumptions(user_id, time);
SELECT diesel_manage_updated_at('consumables');
SELECT diesel_manage_updated_at('consumptions');
CREATE TABLE nested_consumables (
    parent_id BIGINT NOT NULL,
    consumable_id BIGINT NOT NULL,
    quantity FLOAT,
    liquid_mls FLOAT,
    comments TEXT,
    created_at timestamptz NOT NULL DEFAULT NOW(),
    updated_at timestamptz NOT NULL DEFAULT NOW(),
    FOREIGN KEY (parent_id) REFERENCES consumables(id) ON UPDATE CASCADE ON DELETE CASCADE,
    FOREIGN KEY (consumable_id) references consumables(id) ON UPDATE CASCADE ON DELETE CASCADE,
    PRIMARY KEY(parent_id, consumable_id)
);
CREATE TABLE consumption_consumables (
    parent_id BIGINT NOT NULL,
    consumable_id BIGINT NOT NULL,
    quantity FLOAT,
    liquid_mls FLOAT,
    comments TEXT,
    created_at timestamptz NOT NULL DEFAULT NOW(),
    updated_at timestamptz NOT NULL DEFAULT NOW(),
    FOREIGN KEY (parent_id) REFERENCES consumptions(id) ON UPDATE CASCADE ON DELETE CASCADE,
    FOREIGN KEY (consumable_id) references consumables(id) ON UPDATE CASCADE ON DELETE CASCADE,
    PRIMARY KEY(parent_id, consumable_id)
);
SELECT diesel_manage_updated_at('nested_consumables');
SELECT diesel_manage_updated_at('consumption_consumables');