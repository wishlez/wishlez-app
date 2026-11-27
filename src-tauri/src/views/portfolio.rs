use super::{focus_window, launcher};
use crate::data::portfolio::Portfolio;
use crate::error::WishlezError;
use crate::menu;
use crate::state::OpenPortfolios;
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

fn focus_window_if_already_open(app: &AppHandle, path: &str) -> tauri::Result<bool> {
    if let Some(window) = app
        .state::<OpenPortfolios>()
        .get_label_from_path(path)
        .and_then(|label| app.get_webview_window(&label))
    {
        return focus_window(&window).map(|_| true);
    }

    Ok(false)
}

fn open_portfolio(app: &AppHandle, path: String, portfolio: Portfolio) -> tauri::Result<()> {
    let label = format!("portfolio-{}", uuid::Uuid::now_v7());

    app.state::<OpenPortfolios>()
        .insert(&label, &path, portfolio);

    let result =
        WebviewWindowBuilder::new(app, label.clone(), WebviewUrl::App("index.html".into()))
            .title(format!("Wishlez - {}", path))
            .inner_size(1200.0, 800.0)
            .min_inner_size(320.0, 480.0)
            .build();

    let window = match result {
        Ok(window) => window,
        Err(error) => {
            app.state::<OpenPortfolios>().remove(label.as_str());
            return Err(error);
        }
    };

    let menu = menu::build(app, &menu::portfolio::menu())?;

    #[cfg(target_os = "macos")]
    app.set_menu(menu)?;

    #[cfg(not(target_os = "macos"))]
    window.set_menu(menu)?;

    focus_window(&window)?;

    if let Some(launcher) = app.get_webview_window(launcher::LAUNCHER) {
        launcher.close()?;
    }

    Ok(())
}

pub fn open(app: &AppHandle, path: String) -> tauri::Result<()> {
    if focus_window_if_already_open(app, &path)? {
        return Ok(());
    }

    match Portfolio::open(&path) {
        Ok(portfolio) => open_portfolio(app, path, portfolio),
        Err(error) => {
            eprintln!("Failed to open portfolio at {path}: {error}");
            launcher::open_launcher(app)
        }
    }
}

#[tauri::command]
pub async fn create_portfolio(app: AppHandle, path: String) -> Result<(), WishlezError> {
    let portfolio = Portfolio::create(&path)?;
    open_portfolio(&app, path, portfolio)?;
    Ok(())
}

#[tauri::command]
pub async fn access_portfolio(app: AppHandle, path: String) -> Result<(), WishlezError> {
    if focus_window_if_already_open(&app, &path)? {
        return Ok(());
    }
    let portfolio = Portfolio::open(&path)?;
    open_portfolio(&app, path, portfolio)?;
    Ok(())
}
