use tauri::{AppHandle, Emitter, Manager, menu::MenuEvent};

mod build;
pub mod launcher;
mod spec;

pub use build::build;

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
    let Some(label) = app.state::<ActiveWindow>().get() else {
        return;
    };

    let name = format!("menu:{}", event.id().as_ref());
    if let Err(error) = app.emit_to(label.as_str(), &name, ()) {
        eprintln!("Failed to emit {name}: {error}");
    }
}
