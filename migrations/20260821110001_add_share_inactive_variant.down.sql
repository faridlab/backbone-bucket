-- Down for add_share_inactive_variant: enum values cannot be dropped in
-- place. The status_lifecycle down migration stamps rows out of 'inactive'
-- before the column work; the variant itself lingers harmlessly in the type
-- until the type is recreated (recreate-last resort only).

SELECT 1;
