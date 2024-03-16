-- migrations/20240204220437_create_subscriptions_table.sql
-- Create Subscriptions Table
CREATE TABLE subscriptions (
    id uuid NOT NULL,
    email TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    subscribed_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY (id)
);
