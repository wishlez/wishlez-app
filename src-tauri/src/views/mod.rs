use crate::{menu, state};
use tauri::Manager;

pub mod launcher;
pub mod portfolio;

fn focus_window(window: &tauri::WebviewWindow) -> tauri::Result<()> {
    window.unminimize()?;
    window.set_always_on_top(true)?;
    window.set_focus()?;
    window.set_always_on_top(false)
}

pub fn handle_event(window: &tauri::Window, event: &tauri::WindowEvent) {
    match event {
        tauri::WindowEvent::Focused(true) => {
            window
                .app_handle()
                .state::<menu::ActiveWindow>()
                .set(window.label());
        }
        tauri::WindowEvent::Destroyed if window.label().starts_with("portfolio-") => {
            let app = window.app_handle();

            if app
                .state::<state::OpenPortfolios>()
                .remove_and_check_empty(window.label())
            {
                app.state::<state::ReopenLauncher>().request();
            }
        }
        _ => {}
    }
}
