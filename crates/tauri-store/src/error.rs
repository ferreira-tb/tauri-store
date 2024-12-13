use serde::{Serialize, Serializer};
use std::error::Error as StdError;
use std::result::Result as StdResult;

pub type Result<T> = StdResult<T, Error>;
pub type BoxResult<T> = StdResult<T, Box<dyn StdError>>;

#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error(transparent)]
  Io(#[from] std::io::Error),
  #[error(transparent)]
  Json(#[from] serde_json::Error),
  #[error(transparent)]
  Tauri(#[from] tauri::Error),
}

impl Error {
  pub const fn is_bad_rid(&self) -> bool {
    matches!(self, Self::Tauri(tauri::Error::BadResourceId(_)))
  }
}

impl Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(self.to_string().as_str())
  }
}

#[doc(hidden)]
#[macro_export]
macro_rules! io_err {
  ($variant:ident) => {{
    use $crate::Error;
    use std::io::{Error as IoError, ErrorKind};
    let err = IoError::from(ErrorKind::$variant);
    Err(Error::Io(err))
  }};
  ($variant:ident, $($arg:tt)*) => {{
    use $crate::Error;
    use std::io::{Error as IoError, ErrorKind};
    let err = IoError::new(ErrorKind::$variant, format!($($arg)*));
    Err(Error::Io(err))
  }};
}
