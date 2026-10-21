-- Down: drop the declared rule constraints for bucket module
DROP INDEX IF EXISTS bucket.uq_user_quotas_user_id__quota_exists;
ALTER TABLE IF EXISTS bucket.user_quotas DROP CONSTRAINT IF EXISTS ck_user_quotas_warning_thre_b390032a__invalid_warning_threshold;
ALTER TABLE IF EXISTS bucket.user_quotas DROP CONSTRAINT IF EXISTS ck_user_quotas_max_file_size__file_size_exceeds_storage;
ALTER TABLE IF EXISTS bucket.user_quotas DROP CONSTRAINT IF EXISTS ck_user_quotas_max_file_size__invalid_max_file_size;
ALTER TABLE IF EXISTS bucket.user_quotas DROP CONSTRAINT IF EXISTS ck_user_quotas_limit_bytes__invalid_max_storage;
ALTER TABLE IF EXISTS bucket.user_quotas DROP CONSTRAINT IF EXISTS ck_user_quotas_tier__invalid_tier;
ALTER TABLE IF EXISTS bucket.user_quotas DROP CONSTRAINT IF EXISTS ck_user_quotas_peak_usage_bytes_non_negative__non_negative;
ALTER TABLE IF EXISTS bucket.user_quotas DROP CONSTRAINT IF EXISTS ck_user_quotas_tier_max_length__max_length;
ALTER TABLE IF EXISTS bucket.user_quotas DROP CONSTRAINT IF EXISTS ck_user_quotas_file_count_non_negative__non_negative;
ALTER TABLE IF EXISTS bucket.user_quotas DROP CONSTRAINT IF EXISTS ck_user_quotas_used_bytes_non_negative__non_negative;
ALTER TABLE IF EXISTS bucket.user_quotas DROP CONSTRAINT IF EXISTS ck_user_quotas_limit_bytes_non_negative__non_negative;
ALTER TABLE IF EXISTS bucket.user_quotas DROP CONSTRAINT IF EXISTS ck_user_quotas_user_id_required__required;
DO $$
BEGIN
    IF to_regclass('bucket.uq_user_quotas_user_id__unique') IS NOT NULL AND to_regclass('bucket.idx_user_quotas_user_id') IS NULL THEN
        ALTER INDEX bucket.uq_user_quotas_user_id__unique RENAME TO idx_user_quotas_user_id;
    END IF;
END $$;

ALTER TABLE IF EXISTS bucket.upload_sessions DROP CONSTRAINT IF EXISTS ck_upload_sessions_path__invalid_path;
ALTER TABLE IF EXISTS bucket.upload_sessions DROP CONSTRAINT IF EXISTS ck_upload_sessions_total_chunks__invalid_chunk_count;
ALTER TABLE IF EXISTS bucket.upload_sessions DROP CONSTRAINT IF EXISTS ck_upload_sessions_chunk_size__chunk_too_large;
ALTER TABLE IF EXISTS bucket.upload_sessions DROP CONSTRAINT IF EXISTS ck_upload_sessions_chunk_size__chunk_too_small;
ALTER TABLE IF EXISTS bucket.upload_sessions DROP CONSTRAINT IF EXISTS ck_upload_sessions_file_size__invalid_file_size;
ALTER TABLE IF EXISTS bucket.upload_sessions DROP CONSTRAINT IF EXISTS ck_upload_sessions_uploaded_chunks_non_negative__non_negative;
ALTER TABLE IF EXISTS bucket.upload_sessions DROP CONSTRAINT IF EXISTS ck_upload_sessions_total_chunks_positive__positive;
ALTER TABLE IF EXISTS bucket.upload_sessions DROP CONSTRAINT IF EXISTS ck_upload_sessions_chunk_size_positive__positive;
ALTER TABLE IF EXISTS bucket.upload_sessions DROP CONSTRAINT IF EXISTS ck_upload_sessions_file_size_positive__positive;
ALTER TABLE IF EXISTS bucket.upload_sessions DROP CONSTRAINT IF EXISTS ck_upload_sessions_mime_type_max_length__max_length;
ALTER TABLE IF EXISTS bucket.upload_sessions DROP CONSTRAINT IF EXISTS ck_upload_sessions_filename_max_length__max_length;
ALTER TABLE IF EXISTS bucket.upload_sessions DROP CONSTRAINT IF EXISTS ck_upload_sessions_filename_required__required;
ALTER TABLE IF EXISTS bucket.upload_sessions DROP CONSTRAINT IF EXISTS ck_upload_sessions_path_max_length__max_length;
ALTER TABLE IF EXISTS bucket.upload_sessions DROP CONSTRAINT IF EXISTS ck_upload_sessions_path_required__required;
ALTER TABLE IF EXISTS bucket.upload_sessions DROP CONSTRAINT IF EXISTS ck_upload_sessions_user_id_required__required;
ALTER TABLE IF EXISTS bucket.upload_sessions DROP CONSTRAINT IF EXISTS ck_upload_sessions_bucket_id_required__required;
ALTER TABLE IF EXISTS bucket.thumbnails DROP CONSTRAINT IF EXISTS ck_thumbnails_format_required__required;
ALTER TABLE IF EXISTS bucket.thumbnails DROP CONSTRAINT IF EXISTS ck_thumbnails_mime_type_required__required;
ALTER TABLE IF EXISTS bucket.thumbnails DROP CONSTRAINT IF EXISTS ck_thumbnails_storage_key_required__required;
ALTER TABLE IF EXISTS bucket.thumbnails DROP CONSTRAINT IF EXISTS ck_thumbnails_file_id_required__required;
DO $$
BEGIN
    IF to_regclass('bucket.uq_thumbnails_storage_key__unique') IS NOT NULL AND to_regclass('bucket.idx_thumbnails_storage_key') IS NULL THEN
        ALTER INDEX bucket.uq_thumbnails_storage_key__unique RENAME TO idx_thumbnails_storage_key;
    END IF;
