use crate::account_models::*;
use crate::error::WishlezError;
use crate::sql::*;
use crate::windows_states::CurrentPortfolio;
use rusqlite::named_params;

pub fn create_account_with_tx(
    tx: &rusqlite::Transaction,
    request: CreateOrUpdateAccountRequest,
) -> Result<(), WishlezError> {
    tx.execute(
        INSERT_ACCOUNT,
        named_params! {
            ":id": &request.id,
            ":name": &request.name,
            ":account_type": &request.account_type,
            ":is_built_in": &request.is_built_in,
        },
    )?;

    Ok(())
}

#[tauri::command]
pub async fn read_accounts(portfolio: CurrentPortfolio) -> Result<Vec<Account>, WishlezError> {
    portfolio.with_tx(|tx| {
        let mut stmt = tx.prepare(SELECT_ACCOUNTS)?;
        let accounts = stmt
            .query_map([], |row| Account::try_from(row))?
            .collect::<Result<Vec<Account>, _>>()?;

        Ok(accounts)
    })
}
