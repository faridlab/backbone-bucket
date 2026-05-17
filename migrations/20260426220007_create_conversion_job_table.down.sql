-- Down: drop conversion_jobs table
DROP TABLE IF EXISTS conversion_jobs CASCADE;
DROP FUNCTION IF EXISTS conversion_jobs_audit_timestamp() CASCADE;
