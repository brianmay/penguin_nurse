create table users (
    id BIGSERIAL PRIMARY KEY,
    username text not null unique,
    password text not null,
    full_name text not null,
    oidc_id text unique,
    email text not null,
    is_admin boolean not null default false,
    created_at timestamptz NOT NULL DEFAULT NOW(),
    updated_at timestamptz NOT NULL DEFAULT NOW()
);
create table groups(
    id BIGSERIAL PRIMARY KEY,
    name text not null unique,
    created_at timestamptz NOT NULL DEFAULT NOW(),
    updated_at timestamptz NOT NULL DEFAULT NOW()
);
create table user_groups (
    user_id bigint not null,
    group_id bigint not null,
    foreign key (user_id) references users(id) on update cascade on delete cascade,
    foreign key (group_id) references groups(id) on update cascade on delete cascade,
    PRIMARY KEY(user_id, group_id)
);
create index idx_users_email on users(email);
SELECT diesel_manage_updated_at('users');
SELECT diesel_manage_updated_at('groups');