use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct APIResponse<T>
// where
//     T: Serialize,
// {
//     pub data: Option<T>,
//     pub error: Option<Error>,
// }

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct APIResult<T>(pub Result<T>)
// where
//     T: Serialize;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Error {
    code: ErrorCode,
    note: String,
    meta: HashMap<String, String>,
}

impl Error {
    pub fn new(note: &str, code: ErrorCode) -> Self {
        Self {
            note: (if !note.is_empty() {
                note
            } else {
                match code {
                    ErrorCode::Tech => "A technical issue occured. Please try again later.",
                    ErrorCode::User => "An issue occured due to your input.",
                    ErrorCode::NotFound => "The information you seek was not found.",
                }
            })
            .to_string(),
            code,
            meta: HashMap::new(),
        }
    }

    pub fn user(note: &str) -> Self {
        Self::new(note, ErrorCode::User)
    }

    pub fn tech(note: &str) -> Self {
        Self::new(note, ErrorCode::Tech)
    }

    pub fn notfound(note: &str) -> Self {
        Self::new(note, ErrorCode::NotFound)
    }

    pub fn add_meta(&mut self, key: &str, val: &str) {
        self.meta.insert(key.to_string(), val.to_string());
    }

    pub fn add_meta_x(mut self, key: &str, val: &str) -> Self {
        self.meta.insert(key.to_string(), val.to_string());
        self
    }

    pub fn has_meta(&self) -> bool {
        self.meta.len() > 0
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ErrorCode {
    Tech,
    User,
    NotFound,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

// impl<T> IntoResponse for APIResult<T>
// where
//     T: Serialize,
// {
//     fn into_response(self) -> Response {
//         if let Err(e) = self.0 {
//             e.log();

//             let status_code = match &e.code {
//                 ErrorCode::NotFound => StatusCode::NOT_FOUND,
//                 ErrorCode::User => StatusCode::BAD_REQUEST,
//                 ErrorCode::Tech => StatusCode::INTERNAL_SERVER_ERROR,
//             };

//             let data = Json(APIResponse::<T> {
//                 data: None,
//                 error: Some(e),
//             });

//             return (status_code, data).into_response();
//         }

//         let data = Json(APIResponse::<T> {
//             data: Some(self.0.unwrap()),
//             error: None,
//         });

//         (StatusCode::OK, data).into_response()
//     }
// }

// impl From<sqlx::error::Error> for Error {
//     fn from(e: sqlx::error::Error) -> Self {
//         let mut err = Error::tech("");
//         err.add_meta("from", "sqlx");
//         err.add_meta("error", &e.to_string());
//         err.log();
//         err
//     }
// }

// impl From<sqlx::migrate::MigrateError> for Error {
//     fn from(e: sqlx::migrate::MigrateError) -> Self {
//         let mut err = Error::tech("Failed to run database migration");
//         err.add_meta("from", "sqlx::migrate");
//         err.add_meta("error", &e.to_string());
//         err.log();
//         err
//     }
// }

// impl From<bcrypt::BcryptError> for Error {
//     fn from(e: bcrypt::BcryptError) -> Self {
//         let mut err = Error::tech("");
//         err.add_meta("from", "bcrypt");
//         err.add_meta("error", &e.to_string());
//         err.log();
//         err
//     }
// }

impl From<std::env::VarError> for Error {
    fn from(e: std::env::VarError) -> Self {
        let mut err = Error::tech("");
        err.add_meta("from", "varerror");
        err.add_meta("error", &e.to_string());
        err
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        let mut err = Error::tech("");
        err.add_meta("from", "parse_int_err");
        err.add_meta("error", &e.to_string());
        err
    }
}

// impl From<std::io::Error> for Error {
//     fn from(e: std::io::Error) -> Self {
//         let mut err = Error::tech("");
//         err.add_meta("from", "io_err");
//         err.add_meta("error", &e.to_string());
//         err
//     }
// }

#[cfg(feature = "surreal")]
impl From<surrealdb::Error> for Error {
    fn from(e: surrealdb::Error) -> Self {
        let mut err = Error::tech("");
        err.add_meta("from", "surreal");
        err.add_meta("error", &e.to_string());
        err
    }
}
