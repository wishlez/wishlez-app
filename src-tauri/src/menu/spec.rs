pub struct MenuSpec(pub Vec<Section>);

pub struct Section {
    pub title: &'static str,
    pub entries: Vec<Entry>,
}

pub enum Entry {
    Item {
        id: &'static str,
        label: &'static str,
        shortcut: Option<&'static str>,
    },
    Separator,
    Predefined(Predefined),
}

pub enum Predefined {
    Quit,
    // Close,
    // Undo,
    // Redo,
    // Cut,
    // Copy,
    // Paste,
    // SelectAll,
}

pub fn section(title: &'static str, entries: Vec<Entry>) -> Section {
    Section { title, entries }
}

pub fn item(id: &'static str, label: &'static str) -> Entry {
    Entry::Item {
        id,
        label,
        shortcut: None,
    }
}

// TODO:
// impl Entry {
//     pub fn shortcut(self, shortcut: &'static str) -> Self {
//         match self {
//             Entry::Item { id, label, .. } => Entry::Item {
//                 id,
//                 label,
//                 shortcut: Some(shortcut),
//             },
//             other => other,
//         }
//     }
// }
