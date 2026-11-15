use tauri::Manager;

mod __utils__;
mod account;
mod account_models;
mod account_type_models;
mod menu;
mod portfolio;
mod sql;

pub use __utils__::WishlezError;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(menu::ActiveWindow::default())
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::Focused(true) = event {
                window
                    .app_handle()
                    .state::<menu::ActiveWindow>()
                    .set(window.label());
            }
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let menu = menu::build(app.handle(), &menu::launcher::menu())?;
            if let Some(window) = app.get_webview_window("main") {
                window.set_menu(menu)?;
            }
            Ok(())
        })
        .on_menu_event(menu::handle_event)
        .invoke_handler(tauri::generate_handler![
            portfolio::create_portfolio,
            portfolio::access_portfolio,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
