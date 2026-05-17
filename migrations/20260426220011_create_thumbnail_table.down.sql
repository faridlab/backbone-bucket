-- Down: drop thumbnails table
DROP TABLE IF EXISTS thumbnails CASCADE;
DROP FUNCTION IF EXISTS thumbnails_audit_timestamp() CASCADE;
