-- Down: drop bucket.file_versions table
DROP TABLE IF EXISTS bucket.file_versions CASCADE;
DROP FUNCTION IF EXISTS bucket.file_versions_audit_timestamp() CASCADE;
