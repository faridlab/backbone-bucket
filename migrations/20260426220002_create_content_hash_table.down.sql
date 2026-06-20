-- Down: drop bucket.content_hashes table
DROP TABLE IF EXISTS bucket.content_hashes CASCADE;
DROP FUNCTION IF EXISTS bucket.content_hashes_audit_timestamp() CASCADE;
