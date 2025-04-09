-- Your SQL goes here




CREATE TABLE "groups"
(
    "id"          SERIAL8                     NOT NULL PRIMARY KEY,
    "name"        TEXT                        NOT NULL,
    "remark"      TEXT,
    "update_time" TIMESTAMPTZ,
    "create_time" TIMESTAMPTZ                 NOT NULL,
    "create_by"   INT8                        NOT NULL,
    "update_by"   INT8,
    "is_delete"   BOOL                        NOT NULL,
    "permissions" TEXT[] default '{}'::TEXT[] not null


);

CREATE TABLE "users"
(
    "id"          SERIAL8     NOT NULL PRIMARY KEY,
    "username"    TEXT        NOT NULL,
    "password"    TEXT        NOT NULL,
    "group_id"    INT8        NOT NULL,
    "tenantry"    TEXT        NOT NULL,
    "remark"      TEXT,
    "update_time" TIMESTAMPTZ,
    "create_time" TIMESTAMPTZ NOT NULL,
    "create_by"   INT8        NOT NULL,
    "update_by"   INT8,
    "is_delete"   BOOL        NOT NULL,
    FOREIGN KEY ("group_id") REFERENCES "groups" ("id")
);
alter table users
    add constraint uni_name
        unique (username);

comment on column users.password is 'password  hash or signature hash';




CREATE TYPE order_type AS ENUM ('trading', 'pending', 'following');
CREATE TYPE sell_buy AS ENUM ('sell', 'buy');



INSERT INTO groups (id, name, remark, update_time, create_time, create_by, update_by, is_delete)
VALUES (-1, 'common_user', null, null, now(), -2, null, false);

INSERT INTO groups (id, name, remark, update_time, create_time, create_by, update_by, is_delete,permissions)
VALUES (-2, 'super_admin', null, null, now(), -2, null, false,array ['Admin']);






INSERT INTO users (id, username, password, group_id, tenantry, remark, update_time, create_time, create_by, update_by,
                   is_delete)
VALUES (-1, 'common_user',
        '$argon2id$v=19$m=19456,t=2,p=1$pHJK4Msog1E+V7R4++t+Zg$QnzTOC3JNu50cn0fJcdO5P33WnUUeQRK3oa9M054nko', -1,
        'default', null, null, now(), -2, null, false);

INSERT INTO users (id, username, password, group_id, tenantry, remark, update_time, create_time, create_by, update_by,
                   is_delete)
VALUES (-2, 'super_admin',
        '$argon2id$v=19$m=19456,t=2,p=1$pHJK4Msog1E+V7R4++t+Zg$QnzTOC3JNu50cn0fJcdO5P33WnUUeQRK3oa9M054nko', -2,
        'default', null, null, now(), -2, null, false);

CREATE view user_with_group_views
            (id, username, password, group_id, tenantry, remark, update_time, create_time, create_by, update_by,
             is_delete, group_name)
as
SELECT users.id,
       users.username,
       users.password,
       users.group_id,
       users.tenantry,
       users.remark,
       users.update_time,
       users.create_time,
       users.create_by,
       users.update_by,
       users.is_delete,
       groups.name AS group_name
FROM users
         LEFT JOIN groups ON users.group_id = groups.id;

alter table user_with_group_views
    owner to postgres;

-- auto cast
-- CREATE CAST (text AS bigint) WITH INOUT AS IMPLICIT;
-- CREATE CAST (text AS int4) WITH INOUT AS IMPLICIT;
-- CREATE CAST (text AS numeric) WITH INOUT AS IMPLICIT;
--
-- CREATE CAST (bigint AS text) WITH INOUT AS IMPLICIT;
-- CREATE CAST (int4 AS text) WITH INOUT AS IMPLICIT;
-- CREATE CAST (numeric AS text) WITH INOUT AS IMPLICIT;