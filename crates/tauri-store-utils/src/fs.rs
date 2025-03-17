use bon::builder;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{from_slice, to_vec, to_vec_pretty};
use std::fs::{self, File};
use std::io::ErrorKind::NotFound;
use std::io::Result;
use std::io::Write;
use std::path::Path;

/// Reads the contents of a file and deserializes it into a value.
#[builder]
pub fn read_file<T>(
  #[builder(start_fn)] path: impl AsRef<Path>,
  #[builder(default)] create: bool,
  #[builder(default)] create_pretty: bool,
  #[builder(default)] create_sync: bool,
) -> Result<T>
where
  T: DeserializeOwned + Serialize + Default,
{
  match fs::read(&path) {
    Ok(bytes) => Ok(from_slice(&bytes)?),
    Err(err) if err.kind() == NotFound => {
      let value = T::default();
      if create {
        write_file(&path, &value)
          .create_new(true)
          .pretty(create_pretty)
          .sync(create_sync)
          .call()?;
      }

      Ok(value)
    }
    Err(err) => Err(err),
  }
}

/// Writes a JSON-serializable value to a file.
#[builder]
pub fn write_file<T>(
  #[builder(start_fn)] path: impl AsRef<Path>,
  #[builder(start_fn)] value: &T,
  #[builder(default)] create_new: bool,
  #[builder(default)] pretty: bool,
  #[builder(default)] sync: bool,
) -> Result<()>
where
  T: ?Sized + Serialize,
{
  let path = path.as_ref();
  if let Some(parent) = path.parent() {
    fs::create_dir_all(parent)?;
  }

  let bytes = if pretty {
    to_vec_pretty(value)?
  } else {
    to_vec(value)?
  };

  let mut file = if create_new {
    File::create_new(path)?
  } else {
    File::create(path)?
  };

  file.write_all(&bytes)?;

  if sync {
    file.sync_all()?;
  }

  Ok(())
}
