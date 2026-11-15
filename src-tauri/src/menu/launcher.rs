use super::spec::{Entry, MenuSpec, Predefined, item, section};

pub fn menu() -> MenuSpec {
    MenuSpec(vec![section(
        "File",
        vec![
            item("file_create", "New"),
            item("file_open", "Open"),
            Entry::Separator,
            Entry::Predefined(Predefined::Quit),
        ],
    )])
}