END $$;

DO $$
BEGIN
    IF to_regclass('bucket.uq_thumbnails_size__unique') IS NOT NULL AND to_regclass('bucket.idx_thumbnails_file_id_size') IS NULL THEN
        ALTER INDEX bucket.uq_thumbnails_size__unique RENAME TO idx_thumbnails_file_id_size;
    END IF;
END $$;

ALTER TABLE IF EXISTS bucket.stored_files DROP CONSTRAINT IF EXISTS ck_stored_files_status__file_purged;
ALTER TABLE IF EXISTS bucket.stored_files DROP CONSTRAINT IF EXISTS ck_stored_files_mime_type__invalid_mime_type;
ALTER TABLE IF EXISTS bucket.stored_files DROP CONSTRAINT IF EXISTS ck_stored_files_original_name__filename_too_long;
ALTER TABLE IF EXISTS bucket.stored_files DROP CONSTRAINT IF EXISTS ck_stored_files_path__path_traversal;
ALTER TABLE IF EXISTS bucket.stored_files DROP CONSTRAINT IF EXISTS ck_stored_files_path__path_too_long;
ALTER TABLE IF EXISTS bucket.stored_files DROP CONSTRAINT IF EXISTS ck_stored_files_download_count_non_negative__non_negative;
ALTER TABLE IF EXISTS bucket.stored_files DROP CONSTRAINT IF EXISTS ck_stored_files_storage_key_max_length__max_length;
ALTER TABLE IF EXISTS bucket.stored_files DROP CONSTRAINT IF EXISTS ck_stored_files_storage_key_required__required;
ALTER TABLE IF EXISTS bucket.stored_files DROP CONSTRAINT IF EXISTS ck_stored_files_field_name_max_length__max_length;
ALTER TABLE IF EXISTS bucket.stored_files DROP CONSTRAINT IF EXISTS ck_stored_files_owner_entity_max_length__max_length;
ALTER TABLE IF EXISTS bucket.stored_files DROP CONSTRAINT IF EXISTS ck_stored_files_owner_module_max_length__max_length;
ALTER TABLE IF EXISTS bucket.stored_files DROP CONSTRAINT IF EXISTS ck_stored_files_checksum_max_length__max_length;
ALTER TABLE IF EXISTS bucket.stored_files DROP CONSTRAINT IF EXISTS ck_stored_files_mime_type_max_length__max_length;
ALTER TABLE IF EXISTS bucket.stored_files DROP CONSTRAINT IF EXISTS ck_stored_files_mime_type_required__required;
ALTER TABLE IF EXISTS bucket.stored_files DROP CONSTRAINT IF EXISTS ck_stored_files_size_bytes_non_negative__non_negative;
ALTER TABLE IF EXISTS bucket.stored_files DROP CONSTRAINT IF EXISTS ck_stored_files_original_name_max_length__max_length;
ALTER TABLE IF EXISTS bucket.stored_files DROP CONSTRAINT IF EXISTS ck_stored_files_original_name_required__required;
ALTER TABLE IF EXISTS bucket.stored_files DROP CONSTRAINT IF EXISTS ck_stored_files_path_max_length__max_length;
ALTER TABLE IF EXISTS bucket.stored_files DROP CONSTRAINT IF EXISTS ck_stored_files_path_required__required;
ALTER TABLE IF EXISTS bucket.stored_files DROP CONSTRAINT IF EXISTS ck_stored_files_owner_id_required__required;
ALTER TABLE IF EXISTS bucket.stored_files DROP CONSTRAINT IF EXISTS ck_stored_files_bucket_id_required__required;
DO $$
BEGIN
    IF to_regclass('bucket.uq_stored_files_path__unique') IS NOT NULL AND to_regclass('bucket.idx_stored_files_bucket_id_path_metadata_deleted_at') IS NULL THEN
        ALTER INDEX bucket.uq_stored_files_path__unique RENAME TO idx_stored_files_bucket_id_path_metadata_deleted_at;
    END IF;
