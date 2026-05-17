-- Down: drop access_logs table
DROP TABLE IF EXISTS access_logs CASCADE;
DROP FUNCTION IF EXISTS access_logs_audit_timestamp() CASCADE;
