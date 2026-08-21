-- Down for status_lifecycle: restore the lifecycle booleans.
--
-- Best-effort restore: the file_shares fold is lossy in reverse — rows that
-- entered `inactive` after the up stamp map back to false, and pre-existing
-- revoked shares cannot be distinguished from folded ones (both become
-- is_active = true here except the folded inactive rows). The `inactive`
-- variant added by 20260821110001 lingers in share_status (enum values
-- cannot be dropped in place).

ALTER TABLE bucket.file_versions ADD COLUMN is_deleted BOOLEAN NOT NULL DEFAULT false;
UPDATE bucket.file_versions SET is_deleted = true WHERE status = 'deleted';
ALTER TABLE bucket.file_versions DROP COLUMN status;
DROP TYPE IF EXISTS file_version_status;

ALTER TABLE bucket.file_shares ADD COLUMN is_active BOOLEAN NOT NULL DEFAULT true;
UPDATE bucket.file_shares SET is_active = false WHERE status = 'inactive';
ALTER TABLE bucket.file_shares RENAME COLUMN status TO share_status;
DROP INDEX IF EXISTS idx_file_shares_status;
DROP INDEX IF EXISTS idx_file_shares_file_id_status;
CREATE INDEX idx_file_shares_is_active ON bucket.file_shares (is_active);
CREATE INDEX idx_file_shares_share_status ON bucket.file_shares (share_status);
CREATE INDEX idx_file_shares_file_id_share_status ON bucket.file_shares (file_id, share_status);
