-- Add up migration script here
-- Add down migration script here
DROP SEQUENCE IF EXISTS category_id_seq;
CREATE SEQUENCE category_id_seq INCREMENT 1 MINVALUE 1 MAXVALUE 2147483647 CACHE 1;

CREATE TABLE "public"."category" (
    "id" integer DEFAULT nextval('category_id_seq') NOT NULL,
    "user_id" integer,
    "code" character varying NOT NULL,
    "name" character varying NOT NULL,
    "position" integer DEFAULT '0' NOT NULL,
    "ancestry" character varying,
    "created_at" timestamptz DEFAULT CURRENT_TIMESTAMP NOT NULL,
    "updated_at" timestamptz DEFAULT CURRENT_TIMESTAMP NOT NULL,
    CONSTRAINT "category_code_key" UNIQUE ("code"),
    CONSTRAINT "category_pkey" PRIMARY KEY ("id")
) WITH (oids = false);

INSERT INTO "category" ("id", "user_id", "code", "name", "position", "ancestry", "created_at", "updated_at") VALUES
(1,	NULL,	'POST',	'文章',	0,	NULL,	'2023-07-23 10:29:23.91785+00',	'2023-07-23 10:29:23.91785+00'),
(2,	NULL,	'RUST',	'Rust',	0,	'/1',	'2023-07-23 10:29:23.91785+00',	'2023-07-23 10:29:23.91785+00'),
(3,	NULL,	'CRATE',	'Crate',	0,	'/1',	'2023-07-23 10:29:23.91785+00',	'2023-07-23 10:29:23.91785+00'),
(4,	NULL,	'WASM',	'WebAssembly',	0,	'/1',	'2023-07-23 10:29:23.91785+00',	'2023-07-23 10:29:23.91785+00'),
(5,	NULL,	'TEST',	'测试',	0,	'/1',	'2023-07-23 10:29:23.91785+00',	'2023-07-23 10:29:23.91785+00'),
(6,	NULL,	'DEVTOOL',	'开发工具',	0,	'/1',	'2023-07-23 10:29:23.91785+00',	'2023-07-23 10:29:23.91785+00'),
(7,	NULL,	'DATABASE',	'数据库',	0,	'/1',	'2023-07-23 10:29:23.91785+00',	'2023-07-23 10:29:23.91785+00'),
(8,	NULL,	'FRONTEND',	'前端',	0,	'/1',	'2023-07-23 10:29:23.91785+00',	'2023-07-23 10:29:23.91785+00'),
(9,	NULL,	'DEPLOYMENT',	'部署',	0,	'/1',	'2023-07-23 10:29:23.91785+00',	'2023-07-23 10:29:23.91785+00'),
(10,	NULL,	'OPEN_SOURCE',	'开源项目',	0,	'/1',	'2023-07-23 10:29:23.91785+00',	'2023-07-23 10:29:23.91785+00'),
(11,	NULL,	'OS',	'操作系统',	0,	'/1',	'2023-07-23 10:29:23.91785+00',	'2023-07-23 10:29:23.91785+00'),
(12,	NULL,	'TRANSLATION',	'翻译',	0,	'/1',	'2023-07-23 10:29:23.91785+00',	'2023-07-23 10:29:23.91785+00'),
(13,	NULL,	'BOOK',	'书籍',	0,	'/1',	'2023-07-23 10:29:23.91785+00',	'2023-07-23 10:29:23.91785+00'),
(14,	NULL,	'Operation',	'运维',	0,	'/1',	'2023-07-23 10:29:23.91785+00',	'2023-07-23 10:29:23.91785+00'),
(15,	NULL,	'QUESTION',	'新手问题',	0,	'/1',	'2023-07-23 10:29:23.91785+00',	'2023-07-23 10:29:23.91785+00'),
(16,	NULL,	'PRODUCTION_POPULARIZATION',	'产品推广',	0,	'/1',	'2023-07-23 10:29:23.91785+00',	'2023-07-23 10:29:23.91785+00'),
(17,	NULL,	'JUST_SAY_IT',	'随便说说',	0,	'/1',	'2023-07-23 10:29:23.91785+00',	'2023-07-23 10:29:23.91785+00'),
(18,	NULL,	'ANNOUNCEMENT',	'公告',	0,	'/1',	'2023-07-23 10:29:23.91785+00',	'2023-07-23 10:29:23.91785+00'),
(19,	NULL,	'OTHERS',	'其他',	0,	'/1',	'2023-07-23 10:29:23.91785+00',	'2023-07-23 10:29:23.91785+00'),
(20,	NULL,	'RECRUIT',	'招聘',	0,	NULL,	'2023-07-23 10:29:23.91785+00',	'2023-07-23 10:29:23.91785+00'),
(21,	NULL,	'WIKI',	'Wiki',	0,	NULL,	'2023-07-23 10:29:23.91785+00',	'2023-07-23 10:29:23.91785+00'),
(22,	NULL,	'NO_POINT',	'NoPoint',	0,	NULL,	'2023-07-23 10:29:23.91785+00',	'2023-07-23 10:29:23.91785+00');
