-- Down: drop bucket.user_quotas table
DROP TABLE IF EXISTS bucket.user_quotas CASCADE;
DROP FUNCTION IF EXISTS bucket.user_quotas_audit_timestamp() CASCADE;
