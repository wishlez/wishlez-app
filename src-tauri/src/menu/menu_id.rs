#[derive(serde::Serialize, ts_rs::TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum MenuId {
    FileCreate,
    FileOpen,
    FileOpenLauncher,
    AccountList,
    AccountCreate,
}

impl MenuId {
    pub fn as_str(&self) -> String {
        let value = serde_json::to_value(self).expect("MenuId serialization failed");

        serde_json::from_value(value).expect("MenuId must serialize as a string")
    }
}
