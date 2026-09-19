-- Drop the module's three GLOBAL unique indexes.
--
-- Uniqueness across a whole database is a tenancy POSTURE, not a domain rule
-- of buckets: two org units in one holding may each own a bucket called
-- `invoices`, dedup a content hash within their own fence, and hold one quota
-- row per user per unit. The composing service's tenancy decorator declares
-- the per-unit uniques; a module-owned global index would sit underneath them
-- and keep rejecting the second row, so the decorator's declaration could
-- never take effect (decorators only add indexes, never drop a module's).
--
-- Left GLOBAL on purpose: `file_shares.token` (a secret) and
-- `file_versions.storage_key` (an object-storage address) must stay unique
-- across the entire database and are not touched here.
DROP INDEX IF EXISTS bucket.idx_buckets_slug;
DROP INDEX IF EXISTS bucket.idx_content_hashes_hash;
DROP INDEX IF EXISTS bucket.idx_user_quotas_user_id;
