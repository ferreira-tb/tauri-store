#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]
#![doc(html_favicon_url = "https://tb.dev.br/tauri-store/favicon.ico")]

mod manager;
mod sync;
mod task;
mod time;
mod version;

pub use sync::MutexOption;
pub use task::RemoteCallable;
pub use time::{set_interval, Debounce, Throttle};
pub use version::Semver;
