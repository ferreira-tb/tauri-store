use crate::manager::ManagerExt;
use crate::sync::MutexOption;
use crate::task::{OptionalAbortHandle, RemoteCallable};
use std::future::Future;
use std::ops::Deref;
use std::sync::{Arc, Weak};
use std::time::Duration;
use tauri::{AppHandle, Runtime};
use tokio::select;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio::task::AbortHandle;
use tokio::time::sleep;

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
    }
  }

  pub fn call(&self, app: &AppHandle<R>) {
    if self.sender.send() {
      return;
    }

    let (tx, rx) = unbounded_channel();
    let actor = Actor {
      function: Arc::downgrade(&self.inner),
      receiver: rx,
      duration: self.duration,
    };

    self.sender.replace(tx);
    self.abort_handle.replace(actor.run(app));
  }

  pub fn abort(&self) {
    self.sender.take();
    self.abort_handle.abort();
  }
}

impl<R, T, Fut> RemoteCallable<AppHandle<R>> for Debounce<R, T, Fut>
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
  function: Weak<DebouncedFn<R, Fut>>,
  receiver: UnboundedReceiver<Message>,
  duration: Duration,
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
        select! {
          message = self.receiver.recv() => {
            if message.is_none() { break }
          }
          () = sleep(self.duration) => {
            self.receiver.close();
            if let Some(f) = self.function.upgrade() {
              (f)(app).await;
            }

            break;
          }
        }
      }
    })
  }
}

#[derive(Default)]
pub(super) struct OptionalSender(MutexOption<UnboundedSender<Message>>);

impl OptionalSender {
  pub(super) fn send(&self) -> bool {
    self
      .0
      .map(|it| it.send(Message::Call).is_ok())
      .unwrap_or(false)
  }
}

impl Deref for OptionalSender {
  type Target = MutexOption<UnboundedSender<Message>>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

pub(super) enum Message {
  Call,
}
