-- Down: drop bucket.thumbnails table
DROP TABLE IF EXISTS bucket.thumbnails CASCADE;
DROP FUNCTION IF EXISTS bucket.thumbnails_audit_timestamp() CASCADE;
