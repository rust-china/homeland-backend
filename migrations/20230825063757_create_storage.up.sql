-- Add up migration script here
CREATE SEQUENCE storage_id_seq INCREMENT 1 MINVALUE 1 MAXVALUE 2147483647 CACHE 1;

CREATE TABLE "public"."storage" (
    "id" integer DEFAULT nextval('storage_id_seq') NOT NULL,
    "user_id" integer NOT NULL,
    "path" character varying NOT NULL,
    "content_type" character varying,
    "size" integer NOT NULL,
    "visited_count" integer DEFAULT '0' NOT NULL,
    "today_visited_count" integer DEFAULT '0' NOT NULL,
    "created_at" timestamptz DEFAULT CURRENT_TIMESTAMP NOT NULL,
    "updated_at" timestamptz DEFAULT CURRENT_TIMESTAMP NOT NULL,
    CONSTRAINT "storage_path_key" UNIQUE ("path"),
    CONSTRAINT "storage_pkey" PRIMARY KEY ("id")
) WITH (oids = false);

