use crate::framework::db::DbType;
use diesel::connection::{CacheSize, Instrumentation};
use diesel::query_builder::{AsQuery, IntoUpdateTarget, QueryFragment, QueryId};
use diesel::{AsChangeset, ConnectionResult, QueryResult, debug_query};
use diesel_async::pooled_connection::PoolableConnection;
use diesel_async::{
    AsyncConnection, AsyncConnectionCore, AsyncPgConnection, SimpleAsyncConnection,
    TransactionManager, UpdateAndFetchResults, methods,
};
use futures::future::BoxFuture;
use std::time::Duration;
use tokio::time::Instant;
use tracing::warn;

pub struct LogPgConn(AsyncPgConnection);

const SLOW_SQL_THRESHOLD: u64 = 1;

impl AsyncConnectionCore for LogPgConn {
    type ExecuteFuture<'conn, 'query> =
        <AsyncPgConnection as AsyncConnectionCore>::ExecuteFuture<'conn, 'query>;
    type LoadFuture<'conn, 'query> =
        <AsyncPgConnection as AsyncConnectionCore>::LoadFuture<'conn, 'query>;
    type Stream<'conn, 'query> = <AsyncPgConnection as AsyncConnectionCore>::Stream<'conn, 'query>;
    type Row<'conn, 'query> = <AsyncPgConnection as AsyncConnectionCore>::Row<'conn, 'query>;
    type Backend = <AsyncPgConnection as AsyncConnectionCore>::Backend;

    fn load<'conn, 'query, T>(&'conn mut self, source: T) -> Self::LoadFuture<'conn, 'query>
    where
        T: AsQuery + 'query,
        T::Query: QueryFragment<Self::Backend> + QueryId + 'query,
    {
        let query = source.as_query();
        let sql = debug_query::<DbType, _>(&query).to_string();
        // Call load before the async block so `query` (non-Send) isn't captured;
        // the returned future is BoxedLoadRowFuture which is Send.
        let future = AsyncConnectionCore::load(&mut self.0, query);
        Box::pin(async move {
            let instant = Instant::now();
            let result = future.await;
            let duration = instant.elapsed();
            if duration >= Duration::from_secs(SLOW_SQL_THRESHOLD) {
                warn!("slow sql detected: {sql} ,duration:{duration:?}");
            }
            result
        })
    }

    fn execute_returning_count<'conn, 'query, T>(
        &'conn mut self,
        source: T,
    ) -> Self::ExecuteFuture<'conn, 'query>
    where
        T: QueryFragment<Self::Backend> + QueryId + 'query,
    {
        AsyncConnectionCore::execute_returning_count(&mut self.0, source)
    }
}

impl AsyncConnectionCore for &LogPgConn {
    type ExecuteFuture<'conn, 'query> =
        <AsyncPgConnection as AsyncConnectionCore>::ExecuteFuture<'conn, 'query>;
    type LoadFuture<'conn, 'query> =
        <AsyncPgConnection as AsyncConnectionCore>::LoadFuture<'conn, 'query>;
    type Stream<'conn, 'query> = <AsyncPgConnection as AsyncConnectionCore>::Stream<'conn, 'query>;
    type Row<'conn, 'query> = <AsyncPgConnection as AsyncConnectionCore>::Row<'conn, 'query>;
    type Backend = <AsyncPgConnection as AsyncConnectionCore>::Backend;

    fn load<'conn, 'query, T>(&'conn mut self, source: T) -> Self::LoadFuture<'conn, 'query>
    where
        T: AsQuery + 'query,
        T::Query: QueryFragment<Self::Backend> + QueryId + 'query,
    {
        let query = source.as_query();
        let sql = debug_query::<DbType, _>(&query).to_string();
        // Call load before the async block so `query` (non-Send) isn't captured;
        // the returned future is BoxedLoadRowFuture which is Send.
        let future = AsyncConnectionCore::load(&mut &self.0, query);
        Box::pin(async move {
            let instant = Instant::now();
            let result = future.await;
            let duration = instant.elapsed();
            if duration >= Duration::from_secs(SLOW_SQL_THRESHOLD) {
                warn!("slow sql detected:{sql},duration:{duration:?}");
            }
            result
        })
    }

    fn execute_returning_count<'conn, 'query, T>(
        &'conn mut self,
        source: T,
    ) -> Self::ExecuteFuture<'conn, 'query>
    where
        T: QueryFragment<Self::Backend> + QueryId + 'query,
    {
        AsyncConnectionCore::execute_returning_count(&mut &self.0, source)
    }
}

impl SimpleAsyncConnection for LogPgConn {
    fn batch_execute(&mut self, query: &str) -> impl Future<Output = QueryResult<()>> + Send {
        SimpleAsyncConnection::batch_execute(&mut self.0, query)
    }
}

impl SimpleAsyncConnection for &LogPgConn {
    fn batch_execute(&mut self, query: &str) -> impl Future<Output = QueryResult<()>> + Send {
        let conn: &AsyncPgConnection = &self.0;
        Box::pin(async move {
            let mut conn = conn;
            SimpleAsyncConnection::batch_execute(&mut conn, query).await
        })
    }
}

impl AsyncConnection for LogPgConn {
    type TransactionManager = <AsyncPgConnection as AsyncConnection>::TransactionManager;

    fn establish(database_url: &str) -> impl Future<Output = ConnectionResult<Self>> + Send {
        Box::pin(async move {
            let result = <AsyncPgConnection as AsyncConnection>::establish(database_url).await;
            result.map(LogPgConn)
        })
    }

    fn transaction_state(
        &mut self,
    ) -> &mut <Self::TransactionManager as TransactionManager<Self>>::TransactionStateData {
        AsyncConnection::transaction_state(&mut self.0)
    }

    fn instrumentation(&mut self) -> &mut dyn Instrumentation {
        AsyncConnection::instrumentation(&mut self.0)
    }

    fn set_instrumentation(&mut self, instrumentation: impl Instrumentation) {
        AsyncConnection::set_instrumentation(&mut self.0, instrumentation)
    }

    fn set_prepared_statement_cache_size(&mut self, size: CacheSize) {
        AsyncConnection::set_prepared_statement_cache_size(&mut self.0, size)
    }
}

impl<'b, Changes, Output, Tab, V> UpdateAndFetchResults<Changes, Output> for LogPgConn
where
    Output: Send + 'static,
    Changes:
        Copy + AsChangeset<Target = Tab> + Send + diesel::associations::Identifiable<Table = Tab>,
    Tab: diesel::Table + diesel::query_dsl::methods::FindDsl<Changes::Id> + 'b,
    diesel::dsl::Find<Tab, Changes::Id>: IntoUpdateTarget<Table = Tab, WhereClause = V>,
    diesel::query_builder::UpdateStatement<Tab, V, Changes::Changeset>: AsQuery,
    diesel::dsl::Update<Changes, Changes>: methods::LoadQuery<'b, AsyncPgConnection, Output>,
    V: Send + 'b,
    Changes::Changeset: Send + 'b,
    Tab::FromClause: Send,
{
    fn update_and_fetch<'conn, 'changes>(
        &'conn mut self,
        changeset: Changes,
    ) -> BoxFuture<'changes, QueryResult<Output>>
    where
        Changes: 'changes,
        Changes::Changeset: 'changes,
        'conn: 'changes,
        Self: 'changes,
    {
        UpdateAndFetchResults::update_and_fetch(&mut self.0, changeset)
    }
}
impl PoolableConnection for LogPgConn {
    fn is_broken(&mut self) -> bool {
        PoolableConnection::is_broken(&mut self.0)
    }
}
