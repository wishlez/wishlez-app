use super::OpenPortfolios;
use crate::data::portfolio::Portfolio;
use crate::error::WishlezError;
use std::sync::{Arc, Mutex};
use tauri::ipc::{CommandArg, CommandItem, InvokeError};
use tauri::{Manager, Runtime};

pub struct CurrentPortfolio(Arc<Mutex<Portfolio>>);

impl<'deserialize, RT: Runtime> CommandArg<'deserialize, RT> for CurrentPortfolio {
    fn from_command(command: CommandItem<'deserialize, RT>) -> Result<Self, InvokeError> {
        let webview = command.message.webview();

        webview
            .state::<OpenPortfolios>()
            .get(webview.label())
            .map(CurrentPortfolio)
            .ok_or_else(|| {
                InvokeError::from(WishlezError::GeneralError {
                    code: "0006".to_string(),
                    message: format!(
                        "Unable to access portfolio for window '{}'",
                        webview.label()
                    ),
                })
            })
    }
}

impl CurrentPortfolio {
    pub fn with_tx<T>(
        &self,
        f: impl FnOnce(&rusqlite::Transaction) -> Result<T, WishlezError>,
    ) -> Result<T, WishlezError> {
        self.0.lock().unwrap().with_tx(f)
    }
}
