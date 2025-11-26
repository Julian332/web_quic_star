-- Your SQL goes here


CREATE TABLE "req_records"(
	"id" SERIAL8 NOT NULL PRIMARY KEY,
	"username" TEXT,
	"req_id" TEXT NOT NULL,
	"req_body" TEXT,
	"path" TEXT NOT NULL,
	"status_code" TEXT NOT NULL,
	"update_time" TIMESTAMPTZ,
	"create_time" TIMESTAMPTZ NOT NULL,
	"create_by" INT8 NOT NULL,
	"update_by" INT8,
	"is_delete" BOOL NOT NULL default false
);

