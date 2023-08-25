-- Add up migration script here
CREATE SEQUENCE notification_id_seq INCREMENT 1 MINVALUE 1 MAXVALUE 2147483647 CACHE 1;

CREATE TABLE "public"."notification" (
    "id" integer DEFAULT nextval('notification_id_seq') NOT NULL,
    "user_id" integer NOT NULL,
    "post_id" integer NOT NULL,
    "notificationable_id" integer,
    "notificationable_type" character varying,
    "read" boolean DEFAULT false NOT NULL,
    "extra_data" json,
    "created_at" timestamptz DEFAULT CURRENT_TIMESTAMP NOT NULL,
    "updated_at" timestamptz DEFAULT CURRENT_TIMESTAMP NOT NULL,
    CONSTRAINT "notification_pkey" PRIMARY KEY ("id")
) WITH (oids = false);
