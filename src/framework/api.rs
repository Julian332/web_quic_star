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

const _LOGIN_URL: &str = "/auth/login";

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

#[derive(Deserialize, Serialize, JsonSchema, Clone, Default)]
pub enum BoolOp {
    #[default]
    And,
    Or,
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

// #[derive(Deserialize, Serialize, JsonSchema, Clone, Default)]
// pub struct Filter {
//     pub compare: Compare,
//     pub compare_value: Value,
// }

#[derive(Deserialize, Serialize, JsonSchema, Clone)]
pub struct DynFilter {
    pub column: String,
    pub op: Option<BoolOp>,
    pub compare: Option<Compare>,
    pub compare_value: CompareValue,
}
#[derive(Deserialize, Serialize, JsonSchema, Clone)]
pub enum CompareValue {
    BigDecimal(Decimal),
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
