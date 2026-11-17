use super::spec::{Entry, MenuSpec, Predefined};
use tauri::menu::{IsMenuItem, Menu, MenuItem, PredefinedMenuItem, Submenu};
use tauri::{AppHandle, Wry};

type Item = Box<dyn IsMenuItem<Wry>>;

fn make_entry(app: &AppHandle, entry: &Entry) -> tauri::Result<Item> {
    Ok(match entry {
        Entry::Item {
            id,
            label,
            shortcut,
        } => Box::new(MenuItem::with_id(app, *id, *label, true, *shortcut)?),
        Entry::Separator => Box::new(PredefinedMenuItem::separator(app)?),
        Entry::Predefined(kind) => Box::new(match kind {
            Predefined::Quit => PredefinedMenuItem::quit(app, None)?,
            Predefined::Close => PredefinedMenuItem::close_window(app, None)?,
            // Predefined::Undo => PredefinedMenuItem::undo(app, None)?,
            // Predefined::Redo => PredefinedMenuItem::redo(app, None)?,
            // Predefined::Cut => PredefinedMenuItem::cut(app, None)?,
            // Predefined::Copy => PredefinedMenuItem::copy(app, None)?,
            // Predefined::Paste => PredefinedMenuItem::paste(app, None)?,
            // Predefined::SelectAll => PredefinedMenuItem::select_all(app, None)?,
        }),
    })
}

fn is_macos_app_level(entry: &Entry) -> bool {
    matches!(entry, Entry::Predefined(Predefined::Quit))
}

pub fn build(app: &AppHandle, spec: &MenuSpec) -> tauri::Result<Menu<Wry>> {
    let mut submenus: Vec<Submenu<Wry>> = Vec::new();

    if cfg!(target_os = "macos") {
        submenus.push(app_submenu(app)?);
    }

    for section in &spec.0 {
        let mut entries: Vec<&Entry> = section.entries.iter().collect();
        if cfg!(target_os = "macos") {
            entries.retain(|e| !is_macos_app_level(e));
            while matches!(entries.last(), Some(Entry::Separator)) {
                entries.pop();
            }
        }
        if entries.is_empty() {
            continue;
        }

        let items = entries
            .into_iter()
            .map(|e| make_entry(app, e))
            .collect::<tauri::Result<Vec<Item>>>()?;
        let refs: Vec<&dyn IsMenuItem<Wry>> = items.iter().map(|i| i.as_ref()).collect();
        submenus.push(Submenu::with_items(app, section.title, true, &refs)?);
    }

    let refs: Vec<&dyn IsMenuItem<Wry>> = submenus.iter().map(|s| s as _).collect();
    Menu::with_items(app, &refs)
}

#[cfg(target_os = "macos")]
fn app_submenu(app: &AppHandle) -> tauri::Result<Submenu<Wry>> {
    let sep = PredefinedMenuItem::separator(app)?;
    Submenu::with_items(
        app,
        &app.package_info().name,
        true,
        &[
            &PredefinedMenuItem::about(app, None, None)?,
            &sep,
            &PredefinedMenuItem::services(app, None)?,
            &sep,
            &PredefinedMenuItem::hide(app, None)?,
            &PredefinedMenuItem::hide_others(app, None)?,
            &sep,
            &PredefinedMenuItem::quit(app, None)?,
        ],
    )
}

#[cfg(not(target_os = "macos"))]
fn app_submenu(_app: &AppHandle) -> tauri::Result<Submenu<Wry>> {
    unreachable!("only called on macOS")
}
