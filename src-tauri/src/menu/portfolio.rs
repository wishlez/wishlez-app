use super::menu_id::MenuId;
use super::spec::{Entry, MenuSpec, Predefined, item, section};

pub fn menu() -> MenuSpec {
    MenuSpec(vec![
        section(
            "File",
            vec![
                item(MenuId::FileCreate, "New"),
                item(MenuId::FileOpen, "Open"),
                Entry::Separator,
                item(MenuId::FileOpenLauncher, "Open Launcher"),
                Entry::Separator,
                Entry::Predefined(Predefined::Close),
                Entry::Predefined(Predefined::Quit),
            ],
        ),
        section(
            "Accounts",
            vec![
                item(MenuId::AccountList, "View Accounts"),
                item(MenuId::AccountCreate, "Create Account..."),
            ],
        ),
    ])
}
