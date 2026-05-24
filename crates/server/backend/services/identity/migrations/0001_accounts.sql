create table if not exists identity_accounts (
    id uuid primary key,
    username text not null unique,
    device_id text not null,
    created_at timestamptz not null default now()
);
