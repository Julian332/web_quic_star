diesel::table! {
    user_with_group_views (id) {
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
        group_name -> Nullable<Text>,
        permissions -> Array<Text>,
    }
}
