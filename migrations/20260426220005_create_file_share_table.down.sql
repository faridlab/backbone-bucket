-- Down: drop bucket.file_shares table
DROP TABLE IF EXISTS bucket.file_shares CASCADE;
DROP FUNCTION IF EXISTS bucket.file_shares_audit_timestamp() CASCADE;
