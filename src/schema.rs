// @generated automatically by Diesel CLI.

diesel::table! {
    groups (id) {
        id -> Int8,
        name -> Text,
        remark -> Nullable<Text>,
        update_time -> Nullable<Timestamptz>,
        create_time -> Timestamptz,
        create_by -> Int8,
        update_by -> Nullable<Int8>,
        is_delete -> Bool,
        permissions -> Array<Text>,
    }
}

diesel::table! {
    groups_permissions (group_id, permission_id) {
        group_id -> Int8,
        permission_id -> Int8,
    }
}

diesel::table! {
    permissions (id) {
        id -> Int8,
        name -> Text,
        remark -> Nullable<Text>,
        update_time -> Nullable<Timestamptz>,
        create_time -> Timestamptz,
        create_by -> Int8,
        update_by -> Nullable<Int8>,
        is_delete -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        username -> Text,
        password -> Text,
        group_id -> Int8,
        tenantry -> Text,
        remark -> Nullable<Text>,
        update_time -> Nullable<Timestamptz>,
        create_time -> Timestamptz,
        create_by -> Int8,
        update_by -> Nullable<Int8>,
        is_delete -> Bool,
    }
}

diesel::joinable!(users -> groups (group_id));

diesel::allow_tables_to_appear_in_same_query!(groups, users,);
