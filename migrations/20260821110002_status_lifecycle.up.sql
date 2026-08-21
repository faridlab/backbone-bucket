-- Migration: replace lifecycle booleans with status enums
--
-- Tree-wide convention: one `status` enum field per lifecycle, no boolean
-- impostors (docs/refactoring-schema in the serpa workspace). The boolean
-- migrates only rows deviating from its own column default, so the fold never
-- resurrects expired/exhausted/revoked shares into looking live. The enum
-- type is created unqualified so it lands beside the module's other enum
-- types in public, where the sqlx type_name resolves.

DO $$ BEGIN CREATE TYPE file_version_status AS ENUM ('active', 'deleted'); EXCEPTION WHEN duplicate_object THEN NULL; END $$;

-- file_shares: fold is_active into share_status, renamed to status per the
-- convention; share-specific index names follow the rename.
UPDATE bucket.file_shares SET share_status = 'inactive' WHERE NOT is_active;
ALTER TABLE bucket.file_shares RENAME COLUMN share_status TO status;
DROP INDEX IF EXISTS idx_file_shares_is_active;
DROP INDEX IF EXISTS idx_file_shares_share_status;
DROP INDEX IF EXISTS idx_file_shares_file_id_share_status;
CREATE INDEX idx_file_shares_status ON bucket.file_shares (status);
CREATE INDEX idx_file_shares_file_id_status ON bucket.file_shares (file_id, status);
ALTER TABLE bucket.file_shares DROP COLUMN is_active;

-- file_versions: the soft-delete marker becomes an explicit lifecycle state.
ALTER TABLE bucket.file_versions ADD COLUMN status file_version_status NOT NULL DEFAULT 'active';
UPDATE bucket.file_versions SET status = 'deleted' WHERE is_deleted;
ALTER TABLE bucket.file_versions DROP COLUMN is_deleted;
