-- Down: drop content_hashes table
DROP TABLE IF EXISTS content_hashes CASCADE;
DROP FUNCTION IF EXISTS content_hashes_audit_timestamp() CASCADE;
