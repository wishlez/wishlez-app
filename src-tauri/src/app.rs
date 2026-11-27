use crate::views::{launcher, portfolio};
use crate::{state, views};
use tauri::Manager;

fn get_portfolio_path(args: impl IntoIterator<Item = String>) -> Option<String> {
    args.into_iter().skip(1).find(|arg| {
        std::path::Path::new(arg)
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("wfolio"))
    })
}

pub fn handle_singleton(app: &tauri::AppHandle, args: Vec<String>, cwd: String) {
    let app = app.clone();

    tauri::async_runtime::spawn(async move {
        let _ = match get_portfolio_path(args) {
            Some(path) => {
                let path = std::path::Path::new(&cwd)
                    .join(path)
                    .to_string_lossy()
                    .into_owned();

                portfolio::open(&app, path)
            }
            None => launcher::open_launcher(&app),
        };
    });
}

pub fn handle_runtime_events(app: &tauri::AppHandle, event: tauri::RunEvent) {
    #[cfg(target_os = "macos")]
    if let tauri::RunEvent::Reopen {
        has_visible_windows: false,
        ..
    } = &event
    {
        let app = app.clone();
        tauri::async_runtime::spawn(async move {
            let _ = launcher::open_launcher(&app);
        });
    }

    if let tauri::RunEvent::ExitRequested {
        code: None, api, ..
    } = event
        && app.state::<state::ReopenLauncher>().take_request()
    {
        api.prevent_exit();
        let app = app.clone();
        tauri::async_runtime::spawn(async move {
            let _ = launcher::open_launcher(&app);
        });
    }
}

pub fn setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    match get_portfolio_path(std::env::args()) {
        Some(path) => views::portfolio::open(app.handle(), path)?,
        None => launcher::open_launcher(app.handle())?,
    }

    Ok(())
}
