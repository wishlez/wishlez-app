use thiserror::Error;

#[derive(Debug, Error, serde::Serialize)]
#[serde(untagged)]
pub enum WishlezError {
    #[error("General error {code}: {message}")]
    GeneralError { code: String, message: String },

    #[error("Database error {code}: {message}")]
    DatabaseError { code: String, message: String },
}

impl From<std::io::Error> for WishlezError {
    fn from(e: std::io::Error) -> Self {
        WishlezError::GeneralError {
            code: "0001".to_string(),
            message: e.to_string(),
        }
    }
}

impl From<rusqlite::Error> for WishlezError {
    fn from(e: rusqlite::Error) -> Self {
        WishlezError::DatabaseError {
            code: "0002".to_string(),
            message: e.to_string(),
        }
    }
}

impl From<tauri::Error> for WishlezError {
    fn from(e: tauri::Error) -> Self {
        WishlezError::GeneralError {
            code: "0005".to_string(),
            message: e.to_string(),
        }
    }
}
