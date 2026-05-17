-- Down: drop stored_files table
DROP TABLE IF EXISTS stored_files CASCADE;
DROP FUNCTION IF EXISTS stored_files_audit_timestamp() CASCADE;
