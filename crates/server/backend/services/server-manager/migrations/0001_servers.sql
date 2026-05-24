create table if not exists game_servers (
    id uuid primary key,
    owner_account_id uuid not null,
    link_code text not null,
    join_code text not null unique,
    access_code text,
    realtime_endpoint text not null,
    name text not null,
    region text not null,
    mode text not null,
    visibility text not null,
    max_players integer not null,
    state text not null,
    created_at timestamptz not null,
    updated_at timestamptz not null
);

alter table game_servers add column if not exists access_code text;
alter table game_servers add column if not exists realtime_endpoint text not null default '';

