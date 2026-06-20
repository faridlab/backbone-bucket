-- Down: drop bucket.processing_jobs table
DROP TABLE IF EXISTS bucket.processing_jobs CASCADE;
DROP FUNCTION IF EXISTS bucket.processing_jobs_audit_timestamp() CASCADE;
