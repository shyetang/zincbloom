-- Add is_banned column to posts table
ALTER TABLE posts ADD COLUMN is_banned BOOLEAN NOT NULL DEFAULT FALSE;

-- Add index for better query performance on banned posts
CREATE INDEX idx_posts_is_banned ON posts(is_banned);
