use crate::WishlezError;
use crate::account_models::*;
use crate::sql::*;
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
