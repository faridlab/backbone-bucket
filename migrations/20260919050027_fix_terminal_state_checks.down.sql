-- Restore the broken terminal-state checks (see the up file for why they were
-- dropped). Re-adding them re-forbids reaching the terminal state on fresh
-- databases; this down exists only for migration symmetry.

DO $$
BEGIN
    IF to_regclass('bucket.processing_jobs') IS NOT NULL
       AND NOT EXISTS (SELECT 1 FROM pg_constraint WHERE conrelid = to_regclass('bucket.processing_jobs') AND conname = 'ck_processing_jobs_status__job_completed') THEN
        IF EXISTS (SELECT 1 FROM bucket.processing_jobs WHERE NOT (("status"::text <> 'completed'))) THEN
            RAISE NOTICE 'ck_processing_jobs_status__job_completed not re-added: stored rows break it';
        ELSE
            ALTER TABLE bucket.processing_jobs ADD CONSTRAINT ck_processing_jobs_status__job_completed CHECK (("status"::text <> 'completed'));
        END IF;
    END IF;
    IF to_regclass('bucket.buckets') IS NOT NULL
       AND NOT EXISTS (SELECT 1 FROM pg_constraint WHERE conrelid = to_regclass('bucket.buckets') AND conname = 'ck_buckets_status__bucket_deleted') THEN
        IF EXISTS (SELECT 1 FROM bucket.buckets WHERE NOT (("status"::text <> 'deleted'))) THEN
            RAISE NOTICE 'ck_buckets_status__bucket_deleted not re-added: stored rows break it';
        ELSE
            ALTER TABLE bucket.buckets ADD CONSTRAINT ck_buckets_status__bucket_deleted CHECK (("status"::text <> 'deleted'));
        END IF;
    END IF;
    IF to_regclass('bucket.file_locks') IS NOT NULL
       AND NOT EXISTS (SELECT 1 FROM pg_constraint WHERE conrelid = to_regclass('bucket.file_locks') AND conname = 'ck_file_locks_status__lock_expired') THEN
        IF EXISTS (SELECT 1 FROM bucket.file_locks WHERE NOT (("status"::text <> 'expired'))) THEN
            RAISE NOTICE 'ck_file_locks_status__lock_expired not re-added: stored rows break it';
        ELSE
            ALTER TABLE bucket.file_locks ADD CONSTRAINT ck_file_locks_status__lock_expired CHECK (("status"::text <> 'expired'));
        END IF;
    END IF;
END $$;
