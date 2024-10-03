-- migrations/{timestamp}_create_subscriptions_table.sql
-- Create Subscriptions Table
-- • we are keeping track of when a subscription is created with subscribed_at (timestamptz is atime-zone aware date and time type);
-- • we are enforcing email uniqueness at the database-level with a UNIQUE constraint;
-- • we are enforcing that all fields should be populated with a NOT NULL constraint on each column;
-- • we are using TEXT for email and name because we do not have any restriction on their maximum
-- lengths
CREATE TABLE subscriptions (
    id uuid NOT NULL,
    PRIMARY KEY (id),
    email TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    subscribed_at timestamptz NOT NULL
)
