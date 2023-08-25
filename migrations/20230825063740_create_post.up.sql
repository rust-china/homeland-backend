-- Add up migration script here
CREATE SEQUENCE post_id_seq INCREMENT 1 MINVALUE 1 MAXVALUE 2147483647 CACHE 1;

CREATE TABLE "public"."post" (
    "id" integer DEFAULT nextval('post_id_seq') NOT NULL,
    "uuid" uuid NOT NULL,
    "user_id" integer NOT NULL,
    "category_id" integer NOT NULL,
    "title" character varying NOT NULL,
    "body" text NOT NULL,
    "score" integer DEFAULT '0' NOT NULL,
    "read_count" integer DEFAULT '0' NOT NULL,
    "like_count" integer DEFAULT '0' NOT NULL,
    "comment_count" integer DEFAULT '0' NOT NULL,
    "extra_data" json,
    "last_comment_at" timestamptz,
    "deleted_at" timestamptz,
    "created_at" timestamptz DEFAULT CURRENT_TIMESTAMP NOT NULL,
    "updated_at" timestamptz DEFAULT CURRENT_TIMESTAMP NOT NULL,
    CONSTRAINT "post_pkey" PRIMARY KEY ("id"),
    CONSTRAINT "post_uuid_key" UNIQUE ("uuid")
) WITH (oids = false);

CREATE INDEX "idx-post-title" ON "public"."post" USING btree ("title");
