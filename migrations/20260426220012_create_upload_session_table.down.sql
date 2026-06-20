-- Down: drop bucket.upload_sessions table
DROP TABLE IF EXISTS bucket.upload_sessions CASCADE;
DROP FUNCTION IF EXISTS bucket.upload_sessions_audit_timestamp() CASCADE;
