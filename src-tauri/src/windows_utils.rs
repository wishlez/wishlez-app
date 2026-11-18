use crate::windows_states::OpenPortfolios;
use tauri::{AppHandle, Manager};

pub fn focus_window(window: &tauri::WebviewWindow) -> tauri::Result<()> {
    window.unminimize()?;
    window.set_always_on_top(true)?;
    window.set_focus()?;
    window.set_always_on_top(false)
}

pub fn get_portfolio_path(args: impl IntoIterator<Item = String>) -> Option<String> {
    args.into_iter().skip(1).find(|arg| {
        std::path::Path::new(arg)
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("wfolio"))
    })
}

pub fn find_open_portfolio_label(app: &tauri::AppHandle, path: &str) -> Option<String> {
    let target = std::fs::canonicalize(path).ok()?;
    app.state::<OpenPortfolios>()
        .0
        .lock()
        .ok()?
        .iter()
        .find(|(_, (open_path, _))| std::fs::canonicalize(open_path).ok().as_ref() == Some(&target))
        .map(|(label, _)| label.clone())
}

pub fn focus_window_if_already_open(app: &AppHandle, path: &str) -> tauri::Result<bool> {
    if let Some(window) =
        find_open_portfolio_label(app, path).and_then(|label| app.get_webview_window(&label))
    {
        return focus_window(&window).map(|_| true);
    }

    Ok(false)
}
