use thiserror::Error;

#[derive(Debug, Error)]
pub enum WishlezError {
    #[error("Database error: {message}")]
    DatabaseError { message: String },

    #[error("General error: {message}")]
    GeneralError { message: String },
}

impl From<rusqlite::Error> for WishlezError {
    fn from(e: rusqlite::Error) -> Self {
        WishlezError::DatabaseError {
            message: e.to_string(),
        }
    }
}

impl From<std::io::Error> for WishlezError {
    fn from(e: std::io::Error) -> Self {
        WishlezError::GeneralError {
            message: e.to_string(),
        }
    }
}

impl serde::Serialize for WishlezError {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}
