use crate::manager::ManagerExt;
use crate::sync::AtomicOption;
use crate::RemoteCallable;
use std::future::Future;
use std::ops::Deref;
use std::sync::{Arc, Weak};
use std::time::Duration;
use tauri::{AppHandle, Runtime};
use tokio::select;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio::task::AbortHandle;
use tokio::time::sleep;

#[cfg(tauri_store_tracing)]
use {
  std::sync::atomic::{AtomicU64, Ordering},
  tracing::{debug, trace},
};

#[cfg(tauri_store_tracing)]
static DEBOUNCE_ID: AtomicU64 = AtomicU64::new(0);

type DebouncedFn<R, Fut> = dyn Fn(AppHandle<R>) -> Fut + Send + Sync + 'static;

/// Debounces a function call.
pub struct Debounce<R, T, Fut>
where
  R: Runtime,
  T: Send + 'static,
  Fut: Future<Output = T> + Send + 'static,
{
  inner: Arc<DebouncedFn<R, Fut>>,
  sender: Arc<OptionalSender>,
  abort_handle: Arc<OptionalAbortHandle>,
  duration: Duration,

  #[cfg(tauri_store_tracing)]
  id: u64,
}

impl<R, T, Fut> Debounce<R, T, Fut>
where
  R: Runtime,
  T: Send + 'static,
  Fut: Future<Output = T> + Send + 'static,
{
  pub fn new<F>(duration: Duration, f: F) -> Self
  where
    F: Fn(AppHandle<R>) -> Fut + Send + Sync + 'static,
  {
    Self {
      inner: Arc::new(f),
      sender: Arc::new(OptionalSender::default()),
      abort_handle: Arc::new(OptionalAbortHandle::default()),
      duration,

      #[cfg(tauri_store_tracing)]
      id: DEBOUNCE_ID.fetch_add(1, Ordering::SeqCst),
    }
  }

  pub fn call(&self, app: &AppHandle<R>) {
    if self.sender.send() {
      #[cfg(tauri_store_tracing)]
      trace!("debounce {} is ready, message sent", self.id);

      return;
    }

    #[cfg(tauri_store_tracing)]
    trace!("debounce {} is not ready, spawning actor", self.id);

    let (tx, rx) = unbounded_channel();
    let actor = Actor {
      inner: Arc::downgrade(&self.inner),
      receiver: rx,
      duration: self.duration,

      #[cfg(tauri_store_tracing)]
      id: self.id,
    };

    self.sender.replace(tx);
    self
      .abort_handle
      .replace(actor.run(app, Arc::downgrade(&self.sender)));
  }

  pub fn abort(&self) {
    self.sender.take();
    self.abort_handle.abort();

    #[cfg(tauri_store_tracing)]
    debug!("debounce {} aborted", self.id);
  }
}

impl<R, T, Fut> RemoteCallable<AppHandle<R>> for Debounce<R, T, Fut>
where
  R: Runtime,
  T: Send + 'static,
  Fut: Future<Output = T> + Send + 'static,
{
  #[inline]
  fn call(&self, app: &AppHandle<R>) {
    self.call(app);
  }

  #[inline]
  fn abort(&self) {
    self.abort();
  }
}

impl<R, T, Fut> Drop for Debounce<R, T, Fut>
where
  R: Runtime,
  T: Send + 'static,
  Fut: Future<Output = T> + Send + 'static,
{
  fn drop(&mut self) {
    self.abort();
  }
}

struct Actor<R, T, Fut>
where
  R: Runtime,
  T: Send + 'static,
  Fut: Future<Output = T> + Send + 'static,
{
  inner: Weak<DebouncedFn<R, Fut>>,
  receiver: UnboundedReceiver<()>,
  duration: Duration,

  #[cfg(tauri_store_tracing)]
  id: u64,
}

impl<R, T, Fut> Actor<R, T, Fut>
where
  R: Runtime,
  T: Send + 'static,
  Fut: Future<Output = T> + Send + 'static,
{
  fn run(mut self, app: &AppHandle<R>, sender: Weak<OptionalSender>) -> AbortHandle {
    app.spawn(move |app| async move {
      loop {
        select! {
          message = self.receiver.recv() => {
            if message.is_none() { break };
          }
          () = sleep(self.duration) => {
            if let Some(f) = self.inner.upgrade() {
              (f)(app).await;
            }

            if let Some(sender) = sender.upgrade() {
              sender.take();
            }

            #[cfg(tauri_store_tracing)]
            debug!("debounce {} completed", self.id);

            break;
          }
        }
      }
    })
  }
}

#[derive(Default)]
struct OptionalSender(AtomicOption<UnboundedSender<()>>);

impl OptionalSender {
  fn send(&self) -> bool {
    self
      .map(|it| it.send(()).is_ok())
      .unwrap_or(false)
  }
}

impl Deref for OptionalSender {
  type Target = AtomicOption<UnboundedSender<()>>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

#[derive(Default)]
struct OptionalAbortHandle(AtomicOption<AbortHandle>);

impl OptionalAbortHandle {
  fn abort(&self) {
    if let Some(handle) = self.take() {
      handle.abort();
    }
  }
}

impl Deref for OptionalAbortHandle {
  type Target = AtomicOption<AbortHandle>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
