use serde::{Serialize, Serializer};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error(transparent)]
  Io(#[from] std::io::Error),
  #[error(transparent)]
  Json(#[from] serde_json::Error),
  #[error(transparent)]
  Tauri(#[from] tauri::Error),
}

impl Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
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
    use $crate::error::Error;
    use std::io::{Error as IoError, ErrorKind};
    let err = IoError::from(ErrorKind::$variant);
    Err(Error::Io(err))
  }};
  ($variant:ident, $($arg:tt)*) => {{
    use $crate::error::Error;
    use std::io::{Error as IoError, ErrorKind};
    let err = IoError::new(ErrorKind::$variant, format!($($arg)*));
    Err(Error::Io(err))
  }};
}
