create table "session" (
    id text primary key not null,
    data Jsonb not null,
    expiry_date timestamptz not null
);
