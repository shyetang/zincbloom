-- Add migration script here
-- backend/migrations/YYYYMMDDHHMMSS_add_verified_at_to_users.sql
ALTER TABLE users
    ADD COLUMN email_verified_at TIMESTAMPTZ; -- 初始为 NULL，表示未验证