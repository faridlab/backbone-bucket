-- Down: drop bucket.stored_files table
DROP TABLE IF EXISTS bucket.stored_files CASCADE;
DROP FUNCTION IF EXISTS bucket.stored_files_audit_timestamp() CASCADE;
