-- Add migration script here
-- Add migration script here

CREATE TABLE subscriptions(
    id uuid NOT NULL,
    PRIMARY Key (id),
    email text NOT NULL UNIQUE,
    name TEXT NOT NULL,
    subscribed_at timestamptz NOT NULL
);
