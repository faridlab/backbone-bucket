-- Down: drop bucket.file_locks table
DROP TABLE IF EXISTS bucket.file_locks CASCADE;
DROP FUNCTION IF EXISTS bucket.file_locks_audit_timestamp() CASCADE;
