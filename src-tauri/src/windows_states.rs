use crate::WishlezError;
use crate::portfolio::Portfolio;
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use tauri::ipc::{CommandArg, CommandItem, InvokeError};
use tauri::{Manager, Runtime};

pub type PortfolioEntry = (String, Arc<Mutex<Portfolio>>);
pub struct OpenPortfolios(pub Mutex<HashMap<String, PortfolioEntry>>);

#[derive(Default)]
pub struct ReopenLauncher(pub AtomicBool);

pub struct CurrentPortfolio(Arc<Mutex<Portfolio>>);

impl<'deserialize, RT: Runtime> CommandArg<'deserialize, RT> for CurrentPortfolio {
    fn from_command(command: CommandItem<'deserialize, RT>) -> Result<Self, InvokeError> {
        let webview = command.message.webview();

        webview
            .state::<OpenPortfolios>()
            .0
            .lock()
            .unwrap()
            .get(webview.label())
            .map(|(_, portfolio)| CurrentPortfolio(portfolio.clone()))
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
