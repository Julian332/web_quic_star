use crate::framework::db::DbType;
use diesel::expression::BoxableExpression;
use diesel::query_builder::{BoxedSelectStatement, FromClause};
use diesel::sql_types::Bool;
use diesel_dynamic_schema::Table;
use rust_decimal::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(default)]
pub struct PageParam<T: Default> {
    pub filters: T,
    pub page_no: i64,
    pub page_size: i64,
    pub order_column: String,
    pub is_desc: bool,
}

impl<T: Default> Default for PageParam<T> {
    fn default() -> Self {
        PageParam {
            filters: T::default(),
            page_no: 1,
            page_size: 10,
            order_column: "create_time".to_string(),
            is_desc: true,
        }
    }
}

impl<T: Default> PageParam<T> {
    pub fn get_offset_limit(&self) -> (i64, i64) {
        ((self.page_no - 1) * self.page_size, self.page_size)
    }
}

#[derive(Deserialize, Serialize, JsonSchema, Clone, Default)]
pub enum Compare {
    NotEqual,
    #[default]
    Equal,
    Greater,
    GreaterAndEqual,
    Less,
    LessAndEqual,
}

impl Compare {
    pub fn to_ident(self) -> String {
        match self {
            Compare::NotEqual => "ne",
            Compare::Equal => "eq",
            Compare::Greater => "gt",
            Compare::GreaterAndEqual => "ge",
            Compare::Less => "lt",
            Compare::LessAndEqual => "le",
        }
        .to_string()
    }
}

#[derive(Deserialize, Serialize, JsonSchema, Clone)]
pub enum Filter {
    Condition(Condition),
    And(Vec<Filter>),
    Or(Vec<Filter>),
}

impl Default for Filter {
    fn default() -> Self {
        Filter::And(Vec::new())
    }
}

impl Filter {
    pub fn append_to_sql<'a, ST, QS, GB>(
        self,
        statement: BoxedSelectStatement<'a, ST, FromClause<QS>, DbType, GB>,
        table: &'a Table<&'a str, &'a str>,
    ) -> BoxedSelectStatement<'a, ST, FromClause<QS>, DbType, GB>
    where
        QS: diesel::QuerySource + 'static,
        ST: 'a,
        GB: 'a,
    {
        use diesel::query_dsl::methods::FilterDsl;
        match self.to_boxed_expr::<QS>(table) {
            Some(expr) => FilterDsl::filter(statement, expr),
            None => statement,
        }
    }

    fn to_boxed_expr<'a, QS>(
        self,
        table: &'a Table<&'a str, &'a str>,
    ) -> Option<Box<dyn BoxableExpression<QS, DbType, (), SqlType = Bool> + 'a>>
    where
        QS: diesel::QuerySource + 'static,
    {
        use diesel::BoolExpressionMethods;
        use diesel::ExpressionMethods;
        match self {
            Filter::And(list) => {
                let mut iter = list.into_iter().filter_map(|f| f.to_boxed_expr(table));
                let first = iter.next()?;
                Some(iter.fold(first, |acc, e| Box::new(acc.and(e))))
            }
            Filter::Or(list) => {
                let mut iter = list.into_iter().filter_map(|f| f.to_boxed_expr(table));
                let first = iter.next()?;
                Some(iter.fold(first, |acc, e| Box::new(acc.or(e))))
            }
            Filter::Condition(cond) => {
                let compare = cond.compare.unwrap_or_default();
                match cond.compare_value {
                    CompareValue::Bool(v) => {
                        let col = table.column::<Bool, _>(cond.column);
                        Some(match compare {
                            Compare::Equal => Box::new(col.eq(v)),
                            Compare::NotEqual => Box::new(col.ne(v)),
                            Compare::Greater => Box::new(col.gt(v)),
                            Compare::GreaterAndEqual => Box::new(col.ge(v)),
                            Compare::Less => Box::new(col.lt(v)),
                            Compare::LessAndEqual => Box::new(col.le(v)),
                        })
                    }
                    CompareValue::Float(v) => {
                        let col = table.column::<diesel::sql_types::Float8, _>(cond.column);
                        Some(match compare {
                            Compare::Equal => Box::new(col.eq(v)),
                            Compare::NotEqual => Box::new(col.ne(v)),
                            Compare::Greater => Box::new(col.gt(v)),
                            Compare::GreaterAndEqual => Box::new(col.ge(v)),
                            Compare::Less => Box::new(col.lt(v)),
                            Compare::LessAndEqual => Box::new(col.le(v)),
                        })
                    }
                    CompareValue::String(v) => {
                        let col = table.column::<diesel::sql_types::Text, _>(cond.column);
                        Some(match compare {
                            Compare::Equal => Box::new(col.eq(v)),
                            Compare::NotEqual => Box::new(col.ne(v)),
                            Compare::Greater => Box::new(col.gt(v)),
                            Compare::GreaterAndEqual => Box::new(col.ge(v)),
                            Compare::Less => Box::new(col.lt(v)),
                            Compare::LessAndEqual => Box::new(col.le(v)),
                        })
                    }
                    CompareValue::Decimal(v) => {
                        let col = table.column::<diesel::sql_types::Numeric, _>(cond.column);
                        Some(match compare {
                            Compare::Equal => Box::new(col.eq(v)),
                            Compare::NotEqual => Box::new(col.ne(v)),
                            Compare::Greater => Box::new(col.gt(v)),
                            Compare::GreaterAndEqual => Box::new(col.ge(v)),
                            Compare::Less => Box::new(col.lt(v)),
                            Compare::LessAndEqual => Box::new(col.le(v)),
                        })
                    }
                }
            }
        }
    }
}

