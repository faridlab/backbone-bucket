-- Down: drop user_quotas table
DROP TABLE IF EXISTS user_quotas CASCADE;
DROP FUNCTION IF EXISTS user_quotas_audit_timestamp() CASCADE;
