use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{from_slice, to_vec, to_vec_pretty};
use std::fs::{self, File};
use std::io::ErrorKind::NotFound;
use std::io::Result;
use std::io::Write;
use std::path::Path;

pub fn read_file<T>(path: impl AsRef<Path>) -> Result<T>
where
  T: DeserializeOwned + Default,
{
  match fs::read(path) {
    Ok(bytes) => Ok(from_slice(&bytes)?),
    Err(err) if err.kind() == NotFound => Ok(T::default()),
    Err(err) => Err(err),
  }
}

pub fn write_file<T>(path: impl AsRef<Path>, value: &T, options: &WriteFileOptions) -> Result<()>
where
  T: ?Sized + Serialize,
{
  let path = path.as_ref();
  if let Some(parent) = path.parent() {
    fs::create_dir_all(parent)?;
  }

  let bytes = to_bytes(value, options)?;
  let mut file = File::create(path)?;
  file.write_all(&bytes)?;

  if options.sync {
    file.sync_all()?;
  }

  Ok(())
}

fn to_bytes<T>(value: &T, options: &WriteFileOptions) -> Result<Vec<u8>>
where
  T: ?Sized + Serialize,
{
  if options.pretty {
    Ok(to_vec_pretty(value)?)
  } else {
    Ok(to_vec(value)?)
  }
}

pub struct WriteFileOptions {
  pub pretty: bool,
  pub sync: bool,
}

#[allow(clippy::derivable_impls)]
impl Default for WriteFileOptions {
  fn default() -> Self {
    Self { pretty: false, sync: false }
  }
}
