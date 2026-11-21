use tauri::{AppHandle, Emitter, Manager, menu::MenuEvent};

mod build;
pub mod launcher;
mod menu_id;
pub mod portfolio;
mod spec;

pub use build::build;

use crate::menu::menu_id::MenuId;
use std::sync::Mutex;

#[derive(Default)]
pub struct ActiveWindow(Mutex<Option<String>>);

impl ActiveWindow {
    pub fn set(&self, label: &str) {
        *self.0.lock().unwrap() = Some(label.to_string());
    }
    fn get(&self) -> Option<String> {
        self.0.lock().unwrap().clone()
    }
}

pub fn handle_event(app: &AppHandle, event: MenuEvent) {
    if event.id().as_ref() == MenuId::FileOpenLauncher.as_str() {
        let app = app.clone();
        tauri::async_runtime::spawn(async move {
            let _ = crate::views::launcher::open_launcher(&app);
        });
        return;
    }

    let Some(label) = app.state::<ActiveWindow>().get() else {
        return;
    };

    let name = format!("menu:{}", event.id().as_ref());
    if let Err(error) = app.emit_to(label.as_str(), &name, ()) {
        eprintln!("Failed to emit {name}: {error}");
    }
}
