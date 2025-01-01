CREATE EXTENSION IF NOT EXISTS cube;
CREATE TABLE wees (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGSERIAL NOT NULL,
    time TIMESTAMPTZ NOT NULL,
    duration INTERVAL NOT NULL,
    urgency INT NOT NULL,
    mls INT NOT NULL,
    colour_hue FLOAT4 NOT NULL,
    colour_saturation FLOAT4 NOT NULL,
    colour_value FLOAT4 NOT NULL,
    comments TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    FOREIGN KEY (user_id) REFERENCES users (id),
    CONSTRAINT check_urgency CHECK (
        urgency >= 0
        AND urgency <= 5
    ),
    CONSTRAINT check_mls CHECK (
        mls >= 0
        AND mls <= 5000
    ),
    CONSTRAINT check_color CHECK (
        colour_hue >= 0
        AND colour_hue <= 360
        AND colour_saturation >= 0
        AND colour_saturation <= 1
        AND colour_value >= 0
        AND colour_value <= 1
    )
);
create index idx_wees_user_id on wees(user_id, time);
CREATE TABLE poos (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGSERIAL NOT NULL,
    time TIMESTAMPTZ NOT NULL,
    duration INTERVAL NOT NULL,
    urgency INT NOT NULL,
    quantity INT NOT NULL,
    bristol INT NOT NULL,
    colour_hue FLOAT4 NOT NULL,
    colour_saturation FLOAT4 NOT NULL,
    colour_value FLOAT4 NOT NULL,
    comments TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    FOREIGN KEY (user_id) REFERENCES users (id),
    CONSTRAINT check_urgency CHECK (
        urgency >= 0
        AND urgency <= 5
    ),
    CONSTRAINT check_bristol CHECK (
        bristol >= 0
        AND bristol <= 7
    ),
    CONSTRAINT check_bristol_quantity CHECK (
        (
            bristol = 0
            AND quantity = 0
        )
        OR (
            bristol > 0
            AND quantity > 0
        )
    ),
    CONSTRAINT check_quantity CHECK (
        quantity >= 0
        AND quantity <= 5
    ),
    CONSTRAINT check_color CHECK (
        colour_hue >= 0
        AND colour_hue <= 360
        AND colour_saturation >= 0
        AND colour_saturation <= 1
        AND colour_value >= 0
        AND colour_value <= 1
    )
);
create index idx_poos_user_id on poos(user_id, time);