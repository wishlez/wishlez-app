use super::spec::{Entry, MenuSpec, Predefined, item, section};

pub fn menu() -> MenuSpec {
    MenuSpec(vec![section(
        "File",
        vec![
            item("file_create", "New"),
            item("file_open", "Open"),
            Entry::Separator,
            item("file_open_launcher", "Open Launcher"),
            Entry::Separator,
            Entry::Predefined(Predefined::Close),
            Entry::Predefined(Predefined::Quit),
        ],
    )])
}
