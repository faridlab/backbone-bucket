-- Down: drop deferred foreign keys for bucket module
ALTER TABLE bucket.file_locks DROP CONSTRAINT IF EXISTS fk_file_locks_file_id;
