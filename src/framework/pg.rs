#![allow(unused)]

use diesel::connection::Instrumentation;
use diesel::query_builder::{AsQuery, QueryFragment, QueryId};
use diesel::{ConnectionResult, QueryResult, debug_query};
use diesel_async::pooled_connection::PoolableConnection;
use diesel_async::{
    AnsiTransactionManager, AsyncConnection, AsyncPgConnection, CacheSize, SimpleAsyncConnection,
};
use std::fmt::Display;
use std::time::{Duration, Instant};

pub struct PgConn(AsyncPgConnection);

impl SimpleAsyncConnection for PgConn {
    fn batch_execute(&mut self, query: &str) -> impl Future<Output = QueryResult<()>> + Send {
        self.0.batch_execute(query)
    }
}

impl AsyncConnection for PgConn {
    type ExecuteFuture<'conn, 'query> =
        <AsyncPgConnection as AsyncConnection>::ExecuteFuture<'conn, 'query>;
    type LoadFuture<'conn, 'query> =
        <AsyncPgConnection as AsyncConnection>::LoadFuture<'conn, 'query>;
    type Stream<'conn, 'query> = <AsyncPgConnection as AsyncConnection>::Stream<'conn, 'query>;
    type Row<'conn, 'query> = <AsyncPgConnection as AsyncConnection>::Row<'conn, 'query>;
    type Backend = <AsyncPgConnection as AsyncConnection>::Backend;
    type TransactionManager = <AsyncPgConnection as AsyncConnection>::TransactionManager;

    async fn establish(database_url: &str) -> ConnectionResult<Self> {
        Ok(PgConn(AsyncPgConnection::establish(database_url).await?))
    }

    fn load<'conn, 'query, T>(&'conn mut self, source: T) -> Self::LoadFuture<'conn, 'query>
    where
        T: AsQuery + 'query,
        T::Query: QueryFragment<Self::Backend> + QueryId,
    {
        // async move {
        //     let source = source.as_query();
        //     let debug_string = debug_query::<super::db::DbType, _>(&source);
        //     let start_time = Instant::now();
        //     let result = self.0.load(source).await;
        //     let duration = start_time.elapsed();
        //     if let Err(e) = &result {
        //         log_error_query(&debug_string, duration, e);
        //     } else {
        //         log_query(&debug_string, duration)
        //     }
        //     result
        // }
        // .boxed()
        self.0.load(source)
    }

    fn execute_returning_count<'conn, 'query, T>(
        &'conn mut self,
        source: T,
    ) -> Self::ExecuteFuture<'conn, 'query>
    where
        T: QueryFragment<Self::Backend> + QueryId + 'query,
    {
        let pin = self.0.execute_returning_count(source);
        pin
    }

    fn transaction_state(&mut self) -> &mut AnsiTransactionManager {
        self.0.transaction_state()
    }

    fn instrumentation(&mut self) -> &mut dyn Instrumentation {
        self.0.instrumentation()
    }

    fn set_instrumentation(&mut self, instrumentation: impl Instrumentation) {
        self.0.set_instrumentation(instrumentation)
    }

    fn set_prepared_statement_cache_size(&mut self, size: CacheSize) {
        self.0.set_prepared_statement_cache_size(size)
    }
}

impl PoolableConnection for PgConn {
    fn is_broken(&mut self) -> bool {
        self.0.is_broken()
    }
}

#[allow(unused)]
impl PgConn {
    fn inner<'conn, 'query, T>(
        &'conn mut self,
        source: Box<T>,
    ) -> <AsyncPgConnection as AsyncConnection>::LoadFuture<'conn, 'query>
    where
        T: AsQuery + 'query,
        T::Query: QueryFragment<super::db::DbType> + QueryId,
    {
        let source = source.as_query();
        let debug_string = debug_query::<super::db::DbType, _>(&source);
        let start_time = Instant::now();
        let result = self.0.load(source);
        let duration = start_time.elapsed();
        todo!()
        // if let Err(e) = &result {
        //     log_error_query(&debug_string, duration, e);
        // } else {
        //     log_query(&debug_string, duration)
        // }
        // result.unwrap().boxed()
    }
}

//
// impl Drop for PgConn {
//     fn drop(&mut self) {
//         self.0.drop()
//     }
// }
mod inner {
    use crate::framework::pg::PgConn;
    use diesel::query_builder::IntoUpdateTarget;
    use diesel::{AsChangeset, QueryResult};
    use diesel_async::{RunQueryDsl, UpdateAndFetchResults, methods};
    use futures::FutureExt;
    use futures::future::BoxFuture;

    impl<'b, Changes, Output, Tab, V> UpdateAndFetchResults<Changes, Output> for PgConn
    where
        Output: Send + 'static,
        Changes: Copy
            + AsChangeset<Target = Tab>
            + Send
            + diesel::associations::Identifiable<Table = Tab>,
        Tab: diesel::Table + diesel::query_dsl::methods::FindDsl<Changes::Id> + 'b,
        diesel::dsl::Find<Tab, Changes::Id>: IntoUpdateTarget<Table = Tab, WhereClause = V>,
        diesel::query_builder::UpdateStatement<Tab, V, Changes::Changeset>:
            diesel::query_builder::AsQuery,
        diesel::dsl::Update<Changes, Changes>: methods::LoadQuery<'b, Self, Output>,
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
            async move {
                diesel::update(changeset)
                    .set(changeset)
                    .get_result(self)
                    .await
            }
            .boxed()
        }
    }
}
fn log_error_query(query: &dyn Display, duration: Duration, e: &diesel::result::Error) {
    tracing::error!(
        "Error QUERY, Error: {e} , [{:.2} s]: {}",
        duration_to_secs(duration),
        query
    );
}
fn duration_to_secs(duration: Duration) -> f32 {
    duration_to_ms(duration) / MILLIS_PER_SEC as f32
}
fn log_query(query: &dyn Display, duration: Duration) {
    if duration.as_secs() >= 5 {
        tracing::warn!(
            "SLOW QUERY [{:.2} s]: {}",
            duration_to_secs(duration),
            query
        );
    } else if duration.as_secs() >= 1 {
        tracing::info!(
            "SLOW QUERY [{:.2} s]: {}",
            duration_to_secs(duration),
            query
        );
    } else {
        tracing::debug!("QUERY: [{:.1}ms]: {}", duration_to_ms(duration), query);
    }
}
const NANOS_PER_MILLI: u32 = 1_000_000;
const MILLIS_PER_SEC: u32 = 1_000;
fn duration_to_ms(duration: Duration) -> f32 {
    (duration.as_secs() as u32 * 1000) as f32
        + (duration.subsec_nanos() as f32 / NANOS_PER_MILLI as f32)
}