#[derive(Deserialize, Serialize, JsonSchema, Clone)]
pub struct Condition {
    pub column: String,
    pub compare: Option<Compare>,
    pub compare_value: CompareValue,
}

#[derive(Deserialize, Serialize, JsonSchema, Clone)]
pub enum CompareValue {
    Decimal(Decimal),
    Bool(bool),
    Float(f64),
    String(String),
}
#[derive(Debug, Serialize, Deserialize, Default, JsonSchema)]
#[serde(default)]
pub struct PageRes<T: Default, TBuilder: Default> {
    pub page_no: i64,
    pub page_size: i64,
    pub records: Vec<T>,
    pub total_page: i64,
    pub filters: TBuilder,
}

impl<T: Default, TBuilder: Default> PageRes<T, TBuilder> {
    pub fn from_param_records(param: PageParam<TBuilder>, records: Vec<T>) -> PageRes<T, TBuilder> {
        PageRes {
            page_no: param.page_no,
            page_size: param.page_size,
            records,
            total_page: -1,
            filters: param.filters,
        }
    }
    pub fn from_param_records_count(
        param: PageParam<TBuilder>,
        records: Vec<T>,
        total_count: i64,
    ) -> PageRes<T, TBuilder> {
        if total_count % param.page_size == 0 {
            PageRes {
                page_no: param.page_no,
                page_size: param.page_size,
                records,
                total_page: total_count / param.page_size,
                filters: param.filters,
            }
        } else {
            PageRes {
                page_no: param.page_no,
                page_size: param.page_size,
                records,
                total_page: total_count / param.page_size + 1,
                filters: param.filters,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::groups;
    use diesel::QueryDsl;
    use diesel::debug_query;

    fn cond(column: &str, compare: Compare, value: CompareValue) -> Filter {
        Filter::Condition(Condition {
            column: column.to_string(),
            compare: Some(compare),
            compare_value: value,
        })
    }

    fn sql_of(filter: Filter) -> String {
        let stmt: BoxedSelectStatement<'_, _, FromClause<groups::table>, DbType> =
            groups::table.into_boxed();
        let table = diesel_dynamic_schema::table("groups");
        let stmt = filter.append_to_sql(stmt, &table);
        debug_query::<DbType, _>(&stmt).to_string()
    }

    #[test]
    fn empty_and_adds_no_where() {
        let sql = sql_of(Filter::And(vec![]));
        assert!(!sql.to_lowercase().contains("where"), "got: {sql}");
    }

    #[test]
    fn empty_or_adds_no_where() {
        let sql = sql_of(Filter::Or(vec![]));
        assert!(!sql.to_lowercase().contains("where"), "got: {sql}");
    }

    #[test]
    fn single_condition_eq_string() {
        let sql = sql_of(cond(
            "name",
            Compare::Equal,
            CompareValue::String("alice".into()),
        ));
        println!("{}", sql);
        assert!(sql.contains("\"name\" = $1"), "got: {sql}");
        assert!(sql.contains("alice"), "got: {sql}");
    }

    #[test]
    fn single_condition_gt_float() {
        let sql = sql_of(cond("score", Compare::Greater, CompareValue::Float(1.5)));
        assert!(sql.contains("\"score\" > $1"), "got: {sql}");
    }

    #[test]
    fn and_combines_with_and() {
        let sql = sql_of(Filter::And(vec![
            cond("name", Compare::Equal, CompareValue::String("a".into())),
            cond("is_delete", Compare::NotEqual, CompareValue::Bool(true)),
        ]));
        println!("{}", sql);
        let lower = sql.to_lowercase();
        assert!(lower.contains(" and "), "expected AND, got: {sql}");
        assert!(sql.contains("\"name\""));
        assert!(sql.contains("\"is_delete\""));
    }

    #[test]
    fn or_combines_with_or() {
        let sql = sql_of(Filter::Or(vec![
            cond("name", Compare::Equal, CompareValue::String("a".into())),
            cond("name", Compare::Equal, CompareValue::String("b".into())),
        ]));
        println!("{}", sql);

        assert!(sql.to_lowercase().contains(" or "), "got: {sql}");
    }

    #[test]
    fn nested_and_of_or_preserves_grouping() {
        // A AND (B OR C)  =>  SQL must wrap the OR in parentheses
        let filter = Filter::And(vec![
            cond("name", Compare::Equal, CompareValue::String("a".into())),
            Filter::Or(vec![
                cond("is_delete", Compare::Equal, CompareValue::Bool(false)),
                cond("is_delete", Compare::Equal, CompareValue::Bool(true)),
            ]),
        ]);
        let sql = sql_of(filter);
        println!("{}", sql);

        let lower = sql.to_lowercase();
        assert!(lower.contains(" and "), "got: {sql}");
        assert!(lower.contains(" or "), "got: {sql}");
        // The OR sub-expression must be parenthesized so AND doesn't bind into it.
        // Expected shape: `... AND (... OR ...)` — so " AND (" must appear before " OR ".
        let and_pos = lower
            .find(" and (")
            .expect(&format!("missing ` AND (`, got: {sql}"));
        let or_pos = lower
            .find(" or ")
            .expect(&format!("missing ` OR `, got: {sql}"));
        assert!(
            and_pos < or_pos,
            "AND should appear before grouped OR, got: {sql}"
        );
    }
}
