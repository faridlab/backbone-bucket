-- Down: drop file_versions table
DROP TABLE IF EXISTS file_versions CASCADE;
DROP FUNCTION IF EXISTS file_versions_audit_timestamp() CASCADE;