END $$;

DROP INDEX IF EXISTS bucket.uq_processing_jobs_job_type__duplicate_job;
ALTER TABLE IF EXISTS bucket.processing_jobs DROP CONSTRAINT IF EXISTS ck_processing_jobs_status__job_completed;
ALTER TABLE IF EXISTS bucket.processing_jobs DROP CONSTRAINT IF EXISTS ck_processing_jobs_priority__invalid_priority;
ALTER TABLE IF EXISTS bucket.processing_jobs DROP CONSTRAINT IF EXISTS ck_processing_jobs_job_type__invalid_job_type;
ALTER TABLE IF EXISTS bucket.processing_jobs DROP CONSTRAINT IF EXISTS ck_processing_jobs_max_retries_positive__positive;
ALTER TABLE IF EXISTS bucket.processing_jobs DROP CONSTRAINT IF EXISTS ck_processing_jobs_retry_count_non_negative__non_negative;
ALTER TABLE IF EXISTS bucket.processing_jobs DROP CONSTRAINT IF EXISTS ck_processing_jobs_file_id_required__required;
ALTER TABLE IF EXISTS bucket.file_versions DROP CONSTRAINT IF EXISTS ck_file_versions_size_bytes_non_negative__non_negative;
ALTER TABLE IF EXISTS bucket.file_versions DROP CONSTRAINT IF EXISTS ck_file_versions_change_summary_max_length__max_length;
ALTER TABLE IF EXISTS bucket.file_versions DROP CONSTRAINT IF EXISTS ck_file_versions_created_by_id_required__required;
ALTER TABLE IF EXISTS bucket.file_versions DROP CONSTRAINT IF EXISTS ck_file_versions_checksum_sha256_max_length__max_length;
ALTER TABLE IF EXISTS bucket.file_versions DROP CONSTRAINT IF EXISTS ck_file_versions_checksum_md5_max_length__max_length;
ALTER TABLE IF EXISTS bucket.file_versions DROP CONSTRAINT IF EXISTS ck_file_versions_mime_type_required__required;
ALTER TABLE IF EXISTS bucket.file_versions DROP CONSTRAINT IF EXISTS ck_file_versions_name_max_length__max_length;
ALTER TABLE IF EXISTS bucket.file_versions DROP CONSTRAINT IF EXISTS ck_file_versions_name_required__required;
ALTER TABLE IF EXISTS bucket.file_versions DROP CONSTRAINT IF EXISTS ck_file_versions_storage_key_required__required;
ALTER TABLE IF EXISTS bucket.file_versions DROP CONSTRAINT IF EXISTS ck_file_versions_file_id_required__required;
DO $$
BEGIN
    IF to_regclass('bucket.uq_file_versions_storage_key__unique') IS NOT NULL AND to_regclass('bucket.idx_file_versions_storage_key') IS NULL THEN
        ALTER INDEX bucket.uq_file_versions_storage_key__unique RENAME TO idx_file_versions_storage_key;
    END IF;
END $$;

DO $$
BEGIN
    IF to_regclass('bucket.uq_file_versions_version_number__unique') IS NOT NULL AND to_regclass('bucket.idx_file_versions_file_id_version_number') IS NULL THEN
        ALTER INDEX bucket.uq_file_versions_version_number__unique RENAME TO idx_file_versions_file_id_version_number;
    END IF;
END $$;

