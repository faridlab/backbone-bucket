-- Down: drop the workflow engine tables of the bucket schema
DROP TABLE IF EXISTS bucket.bb_workflow_steps;
DROP TABLE IF EXISTS bucket.bb_workflow_instances;
