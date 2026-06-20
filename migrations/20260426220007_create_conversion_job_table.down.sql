-- Down: drop bucket.conversion_jobs table
DROP TABLE IF EXISTS bucket.conversion_jobs CASCADE;
DROP FUNCTION IF EXISTS bucket.conversion_jobs_audit_timestamp() CASCADE;
