-- Down: restore the module's global unique indexes (the pre-posture state,
-- for an undecorated deployment that wants holding-wide uniqueness back).
CREATE UNIQUE INDEX IF NOT EXISTS idx_buckets_slug ON bucket.buckets (slug);
CREATE UNIQUE INDEX IF NOT EXISTS idx_content_hashes_hash ON bucket.content_hashes (hash);
CREATE UNIQUE INDEX IF NOT EXISTS idx_user_quotas_user_id ON bucket.user_quotas (user_id);
