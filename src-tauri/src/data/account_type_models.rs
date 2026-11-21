use rusqlite::types::{FromSql, FromSqlResult, ToSql, ToSqlOutput, ValueRef};

#[derive(Clone, serde::Serialize, serde::Deserialize, ts_rs::TS)]
#[ts(export)]
pub enum AccountType {
    Expense,
    Income,
    Liability,
    Asset,
    Equity,
}

impl ToSql for AccountType {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        let value = match self {
            AccountType::Expense => "Expense",
            AccountType::Income => "Income",
            AccountType::Liability => "Liability",
            AccountType::Asset => "Asset",
            AccountType::Equity => "Equity",
        };

        Ok(ToSqlOutput::from(value))
    }
}

impl FromSql for AccountType {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value.as_str()? {
            "Expense" => Ok(AccountType::Expense),
            "Income" => Ok(AccountType::Income),
            "Liability" => Ok(AccountType::Liability),
            "Asset" => Ok(AccountType::Asset),
            "Equity" => Ok(AccountType::Equity),
            _ => Err(rusqlite::types::FromSqlError::InvalidType),
        }
    }
}
