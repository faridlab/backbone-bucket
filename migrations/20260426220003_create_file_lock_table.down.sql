-- Down: drop file_locks table
DROP TABLE IF EXISTS file_locks CASCADE;
DROP FUNCTION IF EXISTS file_locks_audit_timestamp() CASCADE;
