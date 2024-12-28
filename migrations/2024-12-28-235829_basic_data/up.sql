CREATE EXTENSION IF NOT EXISTS cube;
CREATE TABLE wees (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGSERIAL NOT NULL,
    time TIMESTAMPTZ NOT NULL,
    duration INTERVAL NOT NULL,
    mls INT NOT NULL,
    hue FLOAT4 NOT NULL,
    saturation FLOAT4 NOT NULL,
    value FLOAT4 NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    FOREIGN KEY (user_id) REFERENCES users (id),
    CONSTRAINT check_mls CHECK (
        mls >= 0
        AND mls <= 5000
    )
);
create index idx_wees_time on wees(time);
create index idx_wees_duration on wees(duration);
CREATE TABLE poos (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGSERIAL NOT NULL,
    time TIMESTAMPTZ NOT NULL,
    duration INTERVAL NOT NULL,
    quantity INT NOT NULL,
    bristol INT NOT NULL,
    hue FLOAT4 NOT NULL,
    saturation FLOAT4 NOT NULL,
    value FLOAT4 NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    FOREIGN KEY (user_id) REFERENCES users (id),
    CONSTRAINT check_bristol CHECK (
        bristol >= 0
        AND bristol <= 7
    ),
    CONSTRAINT check_quantity CHECK (
        quantity >= 0
        AND quantity <= 5
    )
);
create index idx_poos_time on poos(time);
create index idx_poos_duration on poos(duration);