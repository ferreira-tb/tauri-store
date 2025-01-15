use super::debounce::{Message, OptionalSender};
use crate::manager::ManagerExt;
use crate::task::{OptionalAbortHandle, RemoteCallable};
use std::future::Future;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::{Arc, Weak};
use std::time::Duration;
use tauri::async_runtime::spawn;
use tauri::{AppHandle, Runtime};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};
use tokio::task::AbortHandle;
use tokio::time::sleep;

#[cfg(tauri_store_tracing)]
use {std::sync::atomic::AtomicU64, std::sync::atomic::Ordering::SeqCst, tracing::debug};

#[cfg(tauri_store_tracing)]
static ID: AtomicU64 = AtomicU64::new(0);

type ThrottledFn<R, Fut> = dyn Fn(AppHandle<R>) -> Fut + Send + Sync + 'static;

/// Throttles a function call.
pub struct Throttle<R, T, Fut>
where
  R: Runtime,
  T: Send + 'static,
  Fut: Future<Output = T> + Send + 'static,
{
  inner: Arc<ThrottledFn<R, Fut>>,
  waiting: Arc<AtomicBool>,
  sender: Arc<OptionalSender>,
  abort_handle: Arc<OptionalAbortHandle>,
  duration: Duration,

  #[cfg(tauri_store_tracing)]
  id: u64,
}

impl<R, T, Fut> Throttle<R, T, Fut>
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
      waiting: Arc::new(AtomicBool::new(false)),
      sender: Arc::new(OptionalSender::default()),
      abort_handle: Arc::new(OptionalAbortHandle::default()),
      duration,

      #[cfg(tauri_store_tracing)]
      id: ID.fetch_add(1, SeqCst),
    }
  }

  pub fn call(&self, app: &AppHandle<R>) {
    if self.sender.send() || self.waiting.load(Relaxed) {
      return;
    }

    #[cfg(tauri_store_tracing)]
    debug!("spawning throttle {}", self.id);

    let (tx, rx) = unbounded_channel();
    let actor = Actor {
      function: Arc::downgrade(&self.inner),
      waiting: Arc::downgrade(&self.waiting),
      receiver: rx,
      duration: self.duration,

      #[cfg(tauri_store_tracing)]
      id: self.id,
    };

    let _ = tx.send(Message::Call);

    self.sender.replace(tx);
    self.abort_handle.replace(actor.run(app));
  }

  pub fn abort(&self) {
    self.sender.take();
    self.abort_handle.abort();
    self.waiting.store(false, Relaxed);

    #[cfg(tauri_store_tracing)]
    debug!("throttle {} aborted", self.id);
  }
}

impl<R, T, Fut> RemoteCallable<AppHandle<R>> for Throttle<R, T, Fut>
where
  R: Runtime,
  T: Send + 'static,
  Fut: Future<Output = T> + Send + 'static,
{
  fn call(&self, app: &AppHandle<R>) {
    self.call(app);
  }

  fn abort(&self) {
    self.abort();
  }
}

impl<R, T, Fut> Drop for Throttle<R, T, Fut>
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
  function: Weak<ThrottledFn<R, Fut>>,
  waiting: Weak<AtomicBool>,
  receiver: UnboundedReceiver<Message>,
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
  fn run(mut self, app: &AppHandle<R>) -> AbortHandle {
    app.spawn(move |app| async move {
      loop {
        if (self.receiver.recv().await).is_some() {
          let Some(waiting) = self.waiting.upgrade() else { break };
          let Some(function) = self.function.upgrade() else { break };

          if compare_exchange(&waiting, false, true) {
            (function)(app.clone()).await;

            #[cfg(tauri_store_tracing)]
            debug!("throttle {} called", self.id);

            spawn(async move {
              sleep(self.duration).await;
              waiting.store(false, Relaxed);
            });
          }
        } else {
          self.receiver.close();
          self
            .waiting
            .upgrade()
            .inspect(|it| it.store(false, Relaxed));

          break;
        }
      }
    })
  }
}

fn compare_exchange(waiting: &Arc<AtomicBool>, expected: bool, new: bool) -> bool {
  waiting
    .compare_exchange(expected, new, Relaxed, Relaxed)
    .is_ok()
}
