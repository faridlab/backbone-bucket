-- Down: drop file_shares table
DROP TABLE IF EXISTS file_shares CASCADE;
DROP FUNCTION IF EXISTS file_shares_audit_timestamp() CASCADE;
