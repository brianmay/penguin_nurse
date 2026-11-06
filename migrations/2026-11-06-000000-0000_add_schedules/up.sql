CREATE TABLE schedules (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    time_of_day TIME NOT NULL,
    consumption_type consumption_type NOT NULL,
    comments TEXT,
    active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    FOREIGN KEY (user_id) REFERENCES users (id)
);
CREATE INDEX idx_schedules_user_id ON schedules (user_id, active);

CREATE TABLE schedule_consumables (
    parent_id BIGINT NOT NULL,
    consumable_id BIGINT NOT NULL,
    quantity NUMERIC,
    liquid_mls NUMERIC,
    comments TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    FOREIGN KEY (parent_id) REFERENCES schedules (id) ON UPDATE CASCADE ON DELETE CASCADE,
    FOREIGN KEY (consumable_id) REFERENCES consumables (id) ON UPDATE CASCADE ON DELETE CASCADE,
    PRIMARY KEY (parent_id, consumable_id)
);
SELECT diesel_manage_updated_at('schedules');
SELECT diesel_manage_updated_at('schedule_consumables');
