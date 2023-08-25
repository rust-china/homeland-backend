-- Add up migration script here
CREATE SEQUENCE like_id_seq INCREMENT 1 MINVALUE 1 MAXVALUE 2147483647 CACHE 1;

CREATE TABLE "public"."like" (
    "id" integer DEFAULT nextval('like_id_seq') NOT NULL,
    "user_id" integer NOT NULL,
    "like_able_id" integer NOT NULL,
    "like_able_type" character varying NOT NULL,
    "created_at" timestamptz DEFAULT CURRENT_TIMESTAMP NOT NULL,
    "updated_at" timestamptz DEFAULT CURRENT_TIMESTAMP NOT NULL,
    CONSTRAINT "like_pkey" PRIMARY KEY ("id")
) WITH (oids = false);
