use super::menu_id::MenuId;
use super::spec::{Entry, MenuSpec, Predefined, item, section};

pub fn menu() -> MenuSpec {
    MenuSpec(vec![section(
        "File",
        vec![
            item(MenuId::FileCreate, "New"),
            item(MenuId::FileOpen, "Open"),
            Entry::Separator,
            Entry::Predefined(Predefined::Quit),
        ],
    )])
}
