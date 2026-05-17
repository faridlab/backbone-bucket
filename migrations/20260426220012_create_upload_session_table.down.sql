-- Down: drop upload_sessions table
DROP TABLE IF EXISTS upload_sessions CASCADE;
DROP FUNCTION IF EXISTS upload_sessions_audit_timestamp() CASCADE;