ALTER TABLE IF EXISTS bucket.file_shares DROP CONSTRAINT IF EXISTS ck_file_shares_share_type__missing_recipients;
ALTER TABLE IF EXISTS bucket.file_shares DROP CONSTRAINT IF EXISTS ck_file_shares_max_downloads__invalid_max_downloads;
ALTER TABLE IF EXISTS bucket.file_shares DROP CONSTRAINT IF EXISTS ck_file_shares_token__invalid_token_chars;
ALTER TABLE IF EXISTS bucket.file_shares DROP CONSTRAINT IF EXISTS ck_file_shares_token__invalid_token_length;
ALTER TABLE IF EXISTS bucket.file_shares DROP CONSTRAINT IF EXISTS ck_file_shares_download_count_non_negative__non_negative;
ALTER TABLE IF EXISTS bucket.file_shares DROP CONSTRAINT IF EXISTS ck_file_shares_token_max_length__max_length;
ALTER TABLE IF EXISTS bucket.file_shares DROP CONSTRAINT IF EXISTS ck_file_shares_token_min_length__min_length;
ALTER TABLE IF EXISTS bucket.file_shares DROP CONSTRAINT IF EXISTS ck_file_shares_token_required__required;
ALTER TABLE IF EXISTS bucket.file_shares DROP CONSTRAINT IF EXISTS ck_file_shares_owner_id_required__required;
ALTER TABLE IF EXISTS bucket.file_shares DROP CONSTRAINT IF EXISTS ck_file_shares_file_id_required__required;
DO $$
BEGIN
    IF to_regclass('bucket.uq_file_shares_token__unique') IS NOT NULL AND to_regclass('bucket.idx_file_shares_token') IS NULL THEN
        ALTER INDEX bucket.uq_file_shares_token__unique RENAME TO idx_file_shares_token;
    END IF;
END $$;

ALTER TABLE IF EXISTS bucket.file_locks DROP CONSTRAINT IF EXISTS ck_file_locks_status__lock_expired;
ALTER TABLE IF EXISTS bucket.file_locks DROP CONSTRAINT IF EXISTS ck_file_locks_user_id_required__required;
ALTER TABLE IF EXISTS bucket.file_locks DROP CONSTRAINT IF EXISTS ck_file_locks_file_id_required__required;
DO $$
BEGIN
    IF to_regclass('bucket.uq_file_locks_file_id__unique') IS NOT NULL AND to_regclass('bucket.idx_file_locks_file_id') IS NULL THEN
        ALTER INDEX bucket.uq_file_locks_file_id__unique RENAME TO idx_file_locks_file_id;
    END IF;
END $$;

ALTER TABLE IF EXISTS bucket.file_comments DROP CONSTRAINT IF EXISTS ck_file_comments_status__comment_deleted;
ALTER TABLE IF EXISTS bucket.file_comments DROP CONSTRAINT IF EXISTS ck_file_comments_content__comment_too_long;
ALTER TABLE IF EXISTS bucket.file_comments DROP CONSTRAINT IF EXISTS ck_file_comments_content__empty_comment;
ALTER TABLE IF EXISTS bucket.file_comments DROP CONSTRAINT IF EXISTS ck_file_comments_content_max_length__max_length;
ALTER TABLE IF EXISTS bucket.file_comments DROP CONSTRAINT IF EXISTS ck_file_comments_content_required__required;
ALTER TABLE IF EXISTS bucket.file_comments DROP CONSTRAINT IF EXISTS ck_file_comments_user_id_required__required;
ALTER TABLE IF EXISTS bucket.file_comments DROP CONSTRAINT IF EXISTS ck_file_comments_file_id_required__required;
DROP INDEX IF EXISTS bucket.uq_conversion_jobs_target_format__duplicate_conversion;
ALTER TABLE IF EXISTS bucket.conversion_jobs DROP CONSTRAINT IF EXISTS ck_conversion_jobs_target_format__conversion_not_supported;
ALTER TABLE IF EXISTS bucket.conversion_jobs DROP CONSTRAINT IF EXISTS ck_conversion_jobs_progress_range__range;
ALTER TABLE IF EXISTS bucket.conversion_jobs DROP CONSTRAINT IF EXISTS ck_conversion_jobs_target_format_max_length__max_length;
ALTER TABLE IF EXISTS bucket.conversion_jobs DROP CONSTRAINT IF EXISTS ck_conversion_jobs_target_format_required__required;
ALTER TABLE IF EXISTS bucket.conversion_jobs DROP CONSTRAINT IF EXISTS ck_conversion_jobs_source_file_id_required__required;
ALTER TABLE IF EXISTS bucket.content_hashes DROP CONSTRAINT IF EXISTS ck_content_hashes_fingerprint_max_length__max_length;
ALTER TABLE IF EXISTS bucket.content_hashes DROP CONSTRAINT IF EXISTS ck_content_hashes_reference_count_min__min;
ALTER TABLE IF EXISTS bucket.content_hashes DROP CONSTRAINT IF EXISTS ck_content_hashes_storage_key_max_length__max_length;
ALTER TABLE IF EXISTS bucket.content_hashes DROP CONSTRAINT IF EXISTS ck_content_hashes_storage_key_required__required;
ALTER TABLE IF EXISTS bucket.content_hashes DROP CONSTRAINT IF EXISTS ck_content_hashes_size_bytes_positive__positive;
ALTER TABLE IF EXISTS bucket.content_hashes DROP CONSTRAINT IF EXISTS ck_content_hashes_hash_length__length;
ALTER TABLE IF EXISTS bucket.content_hashes DROP CONSTRAINT IF EXISTS ck_content_hashes_hash_required__required;
DO $$
BEGIN
    IF to_regclass('bucket.uq_content_hashes_hash__unique') IS NOT NULL AND to_regclass('bucket.idx_content_hashes_hash') IS NULL THEN
        ALTER INDEX bucket.uq_content_hashes_hash__unique RENAME TO idx_content_hashes_hash;
    END IF;
