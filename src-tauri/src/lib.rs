use tauri::Manager;

mod account;
mod account_models;
mod account_type_models;
mod error;
mod menu;
mod portfolio;
mod sql;
mod windows;
mod windows_states;
mod windows_utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(menu::ActiveWindow::default())
        .plugin(tauri_plugin_single_instance::init(|app, args, cwd| {
            let app = app.clone();
            tauri::async_runtime::spawn(async move {
                let _ = match windows_utils::get_portfolio_path(args) {
                    Some(path) => {
                        let path = std::path::Path::new(&cwd)
                            .join(path)
                            .to_string_lossy()
                            .into_owned();
                        windows::open_portfolio_path(&app, path)
                    }
                    None => windows::open_launcher(&app),
                };
            });
        }))
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            match windows_utils::get_portfolio_path(std::env::args()) {
                Some(path) => windows::open_portfolio_path(app.handle(), path)?,
                None => windows::open_launcher(app.handle())?,
            }

            Ok(())
        })
        .on_menu_event(menu::handle_event)
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::Focused(true) => {
                window
                    .app_handle()
                    .state::<menu::ActiveWindow>()
                    .set(window.label());
            }
            tauri::WindowEvent::Destroyed if window.label().starts_with("portfolio-") => {
                let app = window.app_handle();
                let portfolios = app.state::<windows_states::OpenPortfolios>();
                let mut open = portfolios.0.lock().unwrap();
                open.remove(window.label());
                if open.is_empty() {
                    app.state::<windows_states::ReopenLauncher>()
                        .0
                        .store(true, std::sync::atomic::Ordering::SeqCst);
                }
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            portfolio::create_portfolio,
            portfolio::access_portfolio,
            account::read_accounts,
        ])
        .manage(windows_states::OpenPortfolios(Default::default()))
        .manage(windows_states::ReopenLauncher(Default::default()))
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app, event| {
            if let tauri::RunEvent::ExitRequested {
                code: None, api, ..
            } = event
                && app
                    .state::<windows_states::ReopenLauncher>()
                    .0
                    .swap(false, std::sync::atomic::Ordering::SeqCst)
            {
                api.prevent_exit();
                let app = app.clone();
                tauri::async_runtime::spawn(async move {
                    let _ = windows::open_launcher(&app);
                });
            }
        });
}
