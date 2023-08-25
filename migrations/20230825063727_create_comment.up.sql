-- Add up migration script here
CREATE SEQUENCE comment_id_seq INCREMENT 1 MINVALUE 1 MAXVALUE 2147483647 CACHE 1;

CREATE TABLE "public"."comment" (
    "id" integer DEFAULT nextval('comment_id_seq') NOT NULL,
    "user_id" integer NOT NULL,
    "post_id" integer NOT NULL,
    "like_count" integer DEFAULT '0' NOT NULL,
    "comment_count" integer DEFAULT '0' NOT NULL,
    "ancestry" character varying,
    "body" character varying NOT NULL,
    "extra_data" json,
    "deleted_at" timestamptz,
    "created_at" timestamptz DEFAULT CURRENT_TIMESTAMP NOT NULL,
    "updated_at" timestamptz DEFAULT CURRENT_TIMESTAMP NOT NULL,
    CONSTRAINT "comment_pkey" PRIMARY KEY ("id")
) WITH (oids = false);
