use crate::portfolio::Portfolio;
use crate::windows_states::OpenPortfolios;
use crate::windows_utils::{focus_window, focus_window_if_already_open};
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

pub const LAUNCHER: &str = "launcher";

pub fn open_launcher(app: &AppHandle) -> tauri::Result<()> {
    if let Some(window) = app.get_webview_window(LAUNCHER) {
        return focus_window(&window);
    }

    let window = WebviewWindowBuilder::new(app, LAUNCHER, WebviewUrl::App("launcher.html".into()))
        .title("Wishlez")
        .inner_size(800.0, 600.0)
        .min_inner_size(320.0, 480.0)
        .build()?;

    window.set_menu(crate::menu::build(app, &crate::menu::launcher::menu())?)?;
    Ok(())
}

pub fn open_portfolio(app: &AppHandle, path: String, portfolio: Portfolio) -> tauri::Result<()> {
    let label = format!("portfolio-{}", uuid::Uuid::now_v7());

    app.state::<OpenPortfolios>().0.lock().unwrap().insert(
        label.clone(),
        (path.clone(), Arc::new(Mutex::new(portfolio))),
    );

    let result =
        WebviewWindowBuilder::new(app, label.clone(), WebviewUrl::App("index.html".into()))
            .title(format!("Wishlez - {}", path))
            .inner_size(1200.0, 800.0)
            .min_inner_size(320.0, 480.0)
            .build();

    let window = match result {
        Ok(window) => window,
        Err(error) => {
            app.state::<OpenPortfolios>()
                .0
                .lock()
                .unwrap()
                .remove(&label);
            return Err(error);
        }
    };

    window.set_menu(crate::menu::build(app, &crate::menu::portfolio::menu())?)?;
    focus_window(&window)?;

    if let Some(launcher) = app.get_webview_window(LAUNCHER) {
        launcher.close()?;
    }

    Ok(())
}

pub fn open_portfolio_path(app: &AppHandle, path: String) -> tauri::Result<()> {
    if focus_window_if_already_open(app, &path)? {
        return Ok(());
    }

    match Portfolio::open(&path) {
        Ok(portfolio) => open_portfolio(app, path, portfolio),
        Err(error) => {
            eprintln!("Failed to open portfolio at {path}: {error}");
            open_launcher(app)
        }
    }
}
