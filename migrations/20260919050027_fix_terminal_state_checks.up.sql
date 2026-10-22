-- Drop the terminal-state checks that forbade their own state outright.
--
-- The rules they came from (`cannot_modify_completed`, `cannot_modify_deleted`,
-- `cannot_modify_expired`) were spelled against the row about to be stored, so
-- the generated CHECK read `status <> 'completed'` and friends: a CHECK sees
-- only the new row, and the very write that reaches the terminal state tripped
-- it — on a fresh database no guarded write could ever complete a job, delete
-- a bucket or expire a lock. The rules now read the OLD row
-- (`$old.status != ...`), which a CHECK cannot express; the write guard
-- enforces them in the service layer, and the per-table transition trigger
-- still refuses undeclared moves. These three constraints are dropped, not
-- re-added.

ALTER TABLE bucket.processing_jobs DROP CONSTRAINT IF EXISTS ck_processing_jobs_status__job_completed;
ALTER TABLE bucket.buckets DROP CONSTRAINT IF EXISTS ck_buckets_status__bucket_deleted;
ALTER TABLE bucket.file_locks DROP CONSTRAINT IF EXISTS ck_file_locks_status__lock_expired;
