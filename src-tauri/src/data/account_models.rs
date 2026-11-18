use super::account_type_models::*;
use rusqlite::Row;

#[derive(serde::Serialize, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Account {
    pub id: String,
    pub name: String,
    pub account_type: AccountType,
    pub is_built_in: bool,
}

impl TryFrom<&Row<'_>> for Account {
    type Error = rusqlite::Error;

    fn try_from(row: &Row) -> Result<Self, Self::Error> {
        Ok(Account {
            id: row.get("id")?,
            name: row.get("name")?,
            account_type: row.get("account_type")?,
            is_built_in: row.get("is_built_in")?,
        })
    }
}

pub enum BuiltinAccountId {
    CashExpenses,
    CashIncomes,
    OpeningBalances,
}

impl BuiltinAccountId {
    pub fn as_str(&self) -> String {
        match self {
            BuiltinAccountId::CashExpenses => "builtin-cash-expenses".to_string(),
            BuiltinAccountId::CashIncomes => "builtin-cash-incomes".to_string(),
            BuiltinAccountId::OpeningBalances => "builtin-opening-balances".to_string(),
        }
    }
}

pub struct CreateOrUpdateAccountRequest {
    pub id: Option<String>,
    pub name: String,
    pub account_type: AccountType,
    pub is_built_in: bool,
}
