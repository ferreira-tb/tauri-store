use anyhow::{Result, bail};
use bon::builder;
use std::path::PathBuf;
use std::process::Command;

#[builder]
pub fn command(
  #[builder(start_fn)] program: &str,
  #[builder(with = <_>::from_iter)] args: Vec<&str>,
  dir: Option<PathBuf>,
) -> Result<()> {
  let mut cmd = if cfg!(windows) {
    Command::new("pwsh")
  } else {
    Command::new(program)
  };

  if cfg!(windows) {
    cmd.args(["-Command", program]);
  }

  if !args.is_empty() {
    cmd.args(args);
  }

  if let Some(cwd) = dir {
    cmd.current_dir(cwd);
  }

  let status = cmd.status()?;

  if !status.success() {
    bail!("command `{program}` failed with status:\n{status}");
  }

  Ok(())
}
