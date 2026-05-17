-- Down: drop file_comments table
DROP TABLE IF EXISTS file_comments CASCADE;
DROP FUNCTION IF EXISTS file_comments_audit_timestamp() CASCADE;
