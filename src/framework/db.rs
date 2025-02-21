use crate::db_models::{Conn, ConnPool, DbType};
use diesel::query_builder::{AstPass, Query, QueryFragment};
use diesel::query_dsl::LoadQuery;
use diesel::r2d2::ConnectionManager;
use diesel::sql_types::BigInt;
use diesel::{QueryId, QueryResult, QueryableByName, RunQueryDsl};
use r2d2::Pool;
use std::env;
#[derive(QueryableByName)]
pub struct Count {
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub count: i64,
}

pub fn setup_connection_pool() -> ConnPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<Conn>::new(database_url);
    // Refer to the `r2d2` documentation for more methods to use
    // when building a connection pool
    Pool::builder()
        .max_size(10)
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}

pub trait Paginate: Sized {
    fn paginate(self, page_no: i64, page_size: i64) -> Paginated<Self>;
}

impl<T> Paginate for T {
    fn paginate(self, page_no: i64, page_size: i64) -> Paginated<Self> {
        Paginated {
            query: self,
            per_page: page_size,
            page: page_no,
            offset: (page_no - 1) * page_size,
        }
    }
}

#[allow(unused)]
#[derive(Debug, Clone, Copy, QueryId)]
pub struct Paginated<T> {
    query: T,
    page: i64,
    per_page: i64,
    offset: i64,
}

impl<T> Paginated<T> {
    pub fn load_and_count_pages<'a, U>(self, conn: &mut Conn) -> QueryResult<(Vec<U>, i64)>
    where
        Self: LoadQuery<'a, Conn, (U, i64)>,
    {
        // let per_page = self.per_page;
        let results = self.load::<(U, i64)>(conn)?;
        let total = results.first().map(|x| x.1).unwrap_or(0);
        let records = results.into_iter().map(|x| x.0).collect();
        // let total_pages = (total as f64 / per_page as f64).ceil() as i64;
        Ok((records, total))
    }
}

impl<T: Query> Query for Paginated<T> {
    type SqlType = (T::SqlType, BigInt);
}

impl<T> RunQueryDsl<Conn> for Paginated<T> {}

impl<T> QueryFragment<DbType> for Paginated<T>
where
    T: QueryFragment<DbType>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DbType>) -> QueryResult<()> {
        out.push_sql("SELECT *, COUNT(1) OVER () FROM (");
        self.query.walk_ast(out.reborrow())?;
        out.push_sql(") t  LIMIT ");
        out.push_bind_param::<BigInt, _>(&self.per_page)?;
        out.push_sql(" OFFSET ");
        out.push_bind_param::<BigInt, _>(&self.offset)?;
        Ok(())
    }
}

pub trait LogicDeleteQuery: Sized {
    fn logic_delete_query(self) -> LogicDeleteStatement<Self>;
}

#[derive(Debug, Clone, Copy, QueryId)]
pub struct LogicDeleteStatement<T> {
    query: T,
}

impl<T> LogicDeleteQuery for T {
    fn logic_delete_query(self) -> LogicDeleteStatement<Self> {
        LogicDeleteStatement { query: self }
    }
}

impl<T: Query> Query for LogicDeleteStatement<T> {
    type SqlType = T::SqlType;
}

impl<T> RunQueryDsl<Conn> for LogicDeleteStatement<T> {}

impl<T> QueryFragment<DbType> for LogicDeleteStatement<T>
where
    T: QueryFragment<DbType>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DbType>) -> QueryResult<()> {
        out.push_sql("SELECT * FROM (");
        self.query.walk_ast(out.reborrow())?;
        out.push_sql(") dt  where dt.is_delete = false ");

        Ok(())
    }
}

#[tokio::test]
async fn test() {
    use crate::db_models::user::User;
    use crate::schema::users::table as users;
    use crate::set_env;
    use diesel::QueryDsl;
    use diesel::SelectableHelper;

    set_env();
    let connection_pool = setup_connection_pool();
    let mut pooled_connection = connection_pool.get().unwrap();
    let x = users
        .select(User::as_select())
        .logic_delete_query()
        .paginate(0, 10)
        .load_and_count_pages(&mut pooled_connection)
        .unwrap();

    println!("{:?}", x);
}

pub fn update_by_id() {}
