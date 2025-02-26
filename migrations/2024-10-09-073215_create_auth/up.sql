-- Your SQL goes here


CREATE TABLE "permissions"
(
    "id"          SERIAL8     NOT NULL PRIMARY KEY,
    "name"        TEXT        NOT NULL,
    "remark"      TEXT,
    "update_time" TIMESTAMPTZ,
    "create_time" TIMESTAMPTZ NOT NULL,
    "create_by"   INT8        NOT NULL,
    "update_by"   INT8,
    "is_delete"   BOOL        NOT NULL
);

CREATE TABLE "groups"
(
    "id"          SERIAL8     NOT NULL PRIMARY KEY,
    "name"        TEXT        NOT NULL,
    "remark"      TEXT,
    "update_time" TIMESTAMPTZ,
    "create_time" TIMESTAMPTZ NOT NULL,
    "create_by"   INT8        NOT NULL,
    "update_by"   INT8,
    "is_delete"   BOOL        NOT NULL
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

CREATE TABLE "groups_permissions"
(
    "group_id"      INT8 NOT NULL,
    "permission_id" INT8 NOT NULL,
    PRIMARY KEY ("group_id", "permission_id")
);

alter table groups_permissions
    add constraint groups_permissions_groups_id_fk
        foreign key (group_id) references groups;

alter table groups_permissions
    add constraint groups_permissions_permissions_id_fk
        foreign key (permission_id) references permissions;

CREATE TYPE order_type AS ENUM ('trading', 'pending', 'following');
CREATE TYPE sell_buy AS ENUM ('sell', 'buy');



INSERT INTO groups (id, name, remark, update_time, create_time, create_by, update_by, is_delete)
VALUES (-1, 'common_user', null, null, now(), -2, null, false);

INSERT INTO groups (id, name, remark, update_time, create_time, create_by, update_by, is_delete)
VALUES (-2, 'super_admin', null, null, now(), -2, null, false);



INSERT INTO permissions (id, name, remark, update_time, create_time, create_by, update_by, is_delete)
VALUES (-1, 'common_read', null, null, now(), -2, null, false);

INSERT INTO permissions (id, name, remark, update_time, create_time, create_by, update_by, is_delete)
VALUES (-2, 'common_add', null, null, now(), -2, null, false);

INSERT INTO permissions (id, name, remark, update_time, create_time, create_by, update_by, is_delete)
VALUES (-3, 'common_delete', null, null, now(), -2, null, false);

INSERT INTO permissions (id, name, remark, update_time, create_time, create_by, update_by, is_delete)
VALUES (-4, 'common_update', null, null, now(), -2, null, false);



INSERT INTO permissions (id, name, remark, update_time, create_time, create_by, update_by, is_delete)
VALUES (-5, 'users_read', null, null, now(), -2, null, false);

INSERT INTO permissions (id, name, remark, update_time, create_time, create_by, update_by, is_delete)
VALUES (-6, 'users_add', null, null, now(), -2, null, false);

INSERT INTO permissions (id, name, remark, update_time, create_time, create_by, update_by, is_delete)
VALUES (-7, 'users_delete', null, null, now(), -2, null, false);

INSERT INTO permissions (id, name, remark, update_time, create_time, create_by, update_by, is_delete)
VALUES (-8, 'users_update', null, null, now(), -2, null, false);



INSERT INTO groups_permissions (group_id, permission_id)
VALUES (-1, -1);
INSERT INTO groups_permissions (group_id, permission_id)
VALUES (-2, -1);
INSERT INTO groups_permissions (group_id, permission_id)
VALUES (-2, -2);
INSERT INTO groups_permissions (group_id, permission_id)
VALUES (-2, -3);
INSERT INTO groups_permissions (group_id, permission_id)
VALUES (-2, -4);
INSERT INTO groups_permissions (group_id, permission_id)
VALUES (-2, -5);
INSERT INTO groups_permissions (group_id, permission_id)
VALUES (-2, -6);
INSERT INTO groups_permissions (group_id, permission_id)
VALUES (-2, -7);
INSERT INTO groups_permissions (group_id, permission_id)
VALUES (-2, -8);



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
CREATE CAST (text AS bigint) WITH INOUT AS IMPLICIT;
CREATE CAST (text AS int4) WITH INOUT AS IMPLICIT;
CREATE CAST (text AS numeric) WITH INOUT AS IMPLICIT;

CREATE CAST (bigint AS text) WITH INOUT AS IMPLICIT;
CREATE CAST (int4 AS text) WITH INOUT AS IMPLICIT;
CREATE CAST (numeric AS text) WITH INOUT AS IMPLICIT;