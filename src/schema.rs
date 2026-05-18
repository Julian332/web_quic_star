// @generated automatically by Diesel CLI.

#[allow(unused_imports, dead_code, unreachable_pub, unused_qualifications)]
pub mod groups {
    pub use self::columns::*;
    use ::diesel;
    use diesel::sql_types::*;
    #[doc = r" Re-exports all of the columns of this table, as well as the"]
    #[doc = r" table struct renamed to the module name. This is meant to be"]
    #[doc = r" glob imported for functions which only deal with one table."]
    pub mod dsl {
        pub use super::columns::create_by;
        pub use super::columns::create_time;
        pub use super::columns::id;
        pub use super::columns::is_delete;
        pub use super::columns::name;
        pub use super::columns::permissions;
        pub use super::columns::remark;
        pub use super::columns::update_by;
        pub use super::columns::update_time;
        pub use super::table as groups;
    }
    #[allow(non_upper_case_globals, dead_code)]
    #[doc = r" A tuple of all of the columns on this table"]
    pub const all_columns: (
        id,
        name,
        remark,
        update_time,
        create_time,
        create_by,
        update_by,
        is_delete,
        permissions,
    ) = (
        id,
        name,
        remark,
        update_time,
        create_time,
        create_by,
        update_by,
        is_delete,
        permissions,
    );
    #[allow(non_camel_case_types)]
    #[derive(Debug, Clone, Copy, diesel::query_builder::QueryId, Default)]
    #[doc = r" The actual table struct"]
    #[doc = r""]
    #[doc = r" This is the type which provides the base methods of the query"]
    #[doc = r" builder, such as `.select` and `.filter`."]
    pub struct table;
    impl table {
        #[allow(dead_code)]
        #[doc = r" Represents `table_name.*`, which is sometimes necessary"]
        #[doc = r" for efficient count queries. It cannot be used in place of"]
        #[doc = r" `all_columns`"]
        pub fn star(&self) -> star {
            star
        }
    }
    #[doc = r" The SQL type of all of the columns on this table"]
    pub type SqlType = (
        Int8,
        Text,
        Nullable<Text>,
        Nullable<Timestamptz>,
        Timestamptz,
        Int8,
        Nullable<Int8>,
        Bool,
        Array<Text>,
    );
    #[doc = r" Helper type for representing a boxed query from this table"]
    pub type BoxedQuery<'a, DB, ST = SqlType> = diesel::internal::table_macro::BoxedSelectStatement<
        'a,
        ST,
        diesel::internal::table_macro::FromClause<table>,
        DB,
    >;
    impl diesel::QuerySource for table {
        type FromClause = diesel::internal::table_macro::StaticQueryFragmentInstance<table>;
        type DefaultSelection = <Self as diesel::Table>::AllColumns;
        fn from_clause(&self) -> Self::FromClause {
            diesel::internal::table_macro::StaticQueryFragmentInstance::new()
        }
        fn default_selection(&self) -> Self::DefaultSelection {
            use diesel::Table;
            Self::all_columns()
        }
    }
    impl<DB> diesel::query_builder::QueryFragment<DB> for table
    where
        DB: diesel::backend::Backend,
        <table as diesel::internal::table_macro::StaticQueryFragment>::Component:
            diesel::query_builder::QueryFragment<DB>,
    {
        fn walk_ast<'b>(
            &'b self,
            __diesel_internal_pass: diesel::query_builder::AstPass<'_, 'b, DB>,
        ) -> diesel::result::QueryResult<()> {
            <table as diesel::internal::table_macro::StaticQueryFragment>::STATIC_COMPONENT
                .walk_ast(__diesel_internal_pass)
        }
    }
    impl diesel::internal::table_macro::StaticQueryFragment for table {
        type Component = diesel::internal::table_macro::Identifier<'static>;
        const STATIC_COMPONENT: &'static Self::Component =
            &diesel::internal::table_macro::Identifier("groups");
    }
    impl diesel::query_builder::AsQuery for table {
        type SqlType = SqlType;
        type Query = diesel::internal::table_macro::SelectStatement<
            diesel::internal::table_macro::FromClause<Self>,
        >;
        fn as_query(self) -> Self::Query {
            diesel::internal::table_macro::SelectStatement::simple(self)
        }
    }
    impl diesel::Table for table {
        type PrimaryKey = id;
        type AllColumns = (
            id,
            name,
            remark,
            update_time,
            create_time,
            create_by,
            update_by,
            is_delete,
            permissions,
        );
        fn primary_key(&self) -> Self::PrimaryKey {
            id
        }
        fn all_columns() -> Self::AllColumns {
            (
                id,
                name,
                remark,
                update_time,
                create_time,
                create_by,
                update_by,
                is_delete,
                permissions,
            )
        }
    }
    impl diesel::associations::HasTable for table {
        type Table = Self;
        fn table() -> Self::Table {
            table
        }
    }
    impl diesel::query_builder::IntoUpdateTarget for table {
        type WhereClause = <<Self as diesel::query_builder::AsQuery>::Query as diesel::query_builder::IntoUpdateTarget>::WhereClause;
        fn into_update_target(
            self,
        ) -> diesel::query_builder::UpdateTarget<Self::Table, Self::WhereClause> {
            use diesel::query_builder::AsQuery;
            let q: diesel::internal::table_macro::SelectStatement<
                diesel::internal::table_macro::FromClause<table>,
            > = self.as_query();
            q.into_update_target()
        }
    }
    impl diesel::query_source::AppearsInFromClause<table> for table {
        type Count = diesel::query_source::Once;
    }
    impl<S> diesel::internal::table_macro::AliasAppearsInFromClause<S, table> for table
    where
        S: diesel::query_source::AliasSource<Target = table>,
    {
        type Count = diesel::query_source::Never;
    }
    impl<S1, S2> diesel::internal::table_macro::AliasAliasAppearsInFromClause<table, S2, S1> for table
    where
        S1: diesel::query_source::AliasSource<Target = table>,
        S2: diesel::query_source::AliasSource<Target = table>,
        S1: diesel::internal::table_macro::AliasAliasAppearsInFromClauseSameTable<S2, table>,
    {
        type Count =
            <S1 as diesel::internal::table_macro::AliasAliasAppearsInFromClauseSameTable<
                S2,
                table,
            >>::Count;
    }
    impl<S> diesel::query_source::AppearsInFromClause<diesel::query_source::Alias<S>> for table
    where
        S: diesel::query_source::AliasSource,
    {
        type Count = diesel::query_source::Never;
    }
    impl<S, C>
        diesel::internal::table_macro::FieldAliasMapperAssociatedTypesDisjointnessTrick<table, S, C>
        for table
    where
        S: diesel::query_source::AliasSource<Target = table> + ::std::clone::Clone,
        C: diesel::query_source::Column<Table = table>,
    {
        type Out = diesel::query_source::AliasedField<S, C>;
        fn map(
            __diesel_internal_column: C,
            __diesel_internal_alias: &diesel::query_source::Alias<S>,
        ) -> Self::Out {
            __diesel_internal_alias.field(__diesel_internal_column)
        }
    }
    impl diesel::query_source::AppearsInFromClause<table>
        for diesel::internal::table_macro::NoFromClause
    {
        type Count = diesel::query_source::Never;
    }
    impl<Left, Right, Kind> diesel::JoinTo<diesel::internal::table_macro::Join<Left, Right, Kind>>
        for table
    where
        diesel::internal::table_macro::Join<Left, Right, Kind>: diesel::JoinTo<table>,
        Left: diesel::query_source::QuerySource,
        Right: diesel::query_source::QuerySource,
    {
        type FromClause = diesel::internal::table_macro::Join<Left, Right, Kind>;
        type OnClause = <diesel::internal::table_macro::Join<Left, Right, Kind> as diesel::JoinTo<table>>::OnClause;
        fn join_target(
            __diesel_internal_rhs: diesel::internal::table_macro::Join<Left, Right, Kind>,
        ) -> (Self::FromClause, Self::OnClause) {
            let (_, __diesel_internal_on_clause) =
                diesel::internal::table_macro::Join::join_target(table);
            (__diesel_internal_rhs, __diesel_internal_on_clause)
        }
    }
    impl<Join, On> diesel::JoinTo<diesel::internal::table_macro::JoinOn<Join, On>> for table
    where
        diesel::internal::table_macro::JoinOn<Join, On>: diesel::JoinTo<table>,
    {
        type FromClause = diesel::internal::table_macro::JoinOn<Join, On>;
        type OnClause =
            <diesel::internal::table_macro::JoinOn<Join, On> as diesel::JoinTo<table>>::OnClause;
        fn join_target(
            __diesel_internal_rhs: diesel::internal::table_macro::JoinOn<Join, On>,
        ) -> (Self::FromClause, Self::OnClause) {
            let (_, __diesel_internal_on_clause) =
                diesel::internal::table_macro::JoinOn::join_target(table);
            (__diesel_internal_rhs, __diesel_internal_on_clause)
        }
    }
    impl<F, S, D, W, O, L, Of, G>
        diesel::JoinTo<
            diesel::internal::table_macro::SelectStatement<
                diesel::internal::table_macro::FromClause<F>,
                S,
                D,
                W,
                O,
                L,
                Of,
                G,
            >,
        > for table
    where
        diesel::internal::table_macro::SelectStatement<
            diesel::internal::table_macro::FromClause<F>,
            S,
            D,
            W,
            O,
            L,
            Of,
            G,
        >: diesel::JoinTo<table>,
        F: diesel::query_source::QuerySource,
    {
        type FromClause = diesel::internal::table_macro::SelectStatement<
            diesel::internal::table_macro::FromClause<F>,
            S,
            D,
            W,
            O,
            L,
            Of,
            G,
        >;
        type OnClause = <diesel::internal::table_macro::SelectStatement<
            diesel::internal::table_macro::FromClause<F>,
            S,
            D,
            W,
            O,
            L,
            Of,
            G,
        > as diesel::JoinTo<table>>::OnClause;
        fn join_target(
            __diesel_internal_rhs: diesel::internal::table_macro::SelectStatement<
                diesel::internal::table_macro::FromClause<F>,
                S,
                D,
                W,
                O,
                L,
                Of,
                G,
            >,
        ) -> (Self::FromClause, Self::OnClause) {
            let (_, __diesel_internal_on_clause) =
                diesel::internal::table_macro::SelectStatement::join_target(table);
            (__diesel_internal_rhs, __diesel_internal_on_clause)
        }
    }
    impl<'a, QS, ST, DB>
        diesel::JoinTo<
            diesel::internal::table_macro::BoxedSelectStatement<
                'a,
                diesel::internal::table_macro::FromClause<QS>,
                ST,
                DB,
            >,
        > for table
    where
        diesel::internal::table_macro::BoxedSelectStatement<
            'a,
            diesel::internal::table_macro::FromClause<QS>,
            ST,
            DB,
        >: diesel::JoinTo<table>,
        QS: diesel::query_source::QuerySource,
    {
        type FromClause = diesel::internal::table_macro::BoxedSelectStatement<
            'a,
            diesel::internal::table_macro::FromClause<QS>,
            ST,
            DB,
        >;
        type OnClause = <diesel::internal::table_macro::BoxedSelectStatement<
            'a,
            diesel::internal::table_macro::FromClause<QS>,
            ST,
            DB,
        > as diesel::JoinTo<table>>::OnClause;
        fn join_target(
            __diesel_internal_rhs: diesel::internal::table_macro::BoxedSelectStatement<
                'a,
                diesel::internal::table_macro::FromClause<QS>,
                ST,
                DB,
            >,
        ) -> (Self::FromClause, Self::OnClause) {
            let (_, __diesel_internal_on_clause) =
                diesel::internal::table_macro::BoxedSelectStatement::join_target(table);
            (__diesel_internal_rhs, __diesel_internal_on_clause)
        }
    }
    impl<S> diesel::JoinTo<diesel::query_source::Alias<S>> for table
    where
        diesel::query_source::Alias<S>: diesel::JoinTo<table>,
    {
        type FromClause = diesel::query_source::Alias<S>;
        type OnClause = <diesel::query_source::Alias<S> as diesel::JoinTo<table>>::OnClause;
        fn join_target(
            __diesel_internal_rhs: diesel::query_source::Alias<S>,
        ) -> (Self::FromClause, Self::OnClause) {
            let (_, __diesel_internal_on_clause) =
                diesel::query_source::Alias::<S>::join_target(table);
            (__diesel_internal_rhs, __diesel_internal_on_clause)
        }
    }
    impl<T> diesel::insertable::Insertable<T> for table
    where
        <table as diesel::query_builder::AsQuery>::Query: diesel::insertable::Insertable<T>,
    {
        type Values =
            <<table as diesel::query_builder::AsQuery>::Query as diesel::insertable::Insertable<
                T,
            >>::Values;
        fn values(self) -> Self::Values {
            use diesel::query_builder::AsQuery;
            self.as_query().values()
        }
    }
    impl<'a, T> diesel::insertable::Insertable<T> for &'a table
    where
        table: diesel::insertable::Insertable<T>,
    {
        type Values = <table as diesel::insertable::Insertable<T>>::Values;
        fn values(self) -> Self::Values {
            (*self).values()
        }
    }
    impl<S> diesel::JoinTo<diesel::query_builder::Only<S>> for table
    where
        diesel::query_builder::Only<S>: diesel::JoinTo<table>,
    {
        type FromClause = diesel::query_builder::Only<S>;
        type OnClause = <diesel::query_builder::Only<S> as diesel::JoinTo<table>>::OnClause;
        fn join_target(
            __diesel_internal_rhs: diesel::query_builder::Only<S>,
        ) -> (Self::FromClause, Self::OnClause) {
            let (_, __diesel_internal_on_clause) =
                diesel::query_builder::Only::<S>::join_target(table);
            (__diesel_internal_rhs, __diesel_internal_on_clause)
        }
    }
    impl diesel::query_source::AppearsInFromClause<diesel::query_builder::Only<table>> for table {
        type Count = diesel::query_source::Once;
    }
    impl diesel::query_source::AppearsInFromClause<table> for diesel::query_builder::Only<table> {
        type Count = diesel::query_source::Once;
    }
    impl<S, TSM> diesel::JoinTo<diesel::query_builder::Tablesample<S, TSM>> for table
    where
        diesel::query_builder::Tablesample<S, TSM>: diesel::JoinTo<table>,
        TSM: diesel::internal::table_macro::TablesampleMethod,
    {
        type FromClause = diesel::query_builder::Tablesample<S, TSM>;
        type OnClause =
            <diesel::query_builder::Tablesample<S, TSM> as diesel::JoinTo<table>>::OnClause;
        fn join_target(
            __diesel_internal_rhs: diesel::query_builder::Tablesample<S, TSM>,
        ) -> (Self::FromClause, Self::OnClause) {
            let (_, __diesel_internal_on_clause) =
                diesel::query_builder::Tablesample::<S, TSM>::join_target(table);
            (__diesel_internal_rhs, __diesel_internal_on_clause)
        }
    }
    impl<TSM>
        diesel::query_source::AppearsInFromClause<diesel::query_builder::Tablesample<table, TSM>>
        for table
    where
        TSM: diesel::internal::table_macro::TablesampleMethod,
    {
        type Count = diesel::query_source::Once;
    }
    impl<TSM> diesel::query_source::AppearsInFromClause<table>
        for diesel::query_builder::Tablesample<table, TSM>
    where
        TSM: diesel::internal::table_macro::TablesampleMethod,
    {
        type Count = diesel::query_source::Once;
    }
    #[doc = r" Contains all of the columns of this table"]
    pub mod columns {
        use super::table;
        use ::diesel;
        use diesel::sql_types::*;
        #[allow(non_camel_case_types, dead_code)]
        #[derive(Debug, Clone, Copy, diesel::query_builder::QueryId)]
        #[doc = r" Represents `table_name.*`, which is sometimes needed for"]
        #[doc = r" efficient count queries. It cannot be used in place of"]
        #[doc = r" `all_columns`, and has a `SqlType` of `()` to prevent it"]
        #[doc = r" being used that way"]
        pub struct star;
        impl<__GB> diesel::expression::ValidGrouping<__GB> for star
        where
            (
                id,
                name,
                remark,
                update_time,
                create_time,
                create_by,
                update_by,
                is_delete,
                permissions,
            ): diesel::expression::ValidGrouping<__GB>,
        {
            type IsAggregate = <(
                id,
                name,
                remark,
                update_time,
                create_time,
                create_by,
                update_by,
                is_delete,
                permissions,
            ) as diesel::expression::ValidGrouping<__GB>>::IsAggregate;
        }
        impl diesel::Expression for star {
            type SqlType = diesel::expression::expression_types::NotSelectable;
        }
        impl<DB: diesel::backend::Backend> diesel::query_builder::QueryFragment<DB> for star
        where
            <table as diesel::QuerySource>::FromClause: diesel::query_builder::QueryFragment<DB>,
        {
            #[allow(non_snake_case)]
            fn walk_ast<'b>(
                &'b self,
                mut __diesel_internal_out: diesel::query_builder::AstPass<'_, 'b, DB>,
            ) -> diesel::result::QueryResult<()> {
                use diesel::QuerySource;
                if !__diesel_internal_out.should_skip_from() {
                    const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                    FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                    __diesel_internal_out.push_sql(".");
                }
                __diesel_internal_out.push_sql("*");
                Ok(())
            }
        }
        impl diesel::SelectableExpression<table> for star {}
        impl diesel::AppearsOnTable<table> for star {}
        #[allow(non_camel_case_types, dead_code)]
        #[derive(Debug, Clone, Copy, diesel::query_builder::QueryId, Default)]
        pub struct id;
        impl diesel::expression::Expression for id {
            type SqlType = Int8;
        }
        impl<DB> diesel::query_builder::QueryFragment<DB> for id
        where
            DB: diesel::backend::Backend,
            diesel::internal::table_macro::StaticQueryFragmentInstance<table>:
                diesel::query_builder::QueryFragment<DB>,
        {
            #[allow(non_snake_case)]
            fn walk_ast<'b>(
                &'b self,
                mut __diesel_internal_out: diesel::query_builder::AstPass<'_, 'b, DB>,
            ) -> diesel::result::QueryResult<()> {
                if !__diesel_internal_out.should_skip_from() {
                    const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                    FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                    __diesel_internal_out.push_sql(".");
                }
                __diesel_internal_out.push_identifier("id")
            }
        }
        impl diesel::SelectableExpression<super::table> for id {}
        impl<QS> diesel::AppearsOnTable<QS> for id where
            QS: diesel::query_source::AppearsInFromClause<
                    super::table,
                    Count = diesel::query_source::Once,
                >
        {
        }
        impl<Left, Right>
            diesel::SelectableExpression<
                diesel::internal::table_macro::Join<
                    Left,
                    Right,
                    diesel::internal::table_macro::LeftOuter,
                >,
            > for id
        where
            id: diesel::AppearsOnTable<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                >,
            Self: diesel::SelectableExpression<Left>,
            Right: diesel::query_source::AppearsInFromClause<
                    super::table,
                    Count = diesel::query_source::Never,
                > + diesel::query_source::QuerySource,
            Left: diesel::query_source::QuerySource,
        {
        }
        impl<Left, Right>
            diesel::SelectableExpression<
                diesel::internal::table_macro::Join<
                    Left,
                    Right,
                    diesel::internal::table_macro::Inner,
                >,
            > for id
        where
            id: diesel::AppearsOnTable<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                >,
            Left: diesel::query_source::AppearsInFromClause<super::table>
                + diesel::query_source::QuerySource,
            Right: diesel::query_source::AppearsInFromClause<super::table>
                + diesel::query_source::QuerySource,
            (Left::Count, Right::Count): diesel::internal::table_macro::Pick<Left, Right>,
            Self:
                diesel::SelectableExpression<
                        <(Left::Count, Right::Count) as diesel::internal::table_macro::Pick<
                            Left,
                            Right,
                        >>::Selection,
                    >,
        {
        }
        impl<Join, On> diesel::SelectableExpression<diesel::internal::table_macro::JoinOn<Join, On>> for id where
            id: diesel::SelectableExpression<Join>
                + diesel::AppearsOnTable<diesel::internal::table_macro::JoinOn<Join, On>>
        {
        }
        impl<From>
            diesel::SelectableExpression<
                diesel::internal::table_macro::SelectStatement<
                    diesel::internal::table_macro::FromClause<From>,
                >,
            > for id
        where
            From: diesel::query_source::QuerySource,
            id: diesel::SelectableExpression<From>
                + diesel::AppearsOnTable<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                >,
        {
        }
        impl<__GB> diesel::expression::ValidGrouping<__GB> for id
        where
            __GB: diesel::expression::IsContainedInGroupBy<
                    id,
                    Output = diesel::expression::is_contained_in_group_by::Yes,
                >,
        {
            type IsAggregate = diesel::expression::is_aggregate::Yes;
        }
        impl diesel::expression::ValidGrouping<()> for id {
            type IsAggregate = diesel::expression::is_aggregate::No;
        }
        impl diesel::expression::IsContainedInGroupBy<id> for id {
            type Output = diesel::expression::is_contained_in_group_by::Yes;
        }
        impl diesel::query_source::Column for id {
            type Table = super::table;
            const NAME: &'static str = "id";
        }
        impl<T> diesel::EqAll<T> for id
        where
            T: diesel::expression::AsExpression<Int8>,
            diesel::dsl::Eq<id, T::Expression>:
                diesel::Expression<SqlType = diesel::sql_types::Bool>,
        {
            type Output = diesel::dsl::Eq<Self, T::Expression>;
            fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                use diesel::expression_methods::ExpressionMethods;
                self.eq(__diesel_internal_rhs)
            }
        }
        impl<Rhs> ::std::ops::Add<Rhs> for id
        where
            Rhs: diesel::expression::AsExpression<
                    <<id as diesel::Expression>::SqlType as diesel::sql_types::ops::Add>::Rhs,
                >,
        {
            type Output = diesel::internal::table_macro::ops::Add<Self, Rhs::Expression>;
            fn add(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                diesel::internal::table_macro::ops::Add::new(
                    self,
                    __diesel_internal_rhs.as_expression(),
                )
            }
        }
        impl<Rhs> ::std::ops::Sub<Rhs> for id
        where
            Rhs: diesel::expression::AsExpression<
                    <<id as diesel::Expression>::SqlType as diesel::sql_types::ops::Sub>::Rhs,
                >,
        {
            type Output = diesel::internal::table_macro::ops::Sub<Self, Rhs::Expression>;
            fn sub(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                diesel::internal::table_macro::ops::Sub::new(
                    self,
                    __diesel_internal_rhs.as_expression(),
                )
            }
        }
        impl<Rhs> ::std::ops::Div<Rhs> for id
        where
            Rhs: diesel::expression::AsExpression<
                    <<id as diesel::Expression>::SqlType as diesel::sql_types::ops::Div>::Rhs,
                >,
        {
            type Output = diesel::internal::table_macro::ops::Div<Self, Rhs::Expression>;
            fn div(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                diesel::internal::table_macro::ops::Div::new(
                    self,
                    __diesel_internal_rhs.as_expression(),
                )
            }
        }
        impl<Rhs> ::std::ops::Mul<Rhs> for id
        where
            Rhs: diesel::expression::AsExpression<
                    <<id as diesel::Expression>::SqlType as diesel::sql_types::ops::Mul>::Rhs,
                >,
        {
            type Output = diesel::internal::table_macro::ops::Mul<Self, Rhs::Expression>;
            fn mul(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                diesel::internal::table_macro::ops::Mul::new(
                    self,
                    __diesel_internal_rhs.as_expression(),
                )
            }
        }
        impl diesel::query_source::AppearsInFromClause<diesel::query_builder::Only<super::table>> for id {
            type Count = diesel::query_source::Once;
        }
        impl diesel::SelectableExpression<diesel::query_builder::Only<super::table>> for id {}
        impl<TSM>
            diesel::query_source::AppearsInFromClause<
                diesel::query_builder::Tablesample<super::table, TSM>,
            > for id
        where
            TSM: diesel::internal::table_macro::TablesampleMethod,
        {
            type Count = diesel::query_source::Once;
        }
        impl<TSM>
            diesel::SelectableExpression<diesel::query_builder::Tablesample<super::table, TSM>>
            for id
        where
            TSM: diesel::internal::table_macro::TablesampleMethod,
        {
        }
        #[allow(non_camel_case_types, dead_code)]
        #[derive(Debug, Clone, Copy, diesel::query_builder::QueryId, Default)]
        pub struct name;
        impl diesel::expression::Expression for name {
            type SqlType = Text;
        }
        impl<DB> diesel::query_builder::QueryFragment<DB> for name
        where
            DB: diesel::backend::Backend,
            diesel::internal::table_macro::StaticQueryFragmentInstance<table>:
                diesel::query_builder::QueryFragment<DB>,
        {
            #[allow(non_snake_case)]
            fn walk_ast<'b>(
                &'b self,
                mut __diesel_internal_out: diesel::query_builder::AstPass<'_, 'b, DB>,
            ) -> diesel::result::QueryResult<()> {
                if !__diesel_internal_out.should_skip_from() {
                    const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                    FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                    __diesel_internal_out.push_sql(".");
                }
                __diesel_internal_out.push_identifier("name")
            }
        }
        impl diesel::SelectableExpression<super::table> for name {}
        impl<QS> diesel::AppearsOnTable<QS> for name where
            QS: diesel::query_source::AppearsInFromClause<
                    super::table,
                    Count = diesel::query_source::Once,
                >
        {
        }
        impl<Left, Right>
            diesel::SelectableExpression<
                diesel::internal::table_macro::Join<
                    Left,
                    Right,
                    diesel::internal::table_macro::LeftOuter,
                >,
            > for name
        where
            name: diesel::AppearsOnTable<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                >,
            Self: diesel::SelectableExpression<Left>,
            Right: diesel::query_source::AppearsInFromClause<
                    super::table,
                    Count = diesel::query_source::Never,
                > + diesel::query_source::QuerySource,
            Left: diesel::query_source::QuerySource,
        {
        }
        impl<Left, Right>
            diesel::SelectableExpression<
                diesel::internal::table_macro::Join<
                    Left,
                    Right,
                    diesel::internal::table_macro::Inner,
                >,
            > for name
        where
            name: diesel::AppearsOnTable<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                >,
            Left: diesel::query_source::AppearsInFromClause<super::table>
                + diesel::query_source::QuerySource,
            Right: diesel::query_source::AppearsInFromClause<super::table>
                + diesel::query_source::QuerySource,
            (Left::Count, Right::Count): diesel::internal::table_macro::Pick<Left, Right>,
            Self:
                diesel::SelectableExpression<
                        <(Left::Count, Right::Count) as diesel::internal::table_macro::Pick<
                            Left,
                            Right,
                        >>::Selection,
                    >,
        {
        }
        impl<Join, On> diesel::SelectableExpression<diesel::internal::table_macro::JoinOn<Join, On>>
            for name
        where
            name: diesel::SelectableExpression<Join>
                + diesel::AppearsOnTable<diesel::internal::table_macro::JoinOn<Join, On>>,
        {
        }
        impl<From>
            diesel::SelectableExpression<
                diesel::internal::table_macro::SelectStatement<
                    diesel::internal::table_macro::FromClause<From>,
                >,
            > for name
        where
            From: diesel::query_source::QuerySource,
            name: diesel::SelectableExpression<From>
                + diesel::AppearsOnTable<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                >,
        {
        }
        impl<__GB> diesel::expression::ValidGrouping<__GB> for name
        where
            __GB: diesel::expression::IsContainedInGroupBy<
                    name,
                    Output = diesel::expression::is_contained_in_group_by::Yes,
                >,
        {
            type IsAggregate = diesel::expression::is_aggregate::Yes;
        }
        impl diesel::expression::ValidGrouping<()> for name {
            type IsAggregate = diesel::expression::is_aggregate::No;
        }
        impl diesel::expression::IsContainedInGroupBy<name> for name {
            type Output = diesel::expression::is_contained_in_group_by::Yes;
        }
        impl diesel::query_source::Column for name {
            type Table = super::table;
            const NAME: &'static str = "name";
        }
        impl<T> diesel::EqAll<T> for name
        where
            T: diesel::expression::AsExpression<Text>,
            diesel::dsl::Eq<name, T::Expression>:
                diesel::Expression<SqlType = diesel::sql_types::Bool>,
        {
            type Output = diesel::dsl::Eq<Self, T::Expression>;
            fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                use diesel::expression_methods::ExpressionMethods;
                self.eq(__diesel_internal_rhs)
            }
        }
        impl diesel::query_source::AppearsInFromClause<diesel::query_builder::Only<super::table>> for name {
            type Count = diesel::query_source::Once;
        }
        impl diesel::SelectableExpression<diesel::query_builder::Only<super::table>> for name {}
        impl<TSM>
            diesel::query_source::AppearsInFromClause<
                diesel::query_builder::Tablesample<super::table, TSM>,
            > for name
        where
            TSM: diesel::internal::table_macro::TablesampleMethod,
        {
            type Count = diesel::query_source::Once;
        }
        impl<TSM>
            diesel::SelectableExpression<diesel::query_builder::Tablesample<super::table, TSM>>
            for name
        where
            TSM: diesel::internal::table_macro::TablesampleMethod,
        {
        }
        #[allow(non_camel_case_types, dead_code)]
        #[derive(Debug, Clone, Copy, diesel::query_builder::QueryId, Default)]
        pub struct remark;
        impl diesel::expression::Expression for remark {
            type SqlType = Nullable<Text>;
        }
        impl<DB> diesel::query_builder::QueryFragment<DB> for remark
        where
            DB: diesel::backend::Backend,
            diesel::internal::table_macro::StaticQueryFragmentInstance<table>:
                diesel::query_builder::QueryFragment<DB>,
        {
            #[allow(non_snake_case)]
            fn walk_ast<'b>(
                &'b self,
                mut __diesel_internal_out: diesel::query_builder::AstPass<'_, 'b, DB>,
            ) -> diesel::result::QueryResult<()> {
                if !__diesel_internal_out.should_skip_from() {
                    const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                    FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                    __diesel_internal_out.push_sql(".");
                }
                __diesel_internal_out.push_identifier("remark")
            }
        }
        impl diesel::SelectableExpression<super::table> for remark {}
        impl<QS> diesel::AppearsOnTable<QS> for remark where
            QS: diesel::query_source::AppearsInFromClause<
                    super::table,
                    Count = diesel::query_source::Once,
                >
        {
        }
        impl<Left, Right>
            diesel::SelectableExpression<
                diesel::internal::table_macro::Join<
                    Left,
                    Right,
                    diesel::internal::table_macro::LeftOuter,
                >,
            > for remark
        where
            remark: diesel::AppearsOnTable<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                >,
            Self: diesel::SelectableExpression<Left>,
            Right: diesel::query_source::AppearsInFromClause<
                    super::table,
                    Count = diesel::query_source::Never,
                > + diesel::query_source::QuerySource,
            Left: diesel::query_source::QuerySource,
        {
        }
        impl<Left, Right>
            diesel::SelectableExpression<
                diesel::internal::table_macro::Join<
                    Left,
                    Right,
                    diesel::internal::table_macro::Inner,
                >,
            > for remark
        where
            remark: diesel::AppearsOnTable<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                >,
            Left: diesel::query_source::AppearsInFromClause<super::table>
                + diesel::query_source::QuerySource,
            Right: diesel::query_source::AppearsInFromClause<super::table>
                + diesel::query_source::QuerySource,
            (Left::Count, Right::Count): diesel::internal::table_macro::Pick<Left, Right>,
            Self:
                diesel::SelectableExpression<
                        <(Left::Count, Right::Count) as diesel::internal::table_macro::Pick<
                            Left,
                            Right,
                        >>::Selection,
                    >,
        {
        }
        impl<Join, On> diesel::SelectableExpression<diesel::internal::table_macro::JoinOn<Join, On>>
            for remark
        where
            remark: diesel::SelectableExpression<Join>
                + diesel::AppearsOnTable<diesel::internal::table_macro::JoinOn<Join, On>>,
        {
        }
        impl<From>
            diesel::SelectableExpression<
                diesel::internal::table_macro::SelectStatement<
                    diesel::internal::table_macro::FromClause<From>,
                >,
            > for remark
        where
            From: diesel::query_source::QuerySource,
            remark: diesel::SelectableExpression<From>
                + diesel::AppearsOnTable<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                >,
        {
        }
        impl<__GB> diesel::expression::ValidGrouping<__GB> for remark
        where
            __GB: diesel::expression::IsContainedInGroupBy<
                    remark,
                    Output = diesel::expression::is_contained_in_group_by::Yes,
                >,
        {
            type IsAggregate = diesel::expression::is_aggregate::Yes;
        }
        impl diesel::expression::ValidGrouping<()> for remark {
            type IsAggregate = diesel::expression::is_aggregate::No;
        }
        impl diesel::expression::IsContainedInGroupBy<remark> for remark {
            type Output = diesel::expression::is_contained_in_group_by::Yes;
        }
        impl diesel::query_source::Column for remark {
            type Table = super::table;
            const NAME: &'static str = "remark";
        }
        impl<T> diesel::EqAll<T> for remark
        where
            T: diesel::expression::AsExpression<Nullable<Text>>,
            diesel::dsl::Eq<remark, T::Expression>:
                diesel::Expression<SqlType = diesel::sql_types::Bool>,
        {
            type Output = diesel::dsl::Eq<Self, T::Expression>;
            fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                use diesel::expression_methods::ExpressionMethods;
                self.eq(__diesel_internal_rhs)
            }
        }
        impl diesel::query_source::AppearsInFromClause<diesel::query_builder::Only<super::table>>
            for remark
        {
            type Count = diesel::query_source::Once;
        }
        impl diesel::SelectableExpression<diesel::query_builder::Only<super::table>> for remark {}
        impl<TSM>
            diesel::query_source::AppearsInFromClause<
                diesel::query_builder::Tablesample<super::table, TSM>,
            > for remark
        where
            TSM: diesel::internal::table_macro::TablesampleMethod,
        {
            type Count = diesel::query_source::Once;
        }
        impl<TSM>
            diesel::SelectableExpression<diesel::query_builder::Tablesample<super::table, TSM>>
            for remark
        where
            TSM: diesel::internal::table_macro::TablesampleMethod,
        {
        }
        #[allow(non_camel_case_types, dead_code)]
        #[derive(Debug, Clone, Copy, diesel::query_builder::QueryId, Default)]
        pub struct update_time;
        impl diesel::expression::Expression for update_time {
            type SqlType = Nullable<Timestamptz>;
        }
        impl<DB> diesel::query_builder::QueryFragment<DB> for update_time
        where
            DB: diesel::backend::Backend,
            diesel::internal::table_macro::StaticQueryFragmentInstance<table>:
                diesel::query_builder::QueryFragment<DB>,
        {
            #[allow(non_snake_case)]
            fn walk_ast<'b>(
                &'b self,
                mut __diesel_internal_out: diesel::query_builder::AstPass<'_, 'b, DB>,
            ) -> diesel::result::QueryResult<()> {
                if !__diesel_internal_out.should_skip_from() {
                    const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                    FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                    __diesel_internal_out.push_sql(".");
                }
                __diesel_internal_out.push_identifier("update_time")
            }
        }
        impl diesel::SelectableExpression<super::table> for update_time {}
        impl<QS> diesel::AppearsOnTable<QS> for update_time where
            QS: diesel::query_source::AppearsInFromClause<
                    super::table,
                    Count = diesel::query_source::Once,
                >
        {
        }
        impl<Left, Right>
            diesel::SelectableExpression<
                diesel::internal::table_macro::Join<
                    Left,
                    Right,
                    diesel::internal::table_macro::LeftOuter,
                >,
            > for update_time
        where
            update_time: diesel::AppearsOnTable<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                >,
            Self: diesel::SelectableExpression<Left>,
            Right: diesel::query_source::AppearsInFromClause<
                    super::table,
                    Count = diesel::query_source::Never,
                > + diesel::query_source::QuerySource,
            Left: diesel::query_source::QuerySource,
        {
        }
        impl<Left, Right>
            diesel::SelectableExpression<
                diesel::internal::table_macro::Join<
                    Left,
                    Right,
                    diesel::internal::table_macro::Inner,
                >,
            > for update_time
        where
            update_time: diesel::AppearsOnTable<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                >,
            Left: diesel::query_source::AppearsInFromClause<super::table>
                + diesel::query_source::QuerySource,
            Right: diesel::query_source::AppearsInFromClause<super::table>
                + diesel::query_source::QuerySource,
            (Left::Count, Right::Count): diesel::internal::table_macro::Pick<Left, Right>,
            Self:
                diesel::SelectableExpression<
                        <(Left::Count, Right::Count) as diesel::internal::table_macro::Pick<
                            Left,
                            Right,
                        >>::Selection,
                    >,
        {
        }
        impl<Join, On> diesel::SelectableExpression<diesel::internal::table_macro::JoinOn<Join, On>>
            for update_time
        where
            update_time: diesel::SelectableExpression<Join>
                + diesel::AppearsOnTable<diesel::internal::table_macro::JoinOn<Join, On>>,
        {
        }
        impl<From>
            diesel::SelectableExpression<
                diesel::internal::table_macro::SelectStatement<
                    diesel::internal::table_macro::FromClause<From>,
                >,
            > for update_time
        where
            From: diesel::query_source::QuerySource,
            update_time: diesel::SelectableExpression<From>
                + diesel::AppearsOnTable<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                >,
        {
        }
        impl<__GB> diesel::expression::ValidGrouping<__GB> for update_time
        where
            __GB: diesel::expression::IsContainedInGroupBy<
                    update_time,
                    Output = diesel::expression::is_contained_in_group_by::Yes,
                >,
        {
            type IsAggregate = diesel::expression::is_aggregate::Yes;
        }
        impl diesel::expression::ValidGrouping<()> for update_time {
            type IsAggregate = diesel::expression::is_aggregate::No;
        }
        impl diesel::expression::IsContainedInGroupBy<update_time> for update_time {
            type Output = diesel::expression::is_contained_in_group_by::Yes;
        }
        impl diesel::query_source::Column for update_time {
            type Table = super::table;
            const NAME: &'static str = "update_time";
        }
        impl<T> diesel::EqAll<T> for update_time
        where
            T: diesel::expression::AsExpression<Nullable<Timestamptz>>,
            diesel::dsl::Eq<update_time, T::Expression>:
                diesel::Expression<SqlType = diesel::sql_types::Bool>,
        {
            type Output = diesel::dsl::Eq<Self, T::Expression>;
            fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                use diesel::expression_methods::ExpressionMethods;
                self.eq(__diesel_internal_rhs)
            }
        }
        impl<Rhs> ::std::ops::Add<Rhs> for update_time
        where
            Rhs: diesel::expression::AsExpression<<<update_time as diesel::Expression>::SqlType as diesel::sql_types::ops::Add>::Rhs, >,
        {
            type Output = diesel::internal::table_macro::ops::Add<Self, Rhs::Expression>;
            fn add(self, __diesel_internal_rhs: Rhs) -> Self::Output { diesel::internal::table_macro::ops::Add::new(self, __diesel_internal_rhs.as_expression()) }
        }
        impl<Rhs> ::std::ops::Sub<Rhs> for update_time
        where
            Rhs: diesel::expression::AsExpression<<<update_time as diesel::Expression>::SqlType as diesel::sql_types::ops::Sub>::Rhs, >,
        {
            type Output = diesel::internal::table_macro::ops::Sub<Self, Rhs::Expression>;
            fn sub(self, __diesel_internal_rhs: Rhs) -> Self::Output { diesel::internal::table_macro::ops::Sub::new(self, __diesel_internal_rhs.as_expression()) }
        }
        impl diesel::query_source::AppearsInFromClause<diesel::query_builder::Only<super::table>>
            for update_time
        {
            type Count = diesel::query_source::Once;
        }
        impl diesel::SelectableExpression<diesel::query_builder::Only<super::table>> for update_time {}
        impl<TSM>
            diesel::query_source::AppearsInFromClause<
                diesel::query_builder::Tablesample<super::table, TSM>,
            > for update_time
        where
            TSM: diesel::internal::table_macro::TablesampleMethod,
        {
            type Count = diesel::query_source::Once;
        }
        impl<TSM>
            diesel::SelectableExpression<diesel::query_builder::Tablesample<super::table, TSM>>
            for update_time
        where
            TSM: diesel::internal::table_macro::TablesampleMethod,
        {
        }
        #[allow(non_camel_case_types, dead_code)]
        #[derive(Debug, Clone, Copy, diesel::query_builder::QueryId, Default)]
        pub struct create_time;
        impl diesel::expression::Expression for create_time {
            type SqlType = Timestamptz;
        }
        impl<DB> diesel::query_builder::QueryFragment<DB> for create_time
        where
            DB: diesel::backend::Backend,
            diesel::internal::table_macro::StaticQueryFragmentInstance<table>:
                diesel::query_builder::QueryFragment<DB>,
        {
            #[allow(non_snake_case)]
            fn walk_ast<'b>(
                &'b self,
                mut __diesel_internal_out: diesel::query_builder::AstPass<'_, 'b, DB>,
            ) -> diesel::result::QueryResult<()> {
                if !__diesel_internal_out.should_skip_from() {
                    const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                    FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                    __diesel_internal_out.push_sql(".");
                }
                __diesel_internal_out.push_identifier("create_time")
            }
        }
        impl diesel::SelectableExpression<super::table> for create_time {}
        impl<QS> diesel::AppearsOnTable<QS> for create_time where
            QS: diesel::query_source::AppearsInFromClause<
                    super::table,
                    Count = diesel::query_source::Once,
                >
        {
        }
        impl<Left, Right>
            diesel::SelectableExpression<
                diesel::internal::table_macro::Join<
                    Left,
                    Right,
                    diesel::internal::table_macro::LeftOuter,
                >,
            > for create_time
        where
            create_time: diesel::AppearsOnTable<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                >,
            Self: diesel::SelectableExpression<Left>,
            Right: diesel::query_source::AppearsInFromClause<
                    super::table,
                    Count = diesel::query_source::Never,
                > + diesel::query_source::QuerySource,
            Left: diesel::query_source::QuerySource,
        {
        }
        impl<Left, Right>
            diesel::SelectableExpression<
                diesel::internal::table_macro::Join<
                    Left,
                    Right,
                    diesel::internal::table_macro::Inner,
                >,
            > for create_time
        where
            create_time: diesel::AppearsOnTable<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                >,
            Left: diesel::query_source::AppearsInFromClause<super::table>
                + diesel::query_source::QuerySource,
            Right: diesel::query_source::AppearsInFromClause<super::table>
                + diesel::query_source::QuerySource,
            (Left::Count, Right::Count): diesel::internal::table_macro::Pick<Left, Right>,
            Self:
                diesel::SelectableExpression<
                        <(Left::Count, Right::Count) as diesel::internal::table_macro::Pick<
                            Left,
                            Right,
                        >>::Selection,
                    >,
        {
        }
        impl<Join, On> diesel::SelectableExpression<diesel::internal::table_macro::JoinOn<Join, On>>
            for create_time
        where
            create_time: diesel::SelectableExpression<Join>
                + diesel::AppearsOnTable<diesel::internal::table_macro::JoinOn<Join, On>>,
        {
        }
        impl<From>
            diesel::SelectableExpression<
                diesel::internal::table_macro::SelectStatement<
                    diesel::internal::table_macro::FromClause<From>,
                >,
            > for create_time
        where
            From: diesel::query_source::QuerySource,
            create_time: diesel::SelectableExpression<From>
                + diesel::AppearsOnTable<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                >,
        {
        }
        impl<__GB> diesel::expression::ValidGrouping<__GB> for create_time
        where
            __GB: diesel::expression::IsContainedInGroupBy<
                    create_time,
                    Output = diesel::expression::is_contained_in_group_by::Yes,
                >,
        {
            type IsAggregate = diesel::expression::is_aggregate::Yes;
        }
        impl diesel::expression::ValidGrouping<()> for create_time {
            type IsAggregate = diesel::expression::is_aggregate::No;
        }
        impl diesel::expression::IsContainedInGroupBy<create_time> for create_time {
            type Output = diesel::expression::is_contained_in_group_by::Yes;
        }
        impl diesel::query_source::Column for create_time {
            type Table = super::table;
            const NAME: &'static str = "create_time";
        }
        impl<T> diesel::EqAll<T> for create_time
        where
            T: diesel::expression::AsExpression<Timestamptz>,
            diesel::dsl::Eq<create_time, T::Expression>:
                diesel::Expression<SqlType = diesel::sql_types::Bool>,
        {
            type Output = diesel::dsl::Eq<Self, T::Expression>;
            fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                use diesel::expression_methods::ExpressionMethods;
                self.eq(__diesel_internal_rhs)
            }
        }
        impl<Rhs> ::std::ops::Add<Rhs> for create_time
        where
            Rhs: diesel::expression::AsExpression<<<create_time as diesel::Expression>::SqlType as diesel::sql_types::ops::Add>::Rhs, >,
        {
            type Output = diesel::internal::table_macro::ops::Add<Self, Rhs::Expression>;
            fn add(self, __diesel_internal_rhs: Rhs) -> Self::Output { diesel::internal::table_macro::ops::Add::new(self, __diesel_internal_rhs.as_expression()) }
        }
        impl<Rhs> ::std::ops::Sub<Rhs> for create_time
        where
            Rhs: diesel::expression::AsExpression<<<create_time as diesel::Expression>::SqlType as diesel::sql_types::ops::Sub>::Rhs, >,
        {
            type Output = diesel::internal::table_macro::ops::Sub<Self, Rhs::Expression>;
            fn sub(self, __diesel_internal_rhs: Rhs) -> Self::Output { diesel::internal::table_macro::ops::Sub::new(self, __diesel_internal_rhs.as_expression()) }
        }
        impl diesel::query_source::AppearsInFromClause<diesel::query_builder::Only<super::table>>
            for create_time
        {
            type Count = diesel::query_source::Once;
        }
        impl diesel::SelectableExpression<diesel::query_builder::Only<super::table>> for create_time {}
        impl<TSM>
            diesel::query_source::AppearsInFromClause<
                diesel::query_builder::Tablesample<super::table, TSM>,
            > for create_time
        where
            TSM: diesel::internal::table_macro::TablesampleMethod,
        {
            type Count = diesel::query_source::Once;
        }
        impl<TSM>
            diesel::SelectableExpression<diesel::query_builder::Tablesample<super::table, TSM>>
            for create_time
        where
            TSM: diesel::internal::table_macro::TablesampleMethod,
        {
        }
        #[allow(non_camel_case_types, dead_code)]
        #[derive(Debug, Clone, Copy, diesel::query_builder::QueryId, Default)]
        pub struct create_by;
        impl diesel::expression::Expression for create_by {
            type SqlType = Int8;
        }
        impl<DB> diesel::query_builder::QueryFragment<DB> for create_by
        where
            DB: diesel::backend::Backend,
            diesel::internal::table_macro::StaticQueryFragmentInstance<table>:
                diesel::query_builder::QueryFragment<DB>,
        {
            #[allow(non_snake_case)]
            fn walk_ast<'b>(
                &'b self,
                mut __diesel_internal_out: diesel::query_builder::AstPass<'_, 'b, DB>,
            ) -> diesel::result::QueryResult<()> {
                if !__diesel_internal_out.should_skip_from() {
                    const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                    FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                    __diesel_internal_out.push_sql(".");
                }
                __diesel_internal_out.push_identifier("create_by")
            }
        }
        impl diesel::SelectableExpression<super::table> for create_by {}
        impl<QS> diesel::AppearsOnTable<QS> for create_by where
            QS: diesel::query_source::AppearsInFromClause<
                    super::table,
                    Count = diesel::query_source::Once,
                >
        {
        }
        impl<Left, Right>
            diesel::SelectableExpression<
                diesel::internal::table_macro::Join<
                    Left,
                    Right,
                    diesel::internal::table_macro::LeftOuter,
                >,
            > for create_by
        where
            create_by: diesel::AppearsOnTable<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                >,
            Self: diesel::SelectableExpression<Left>,
            Right: diesel::query_source::AppearsInFromClause<
                    super::table,
                    Count = diesel::query_source::Never,
                > + diesel::query_source::QuerySource,
            Left: diesel::query_source::QuerySource,
        {
        }
        impl<Left, Right>
            diesel::SelectableExpression<
                diesel::internal::table_macro::Join<
                    Left,
                    Right,
                    diesel::internal::table_macro::Inner,
                >,
            > for create_by
        where
            create_by: diesel::AppearsOnTable<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                >,
            Left: diesel::query_source::AppearsInFromClause<super::table>
                + diesel::query_source::QuerySource,
            Right: diesel::query_source::AppearsInFromClause<super::table>
                + diesel::query_source::QuerySource,
            (Left::Count, Right::Count): diesel::internal::table_macro::Pick<Left, Right>,
            Self:
                diesel::SelectableExpression<
                        <(Left::Count, Right::Count) as diesel::internal::table_macro::Pick<
                            Left,
                            Right,
                        >>::Selection,
                    >,
        {
        }
        impl<Join, On> diesel::SelectableExpression<diesel::internal::table_macro::JoinOn<Join, On>>
            for create_by
        where
            create_by: diesel::SelectableExpression<Join>
                + diesel::AppearsOnTable<diesel::internal::table_macro::JoinOn<Join, On>>,
        {
        }
        impl<From>
            diesel::SelectableExpression<
                diesel::internal::table_macro::SelectStatement<
                    diesel::internal::table_macro::FromClause<From>,
                >,
            > for create_by
        where
            From: diesel::query_source::QuerySource,
            create_by: diesel::SelectableExpression<From>
                + diesel::AppearsOnTable<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                >,
        {
        }
        impl<__GB> diesel::expression::ValidGrouping<__GB> for create_by
        where
            __GB: diesel::expression::IsContainedInGroupBy<
                    create_by,
                    Output = diesel::expression::is_contained_in_group_by::Yes,
                >,
        {
            type IsAggregate = diesel::expression::is_aggregate::Yes;
        }
        impl diesel::expression::ValidGrouping<()> for create_by {
            type IsAggregate = diesel::expression::is_aggregate::No;
        }
        impl diesel::expression::IsContainedInGroupBy<create_by> for create_by {
            type Output = diesel::expression::is_contained_in_group_by::Yes;
        }
        impl diesel::query_source::Column for create_by {
            type Table = super::table;
            const NAME: &'static str = "create_by";
        }
        impl<T> diesel::EqAll<T> for create_by
        where
            T: diesel::expression::AsExpression<Int8>,
            diesel::dsl::Eq<create_by, T::Expression>:
                diesel::Expression<SqlType = diesel::sql_types::Bool>,
        {
            type Output = diesel::dsl::Eq<Self, T::Expression>;
            fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                use diesel::expression_methods::ExpressionMethods;
                self.eq(__diesel_internal_rhs)
            }
        }
        impl<Rhs> ::std::ops::Add<Rhs> for create_by
        where
            Rhs: diesel::expression::AsExpression<<<create_by as diesel::Expression>::SqlType as diesel::sql_types::ops::Add>::Rhs, >,
        {
            type Output = diesel::internal::table_macro::ops::Add<Self, Rhs::Expression>;
            fn add(self, __diesel_internal_rhs: Rhs) -> Self::Output { diesel::internal::table_macro::ops::Add::new(self, __diesel_internal_rhs.as_expression()) }
        }
        impl<Rhs> ::std::ops::Sub<Rhs> for create_by
        where
            Rhs: diesel::expression::AsExpression<<<create_by as diesel::Expression>::SqlType as diesel::sql_types::ops::Sub>::Rhs, >,
        {
            type Output = diesel::internal::table_macro::ops::Sub<Self, Rhs::Expression>;
            fn sub(self, __diesel_internal_rhs: Rhs) -> Self::Output { diesel::internal::table_macro::ops::Sub::new(self, __diesel_internal_rhs.as_expression()) }
        }
        impl<Rhs> ::std::ops::Div<Rhs> for create_by
        where
            Rhs: diesel::expression::AsExpression<<<create_by as diesel::Expression>::SqlType as diesel::sql_types::ops::Div>::Rhs, >,
        {
            type Output = diesel::internal::table_macro::ops::Div<Self, Rhs::Expression>;
            fn div(self, __diesel_internal_rhs: Rhs) -> Self::Output { diesel::internal::table_macro::ops::Div::new(self, __diesel_internal_rhs.as_expression()) }
        }
        impl<Rhs> ::std::ops::Mul<Rhs> for create_by
        where
            Rhs: diesel::expression::AsExpression<<<create_by as diesel::Expression>::SqlType as diesel::sql_types::ops::Mul>::Rhs, >,
        {
            type Output = diesel::internal::table_macro::ops::Mul<Self, Rhs::Expression>;
            fn mul(self, __diesel_internal_rhs: Rhs) -> Self::Output { diesel::internal::table_macro::ops::Mul::new(self, __diesel_internal_rhs.as_expression()) }
        }
        impl diesel::query_source::AppearsInFromClause<diesel::query_builder::Only<super::table>>
            for create_by
        {
            type Count = diesel::query_source::Once;
        }
        impl diesel::SelectableExpression<diesel::query_builder::Only<super::table>> for create_by {}
        impl<TSM>
            diesel::query_source::AppearsInFromClause<
                diesel::query_builder::Tablesample<super::table, TSM>,
            > for create_by
        where
            TSM: diesel::internal::table_macro::TablesampleMethod,
        {
            type Count = diesel::query_source::Once;
        }
        impl<TSM>
            diesel::SelectableExpression<diesel::query_builder::Tablesample<super::table, TSM>>
            for create_by
        where
            TSM: diesel::internal::table_macro::TablesampleMethod,
        {
        }
        #[allow(non_camel_case_types, dead_code)]
        #[derive(Debug, Clone, Copy, diesel::query_builder::QueryId, Default)]
        pub struct update_by;
        impl diesel::expression::Expression for update_by {
            type SqlType = Nullable<Int8>;
        }
        impl<DB> diesel::query_builder::QueryFragment<DB> for update_by
        where
            DB: diesel::backend::Backend,
            diesel::internal::table_macro::StaticQueryFragmentInstance<table>:
                diesel::query_builder::QueryFragment<DB>,
        {
            #[allow(non_snake_case)]
            fn walk_ast<'b>(
                &'b self,
                mut __diesel_internal_out: diesel::query_builder::AstPass<'_, 'b, DB>,
            ) -> diesel::result::QueryResult<()> {
                if !__diesel_internal_out.should_skip_from() {
                    const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                    FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                    __diesel_internal_out.push_sql(".");
                }
                __diesel_internal_out.push_identifier("update_by")
            }
        }
        impl diesel::SelectableExpression<super::table> for update_by {}
        impl<QS> diesel::AppearsOnTable<QS> for update_by where
            QS: diesel::query_source::AppearsInFromClause<
                    super::table,
                    Count = diesel::query_source::Once,
                >
        {
        }
        impl<Left, Right>
            diesel::SelectableExpression<
                diesel::internal::table_macro::Join<
                    Left,
                    Right,
                    diesel::internal::table_macro::LeftOuter,
                >,
            > for update_by
        where
            update_by: diesel::AppearsOnTable<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                >,
            Self: diesel::SelectableExpression<Left>,
            Right: diesel::query_source::AppearsInFromClause<
                    super::table,
                    Count = diesel::query_source::Never,
                > + diesel::query_source::QuerySource,
            Left: diesel::query_source::QuerySource,
        {
        }
        impl<Left, Right>
            diesel::SelectableExpression<
                diesel::internal::table_macro::Join<
                    Left,
                    Right,
                    diesel::internal::table_macro::Inner,
                >,
            > for update_by
        where
            update_by: diesel::AppearsOnTable<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                >,
            Left: diesel::query_source::AppearsInFromClause<super::table>
                + diesel::query_source::QuerySource,
            Right: diesel::query_source::AppearsInFromClause<super::table>
                + diesel::query_source::QuerySource,
            (Left::Count, Right::Count): diesel::internal::table_macro::Pick<Left, Right>,
            Self:
                diesel::SelectableExpression<
                        <(Left::Count, Right::Count) as diesel::internal::table_macro::Pick<
                            Left,
                            Right,
                        >>::Selection,
                    >,
        {
        }
        impl<Join, On> diesel::SelectableExpression<diesel::internal::table_macro::JoinOn<Join, On>>
            for update_by
        where
            update_by: diesel::SelectableExpression<Join>
                + diesel::AppearsOnTable<diesel::internal::table_macro::JoinOn<Join, On>>,
        {
        }
        impl<From>
            diesel::SelectableExpression<
                diesel::internal::table_macro::SelectStatement<
                    diesel::internal::table_macro::FromClause<From>,
                >,
            > for update_by
        where
            From: diesel::query_source::QuerySource,
            update_by: diesel::SelectableExpression<From>
                + diesel::AppearsOnTable<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                >,
        {
        }
        impl<__GB> diesel::expression::ValidGrouping<__GB> for update_by
        where
            __GB: diesel::expression::IsContainedInGroupBy<
                    update_by,
                    Output = diesel::expression::is_contained_in_group_by::Yes,
                >,
        {
            type IsAggregate = diesel::expression::is_aggregate::Yes;
        }
        impl diesel::expression::ValidGrouping<()> for update_by {
            type IsAggregate = diesel::expression::is_aggregate::No;
        }
        impl diesel::expression::IsContainedInGroupBy<update_by> for update_by {
            type Output = diesel::expression::is_contained_in_group_by::Yes;
        }
        impl diesel::query_source::Column for update_by {
            type Table = super::table;
            const NAME: &'static str = "update_by";
        }
        impl<T> diesel::EqAll<T> for update_by
        where
            T: diesel::expression::AsExpression<Nullable<Int8>>,
            diesel::dsl::Eq<update_by, T::Expression>:
                diesel::Expression<SqlType = diesel::sql_types::Bool>,
        {
            type Output = diesel::dsl::Eq<Self, T::Expression>;
            fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                use diesel::expression_methods::ExpressionMethods;
                self.eq(__diesel_internal_rhs)
            }
        }
        impl<Rhs> ::std::ops::Add<Rhs> for update_by
        where
            Rhs: diesel::expression::AsExpression<<<update_by as diesel::Expression>::SqlType as diesel::sql_types::ops::Add>::Rhs, >,
        {
            type Output = diesel::internal::table_macro::ops::Add<Self, Rhs::Expression>;
            fn add(self, __diesel_internal_rhs: Rhs) -> Self::Output { diesel::internal::table_macro::ops::Add::new(self, __diesel_internal_rhs.as_expression()) }
        }
        impl<Rhs> ::std::ops::Sub<Rhs> for update_by
        where
            Rhs: diesel::expression::AsExpression<<<update_by as diesel::Expression>::SqlType as diesel::sql_types::ops::Sub>::Rhs, >,
        {
            type Output = diesel::internal::table_macro::ops::Sub<Self, Rhs::Expression>;
            fn sub(self, __diesel_internal_rhs: Rhs) -> Self::Output { diesel::internal::table_macro::ops::Sub::new(self, __diesel_internal_rhs.as_expression()) }
        }
        impl<Rhs> ::std::ops::Div<Rhs> for update_by
        where
            Rhs: diesel::expression::AsExpression<<<update_by as diesel::Expression>::SqlType as diesel::sql_types::ops::Div>::Rhs, >,
        {
            type Output = diesel::internal::table_macro::ops::Div<Self, Rhs::Expression>;
            fn div(self, __diesel_internal_rhs: Rhs) -> Self::Output { diesel::internal::table_macro::ops::Div::new(self, __diesel_internal_rhs.as_expression()) }
        }
        impl<Rhs> ::std::ops::Mul<Rhs> for update_by
        where
            Rhs: diesel::expression::AsExpression<<<update_by as diesel::Expression>::SqlType as diesel::sql_types::ops::Mul>::Rhs, >,
        {
            type Output = diesel::internal::table_macro::ops::Mul<Self, Rhs::Expression>;
            fn mul(self, __diesel_internal_rhs: Rhs) -> Self::Output { diesel::internal::table_macro::ops::Mul::new(self, __diesel_internal_rhs.as_expression()) }
        }
        impl diesel::query_source::AppearsInFromClause<diesel::query_builder::Only<super::table>>
            for update_by
        {
            type Count = diesel::query_source::Once;
        }
        impl diesel::SelectableExpression<diesel::query_builder::Only<super::table>> for update_by {}
        impl<TSM>
            diesel::query_source::AppearsInFromClause<
                diesel::query_builder::Tablesample<super::table, TSM>,
            > for update_by
        where
            TSM: diesel::internal::table_macro::TablesampleMethod,
        {
            type Count = diesel::query_source::Once;
        }
        impl<TSM>
            diesel::SelectableExpression<diesel::query_builder::Tablesample<super::table, TSM>>
            for update_by
        where
            TSM: diesel::internal::table_macro::TablesampleMethod,
        {
        }
        #[allow(non_camel_case_types, dead_code)]
        #[derive(Debug, Clone, Copy, diesel::query_builder::QueryId, Default)]
        pub struct is_delete;
        impl diesel::expression::Expression for is_delete {
            type SqlType = Bool;
        }
        impl<DB> diesel::query_builder::QueryFragment<DB> for is_delete
        where
            DB: diesel::backend::Backend,
            diesel::internal::table_macro::StaticQueryFragmentInstance<table>:
                diesel::query_builder::QueryFragment<DB>,
        {
            #[allow(non_snake_case)]
            fn walk_ast<'b>(
                &'b self,
                mut __diesel_internal_out: diesel::query_builder::AstPass<'_, 'b, DB>,
            ) -> diesel::result::QueryResult<()> {
                if !__diesel_internal_out.should_skip_from() {
                    const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                    FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                    __diesel_internal_out.push_sql(".");
                }
                __diesel_internal_out.push_identifier("is_delete")
            }
        }
        impl diesel::SelectableExpression<super::table> for is_delete {}
        impl<QS> diesel::AppearsOnTable<QS> for is_delete where
            QS: diesel::query_source::AppearsInFromClause<
                    super::table,
                    Count = diesel::query_source::Once,
                >
        {
        }
        impl<Left, Right>
            diesel::SelectableExpression<
                diesel::internal::table_macro::Join<
                    Left,
                    Right,
                    diesel::internal::table_macro::LeftOuter,
                >,
            > for is_delete
        where
            is_delete: diesel::AppearsOnTable<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                >,
            Self: diesel::SelectableExpression<Left>,
            Right: diesel::query_source::AppearsInFromClause<
                    super::table,
                    Count = diesel::query_source::Never,
                > + diesel::query_source::QuerySource,
            Left: diesel::query_source::QuerySource,
        {
        }
        impl<Left, Right>
            diesel::SelectableExpression<
                diesel::internal::table_macro::Join<
                    Left,
                    Right,
                    diesel::internal::table_macro::Inner,
                >,
            > for is_delete
        where
            is_delete: diesel::AppearsOnTable<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                >,
            Left: diesel::query_source::AppearsInFromClause<super::table>
                + diesel::query_source::QuerySource,
            Right: diesel::query_source::AppearsInFromClause<super::table>
                + diesel::query_source::QuerySource,
            (Left::Count, Right::Count): diesel::internal::table_macro::Pick<Left, Right>,
            Self:
                diesel::SelectableExpression<
                        <(Left::Count, Right::Count) as diesel::internal::table_macro::Pick<
                            Left,
                            Right,
                        >>::Selection,
                    >,
        {
        }
        impl<Join, On> diesel::SelectableExpression<diesel::internal::table_macro::JoinOn<Join, On>>
            for is_delete
        where
            is_delete: diesel::SelectableExpression<Join>
                + diesel::AppearsOnTable<diesel::internal::table_macro::JoinOn<Join, On>>,
        {
        }
        impl<From>
            diesel::SelectableExpression<
                diesel::internal::table_macro::SelectStatement<
                    diesel::internal::table_macro::FromClause<From>,
                >,
            > for is_delete
        where
            From: diesel::query_source::QuerySource,
            is_delete: diesel::SelectableExpression<From>
                + diesel::AppearsOnTable<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                >,
        {
        }
        impl<__GB> diesel::expression::ValidGrouping<__GB> for is_delete
        where
            __GB: diesel::expression::IsContainedInGroupBy<
                    is_delete,
                    Output = diesel::expression::is_contained_in_group_by::Yes,
                >,
        {
            type IsAggregate = diesel::expression::is_aggregate::Yes;
        }
        impl diesel::expression::ValidGrouping<()> for is_delete {
            type IsAggregate = diesel::expression::is_aggregate::No;
        }
        impl diesel::expression::IsContainedInGroupBy<is_delete> for is_delete {
            type Output = diesel::expression::is_contained_in_group_by::Yes;
        }
        impl diesel::query_source::Column for is_delete {
            type Table = super::table;
            const NAME: &'static str = "is_delete";
        }
        impl<T> diesel::EqAll<T> for is_delete
        where
            T: diesel::expression::AsExpression<Bool>,
            diesel::dsl::Eq<is_delete, T::Expression>:
                diesel::Expression<SqlType = diesel::sql_types::Bool>,
        {
            type Output = diesel::dsl::Eq<Self, T::Expression>;
            fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                use diesel::expression_methods::ExpressionMethods;
                self.eq(__diesel_internal_rhs)
            }
        }
        impl diesel::query_source::AppearsInFromClause<diesel::query_builder::Only<super::table>>
            for is_delete
        {
            type Count = diesel::query_source::Once;
        }
        impl diesel::SelectableExpression<diesel::query_builder::Only<super::table>> for is_delete {}
        impl<TSM>
            diesel::query_source::AppearsInFromClause<
                diesel::query_builder::Tablesample<super::table, TSM>,
            > for is_delete
        where
            TSM: diesel::internal::table_macro::TablesampleMethod,
        {
            type Count = diesel::query_source::Once;
        }
        impl<TSM>
            diesel::SelectableExpression<diesel::query_builder::Tablesample<super::table, TSM>>
            for is_delete
        where
            TSM: diesel::internal::table_macro::TablesampleMethod,
        {
        }
        #[allow(non_camel_case_types, dead_code)]
        #[derive(Debug, Clone, Copy, diesel::query_builder::QueryId, Default)]
        pub struct permissions;
        impl diesel::expression::Expression for permissions {
            type SqlType = Array<Text>;
        }
        impl<DB> diesel::query_builder::QueryFragment<DB> for permissions
        where
            DB: diesel::backend::Backend,
            diesel::internal::table_macro::StaticQueryFragmentInstance<table>:
                diesel::query_builder::QueryFragment<DB>,
        {
            #[allow(non_snake_case)]
            fn walk_ast<'b>(
                &'b self,
                mut __diesel_internal_out: diesel::query_builder::AstPass<'_, 'b, DB>,
            ) -> diesel::result::QueryResult<()> {
                if !__diesel_internal_out.should_skip_from() {
                    const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                        table,
                    > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                    FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                    __diesel_internal_out.push_sql(".");
                }
                __diesel_internal_out.push_identifier("permissions")
            }
        }
        impl diesel::SelectableExpression<super::table> for permissions {}
        impl<QS> diesel::AppearsOnTable<QS> for permissions where
            QS: diesel::query_source::AppearsInFromClause<
                    super::table,
                    Count = diesel::query_source::Once,
                >
        {
        }
        impl<Left, Right>
            diesel::SelectableExpression<
                diesel::internal::table_macro::Join<
                    Left,
                    Right,
                    diesel::internal::table_macro::LeftOuter,
                >,
            > for permissions
        where
            permissions: diesel::AppearsOnTable<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                >,
            Self: diesel::SelectableExpression<Left>,
            Right: diesel::query_source::AppearsInFromClause<
                    super::table,
                    Count = diesel::query_source::Never,
                > + diesel::query_source::QuerySource,
            Left: diesel::query_source::QuerySource,
        {
        }
        impl<Left, Right>
            diesel::SelectableExpression<
                diesel::internal::table_macro::Join<
                    Left,
                    Right,
                    diesel::internal::table_macro::Inner,
                >,
            > for permissions
        where
            permissions: diesel::AppearsOnTable<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                >,
            Left: diesel::query_source::AppearsInFromClause<super::table>
                + diesel::query_source::QuerySource,
            Right: diesel::query_source::AppearsInFromClause<super::table>
                + diesel::query_source::QuerySource,
            (Left::Count, Right::Count): diesel::internal::table_macro::Pick<Left, Right>,
            Self:
                diesel::SelectableExpression<
                        <(Left::Count, Right::Count) as diesel::internal::table_macro::Pick<
                            Left,
                            Right,
                        >>::Selection,
                    >,
        {
        }
        impl<Join, On> diesel::SelectableExpression<diesel::internal::table_macro::JoinOn<Join, On>>
            for permissions
        where
            permissions: diesel::SelectableExpression<Join>
                + diesel::AppearsOnTable<diesel::internal::table_macro::JoinOn<Join, On>>,
        {
        }
        impl<From>
            diesel::SelectableExpression<
                diesel::internal::table_macro::SelectStatement<
                    diesel::internal::table_macro::FromClause<From>,
                >,
            > for permissions
        where
            From: diesel::query_source::QuerySource,
            permissions: diesel::SelectableExpression<From>
                + diesel::AppearsOnTable<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                >,
        {
        }
        impl<__GB> diesel::expression::ValidGrouping<__GB> for permissions
        where
            __GB: diesel::expression::IsContainedInGroupBy<
                    permissions,
                    Output = diesel::expression::is_contained_in_group_by::Yes,
                >,
        {
            type IsAggregate = diesel::expression::is_aggregate::Yes;
        }
        impl diesel::expression::ValidGrouping<()> for permissions {
            type IsAggregate = diesel::expression::is_aggregate::No;
        }
        impl diesel::expression::IsContainedInGroupBy<permissions> for permissions {
            type Output = diesel::expression::is_contained_in_group_by::Yes;
        }
        impl diesel::query_source::Column for permissions {
            type Table = super::table;
            const NAME: &'static str = "permissions";
        }
        impl<T> diesel::EqAll<T> for permissions
        where
            T: diesel::expression::AsExpression<Array<Text>>,
            diesel::dsl::Eq<permissions, T::Expression>:
                diesel::Expression<SqlType = diesel::sql_types::Bool>,
        {
            type Output = diesel::dsl::Eq<Self, T::Expression>;
            fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                use diesel::expression_methods::ExpressionMethods;
                self.eq(__diesel_internal_rhs)
            }
        }
        impl diesel::query_source::AppearsInFromClause<diesel::query_builder::Only<super::table>>
            for permissions
        {
            type Count = diesel::query_source::Once;
        }
        impl diesel::SelectableExpression<diesel::query_builder::Only<super::table>> for permissions {}
        impl<TSM>
            diesel::query_source::AppearsInFromClause<
                diesel::query_builder::Tablesample<super::table, TSM>,
            > for permissions
        where
            TSM: diesel::internal::table_macro::TablesampleMethod,
        {
            type Count = diesel::query_source::Once;
        }
        impl<TSM>
            diesel::SelectableExpression<diesel::query_builder::Tablesample<super::table, TSM>>
            for permissions
        where
            TSM: diesel::internal::table_macro::TablesampleMethod,
        {
        }
        impl diesel::expression::IsContainedInGroupBy<id> for name {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<name> for id {
            type Output = diesel::expression::is_contained_in_group_by::Yes;
        }
        impl diesel::expression::IsContainedInGroupBy<id> for remark {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<remark> for id {
            type Output = diesel::expression::is_contained_in_group_by::Yes;
        }
        impl diesel::expression::IsContainedInGroupBy<id> for update_time {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<update_time> for id {
            type Output = diesel::expression::is_contained_in_group_by::Yes;
        }
        impl diesel::expression::IsContainedInGroupBy<id> for create_time {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<create_time> for id {
            type Output = diesel::expression::is_contained_in_group_by::Yes;
        }
        impl diesel::expression::IsContainedInGroupBy<id> for create_by {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<create_by> for id {
            type Output = diesel::expression::is_contained_in_group_by::Yes;
        }
        impl diesel::expression::IsContainedInGroupBy<id> for update_by {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<update_by> for id {
            type Output = diesel::expression::is_contained_in_group_by::Yes;
        }
        impl diesel::expression::IsContainedInGroupBy<id> for is_delete {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<is_delete> for id {
            type Output = diesel::expression::is_contained_in_group_by::Yes;
        }
        impl diesel::expression::IsContainedInGroupBy<id> for permissions {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<permissions> for id {
            type Output = diesel::expression::is_contained_in_group_by::Yes;
        }
        impl diesel::expression::IsContainedInGroupBy<name> for remark {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<remark> for name {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<name> for update_time {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<update_time> for name {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<name> for create_time {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<create_time> for name {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<name> for create_by {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<create_by> for name {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<name> for update_by {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<update_by> for name {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<name> for is_delete {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<is_delete> for name {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<name> for permissions {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<permissions> for name {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<remark> for update_time {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<update_time> for remark {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<remark> for create_time {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<create_time> for remark {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<remark> for create_by {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<create_by> for remark {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<remark> for update_by {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<update_by> for remark {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<remark> for is_delete {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<is_delete> for remark {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<remark> for permissions {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<permissions> for remark {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<update_time> for create_time {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<create_time> for update_time {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<update_time> for create_by {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<create_by> for update_time {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<update_time> for update_by {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<update_by> for update_time {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<update_time> for is_delete {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<is_delete> for update_time {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<update_time> for permissions {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<permissions> for update_time {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<create_time> for create_by {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<create_by> for create_time {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<create_time> for update_by {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<update_by> for create_time {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<create_time> for is_delete {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<is_delete> for create_time {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<create_time> for permissions {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<permissions> for create_time {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<create_by> for update_by {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<update_by> for create_by {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<create_by> for is_delete {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<is_delete> for create_by {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<create_by> for permissions {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<permissions> for create_by {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<update_by> for is_delete {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<is_delete> for update_by {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<update_by> for permissions {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<permissions> for update_by {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<is_delete> for permissions {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
        impl diesel::expression::IsContainedInGroupBy<permissions> for is_delete {
            type Output = diesel::expression::is_contained_in_group_by::No;
        }
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

diesel::table! {
    req_records (id) {
        id -> Int8,

        username -> Nullable<Text>,
        req_id -> Text,
        req_body -> Nullable<Text>,
        path -> Text,
        status_code -> Text,

        update_time -> Nullable<Timestamptz>,
        create_time -> Timestamptz,
        create_by -> Int8,
        update_by -> Nullable<Int8>,
        is_delete -> Bool,
    }
}

diesel::joinable!(users -> groups (group_id));

diesel::allow_tables_to_appear_in_same_query!(groups, users,);
