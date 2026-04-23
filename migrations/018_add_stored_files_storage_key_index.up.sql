-- Add unique index on stored_files.storage_key
--
-- Required by the mode-B serving handler, which looks files up by
-- storage_key on every request. Without this index the lookup is a
-- sequential scan and duplicate storage_key rows become possible.
-- Sibling tables (file_versions, thumbnails) already carry the same
-- unique index; stored_files was the outlier.

CREATE UNIQUE INDEX IF NOT EXISTS idx_stored_files_storage_key
    ON stored_files (storage_key);
