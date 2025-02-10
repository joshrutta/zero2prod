-- Add migration script here
-- Update Subscriptions Table
ALTER TABLE subscriptions ADD COLUMN status TEXT NULL;