-- Add down migration script here
DROP TABLE IF EXISTS "comment";
DROP SEQUENCE IF EXISTS comment_id_seq;