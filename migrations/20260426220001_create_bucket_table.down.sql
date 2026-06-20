-- Down: drop bucket.buckets table
DROP TABLE IF EXISTS bucket.buckets CASCADE;
DROP FUNCTION IF EXISTS bucket.buckets_audit_timestamp() CASCADE;
