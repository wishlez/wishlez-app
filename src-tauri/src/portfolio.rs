use crate::WishlezError;
use crate::account::create_account_with_tx;
use crate::account_models::{BuiltinAccountId, CreateOrUpdateAccountRequest};
use crate::account_type_models::AccountType;
use crate::sql::*;
use rusqlite::Connection;

const SCHEMA_VERSION: i32 = 1;

pub struct Portfolio {
    conn: Connection,
}

impl Portfolio {
    pub fn create(path: &str) -> Result<Self, WishlezError> {
        if std::path::Path::new(path).exists() {
            std::fs::remove_file(path)?;
        }

        let mut portfolio = Self::connect(path)?;

        portfolio.with_tx(|tx| {
            tx.pragma_update(None, "user_version", SCHEMA_VERSION)?;
            tx.execute(CREATE_ACCOUNTS, [])?;
            tx.execute(CREATE_JOURNALS, [])?;
            tx.execute(CREATE_TAGS, [])?;
            tx.execute(CREATE_TAGS_FEEDBACK, [])?;
            tx.execute(CREATE_TRANSACTIONS, [])?;
            tx.execute(CREATE_TRANSACTIONS_TAGS, [])?;

            create_account_with_tx(
                tx,
                CreateOrUpdateAccountRequest {
                    id: Some(BuiltinAccountId::CashExpenses.as_str()),
                    name: "Cash Expenses".to_string(),
                    account_type: AccountType::Expense,
                    is_built_in: true,
                },
            )?;
            create_account_with_tx(
                tx,
                CreateOrUpdateAccountRequest {
                    id: Some(BuiltinAccountId::CashIncomes.as_str()),
                    name: "Cash Incomes".to_string(),
                    account_type: AccountType::Income,
                    is_built_in: true,
                },
            )?;
            create_account_with_tx(
                tx,
                CreateOrUpdateAccountRequest {
                    id: Some(BuiltinAccountId::OpeningBalances.as_str()),
                    name: "Opening Balances".to_string(),
                    account_type: AccountType::Equity,
                    is_built_in: true,
                },
            )?;

            Ok(())
        })?;

        Ok(portfolio)
    }

    pub fn open(path: &str) -> Result<Self, WishlezError> {
        let mut portfolio = Self::connect(path)?;

        let version: i32 = portfolio
            .with_tx(|tx| Ok(tx.query_row("PRAGMA user_version", [], |row| row.get(0))?))?;

        if version != SCHEMA_VERSION {
            return Err(WishlezError::GeneralError {
                code: "0004".to_string(),
                message: format!("Not a Wishlez portfolio (version {version})"),
            });
        }

        Ok(portfolio)
    }

    pub fn with_tx<T>(
        &mut self,
        f: impl FnOnce(&rusqlite::Transaction) -> Result<T, WishlezError>,
    ) -> Result<T, WishlezError> {
        let tx = self.conn.transaction()?;
        let result = f(&tx)?;
        tx.commit()?;

        Ok(result)
    }

    fn connect(path: &str) -> Result<Self, WishlezError> {
        let conn = Connection::open(path)?;
        conn.pragma_update(None, "foreign_keys", true)?;
        Ok(Self { conn })
    }
}

#[tauri::command]
pub async fn create_portfolio(app: tauri::AppHandle, path: String) -> Result<(), WishlezError> {
    let portfolio = Portfolio::create(&path)?;
    crate::windows::open_portfolio(&app, path, portfolio)?;
    Ok(())
}

#[tauri::command]
pub async fn access_portfolio(app: tauri::AppHandle, path: String) -> Result<(), WishlezError> {
    let portfolio = Portfolio::open(&path)?;
    crate::windows::open_portfolio(&app, path, portfolio)?;
    Ok(())
}
