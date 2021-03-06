use serde_json::Value;

use rbatis_core::convert::StmtConvert;
use rbatis_core::db::DriverType;

use crate::sql::Date;

impl Date for DriverType {
    fn date_convert(&self, value: &Value, index: usize) -> rbatis_core::Result<(String, Value)> {
        let mut sql = String::new();
        match self {
            DriverType::Postgres => {
                sql = format!("cast({} as timestamp)", self.stmt_convert(index).as_str());
            }
            _ => {
                sql = self.stmt_convert(index);
            }
        }
        return Ok((sql, value.to_owned()));
    }
}

#[test]
pub fn test_date() {}