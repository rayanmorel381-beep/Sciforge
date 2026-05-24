create table if not exists link_bindings (
    binding_id uuid primary key,
    code text not null unique,
    apk_session_id text not null,
    browser_session_id text,
    account_id uuid,
    expires_at timestamptz not null
);