END $$;

DROP INDEX IF EXISTS bucket.uq_buckets_slug__slug_exists;
ALTER TABLE IF EXISTS bucket.buckets DROP CONSTRAINT IF EXISTS ck_buckets_status__bucket_deleted;
ALTER TABLE IF EXISTS bucket.buckets DROP CONSTRAINT IF EXISTS ck_buckets_root_path__path_traversal;
ALTER TABLE IF EXISTS bucket.buckets DROP CONSTRAINT IF EXISTS ck_buckets_root_path__path_too_long;
ALTER TABLE IF EXISTS bucket.buckets DROP CONSTRAINT IF EXISTS ck_buckets_slug__invalid_slug_chars;
ALTER TABLE IF EXISTS bucket.buckets DROP CONSTRAINT IF EXISTS ck_buckets_slug__invalid_slug_length;
ALTER TABLE IF EXISTS bucket.buckets DROP CONSTRAINT IF EXISTS ck_buckets_name__invalid_name_length;
ALTER TABLE IF EXISTS bucket.buckets DROP CONSTRAINT IF EXISTS ck_buckets_total_size_bytes_non_negative__non_negative;
ALTER TABLE IF EXISTS bucket.buckets DROP CONSTRAINT IF EXISTS ck_buckets_file_count_non_negative__non_negative;
ALTER TABLE IF EXISTS bucket.buckets DROP CONSTRAINT IF EXISTS ck_buckets_root_path_max_length__max_length;
ALTER TABLE IF EXISTS bucket.buckets DROP CONSTRAINT IF EXISTS ck_buckets_root_path_required__required;
ALTER TABLE IF EXISTS bucket.buckets DROP CONSTRAINT IF EXISTS ck_buckets_owner_id_required__required;
ALTER TABLE IF EXISTS bucket.buckets DROP CONSTRAINT IF EXISTS ck_buckets_slug_max_length__max_length;
ALTER TABLE IF EXISTS bucket.buckets DROP CONSTRAINT IF EXISTS ck_buckets_slug_required__required;
ALTER TABLE IF EXISTS bucket.buckets DROP CONSTRAINT IF EXISTS ck_buckets_name_max_length__max_length;
ALTER TABLE IF EXISTS bucket.buckets DROP CONSTRAINT IF EXISTS ck_buckets_name_required__required;
DO $$
BEGIN
    IF to_regclass('bucket.uq_buckets_slug__unique') IS NOT NULL AND to_regclass('bucket.idx_buckets_slug') IS NULL THEN
        ALTER INDEX bucket.uq_buckets_slug__unique RENAME TO idx_buckets_slug;
    END IF;
END $$;

ALTER TABLE IF EXISTS bucket.access_logs DROP CONSTRAINT IF EXISTS ck_access_logs_error_message_max_length__max_length;
ALTER TABLE IF EXISTS bucket.access_logs DROP CONSTRAINT IF EXISTS ck_access_logs_city_max_length__max_length;
ALTER TABLE IF EXISTS bucket.access_logs DROP CONSTRAINT IF EXISTS ck_access_logs_country_code_max_length__max_length;
ALTER TABLE IF EXISTS bucket.access_logs DROP CONSTRAINT IF EXISTS ck_access_logs_referer_max_length__max_length;
ALTER TABLE IF EXISTS bucket.access_logs DROP CONSTRAINT IF EXISTS ck_access_logs_user_agent_max_length__max_length;
ALTER TABLE IF EXISTS bucket.access_logs DROP CONSTRAINT IF EXISTS ck_access_logs_ip_address_max_length__max_length;
ALTER TABLE IF EXISTS bucket.access_logs DROP CONSTRAINT IF EXISTS ck_access_logs_file_id_required__required;
