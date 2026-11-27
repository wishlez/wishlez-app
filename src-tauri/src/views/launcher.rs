use super::focus_window;
use crate::menu;
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

pub(super) const LAUNCHER: &str = "launcher";

pub fn open_launcher(app: &AppHandle) -> tauri::Result<()> {
    if let Some(window) = app.get_webview_window(LAUNCHER) {
        return focus_window(&window);
    }

    let menu = menu::build(app, &menu::launcher::menu())?;

    let _window =
        WebviewWindowBuilder::new(app, LAUNCHER, WebviewUrl::App("src/launcher.html".into()))
            .title("Wishlez")
            .inner_size(800.0, 600.0)
            .min_inner_size(320.0, 480.0)
            .build()?;

    #[cfg(target_os = "macos")]
    app.set_menu(menu)?;

    #[cfg(not(target_os = "macos"))]
    _window.set_menu(None)?;

    Ok(())
}
