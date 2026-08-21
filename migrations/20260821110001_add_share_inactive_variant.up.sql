-- Migration: add the `inactive` variant to share_status
--
-- `file_shares.is_active` folds into the existing share_status enum (the
-- tree-wide one-status-field convention lives in docs/refactoring-schema in
-- the serpa workspace). A deactivated-but-not-revoked share needs its own
-- variant: `revoked` means deauthorized, `expired`/`exhausted` mean the share
-- ran out on its own terms.
--
-- This stamp only adds the value: Postgres cannot reliably use a newly added
-- enum value inside the same transaction that added it, so the data
-- translation rides the next stamp (20260821110002_status_lifecycle).

ALTER TYPE share_status ADD VALUE IF NOT EXISTS 'inactive';
