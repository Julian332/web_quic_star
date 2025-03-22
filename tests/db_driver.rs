use r2d2_postgres::postgres::NoTls;
use r2d2_postgres::PostgresConnectionManager;
use web_quick::db_models::group_permission::GroupsPermission;

#[test]
fn t() {
    let manager = PostgresConnectionManager::new(
        "postgres://postgres:1234qwer@192.168.8.63:5432/web_quic_star"
            .parse()
            .unwrap(),
        NoTls,
    );
    let pool = r2d2::Pool::new(manager).unwrap();
    let new = GroupsPermission {
        group_id: 100,
        permission_id: 100,
    };
    let mut client = pool.get().unwrap();

    // diesel::insert_into(groups_permissions)
    //     .values(new)
    //     .execute(&mut client);
    // for i in 0..10i32 {
    //     let pool = pool.clone();
    //     thread::spawn(move || {
    //         client
    //             .execute("INSERT INTO groups_permissions  VALUES (9,9)", &[])
    //             .unwrap();
    //     });
    // }
}
