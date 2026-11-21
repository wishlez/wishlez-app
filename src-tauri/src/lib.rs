mod app;
mod data;
mod error;
mod menu;
mod state;
mod traits;
mod views;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        //
        // init states
        //
        .manage(menu::ActiveWindow::default())
        .manage(state::OpenPortfolios::default())
        .manage(state::ReopenLauncher::default())
        //
        // plugins
        //
        .plugin(tauri_plugin_single_instance::init(app::handle_singleton))
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        //
        // events
        //
        .on_menu_event(menu::handle_event)
        .on_window_event(views::handle_event)
        //
        // frontend handlers
        //
        .invoke_handler(tauri::generate_handler![
            views::portfolio::create_portfolio,
            views::portfolio::access_portfolio,
            data::account::create_account,
            data::account::read_accounts,
        ])
        //
        // application build
        //
        .setup(app::setup)
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        //
        // application run
        //
        .run(app::handle_runtime_events);
}
