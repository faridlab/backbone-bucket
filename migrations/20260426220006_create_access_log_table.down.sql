-- Down: drop bucket.access_logs table
DROP TABLE IF EXISTS bucket.access_logs CASCADE;
DROP FUNCTION IF EXISTS bucket.access_logs_audit_timestamp() CASCADE;
