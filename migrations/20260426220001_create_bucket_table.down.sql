-- Down: drop buckets table
DROP TABLE IF EXISTS buckets CASCADE;
DROP FUNCTION IF EXISTS buckets_audit_timestamp() CASCADE;
