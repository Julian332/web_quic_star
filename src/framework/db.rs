use crate::{CONFIG, DB};
use deadpool::managed::Object;
use diesel::query_builder::{AstPass, Query, QueryFragment, QueryId};
use diesel::sql_types::BigInt;
use diesel::{QueryResult, QueryableByName};
use diesel_async::RunQueryDsl;
use diesel_async::async_connection_wrapper::AsyncConnectionWrapper;
use diesel_async::methods::LoadQuery;
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::{AsyncDieselConnectionManager, ManagerConfig};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use tracing::info;

#[derive(QueryableByName)]
pub struct Count {
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub count: i64,
}

#[allow(clippy::expect_used)]
pub fn setup_connection_pool() -> ConnPool {
    let database_url = CONFIG.database_url.to_string();
    let manager = AsyncDieselConnectionManager::<Conn>::new_with_config(
        database_url,
        ManagerConfig::default(),
    );

    Pool::builder(manager)
        .max_size(10)
        .build()
        .expect("Could not build db connection pool")
}

pub trait Paginate: Sized {
    fn paginate(self, page_no: i64, page_size: i64) -> Paginated<Self>;
}

impl<T> Paginate for T
where
    T: QueryFragment<DbType>,
{
    fn paginate(self, page_no: i64, page_size: i64) -> Paginated<Self> {
        Paginated {
            query: self,
            per_page: page_size,
            offset: (page_no - 1) * page_size,
        }
    }
}

#[allow(unused)]
#[derive(Debug, Clone, Copy, QueryId)]
pub struct Paginated<T> {
    query: T,
    per_page: i64,
    offset: i64,
}

impl<T: Query> Paginated<T> {
    pub fn load_and_count_pages<'a, U>(
        self,
        conn: &'a mut Object<AsyncDieselConnectionManager<Conn>>,
    ) -> impl Future<Output = QueryResult<(Vec<U>, i64)>> + Send + 'a
    where
        Self: LoadQuery<'a, Object<AsyncDieselConnectionManager<Conn>>, (U, i64)>,
        U: Send + 'a,
        T: 'a,
    {
        // Ignore those linting errors. `get(0)` cannot be replaced with `first()`.
        #![allow(clippy::get_first)]
        let results = self.load::<(U, i64)>(conn);

        async move {
            let results = results.await?;
            let total = results.get(0).map(|x| x.1).unwrap_or(0);
            let records = results.into_iter().map(|x| x.0).collect();
            Ok((records, total))
        }
    }
}

impl<T: Query> Query for Paginated<T> {
    type SqlType = (T::SqlType, BigInt);
}

// impl<T, C: Connection> RunQueryDsl<C> for Paginated<T> {}

#[cfg(feature = "postgres")]
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

impl<T> LogicDeleteQuery for T
where
    T: QueryFragment<DbType>,
{
    fn logic_delete_query(self) -> LogicDeleteStatement<Self> {
        LogicDeleteStatement { query: self }
    }
}

impl<T: Query> Query for LogicDeleteStatement<T> {
    type SqlType = T::SqlType;
}

// impl<T, C: Connection> RunQueryDsl<C> for LogicDeleteStatement<T> {}

#[cfg(feature = "postgres")]
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

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[allow(clippy::unwrap_used)]
pub async fn sync_db_schema() {
    let async_connection = DB.get().await.unwrap();
    let mut async_wrapper: AsyncConnectionWrapper<_> =
        AsyncConnectionWrapper::from(async_connection);

    tokio::task::spawn_blocking(move || {
        let vec = async_wrapper.run_pending_migrations(MIGRATIONS).unwrap();
        info!("db schema update succeed: {vec:?}",);
    })
    .await
    .unwrap();
}

#[cfg(feature = "postgres")]
pub type DbType = diesel::pg::Pg;
pub type ConnPool = diesel_async::pooled_connection::deadpool::Pool<Conn>;
#[cfg(feature = "postgres")]
pub type Conn = diesel_async::AsyncPgConnection;
