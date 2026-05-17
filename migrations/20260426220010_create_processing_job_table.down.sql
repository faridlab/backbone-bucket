-- Down: drop processing_jobs table
DROP TABLE IF EXISTS processing_jobs CASCADE;
DROP FUNCTION IF EXISTS processing_jobs_audit_timestamp() CASCADE;
