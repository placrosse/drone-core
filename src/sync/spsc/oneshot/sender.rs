use super::Inner;
use crate::sync::spsc::SpscInner;
use alloc::sync::Arc;
use core::{
  pin::Pin,
  sync::atomic::Ordering,
  task::{Context, Poll},
};

const IS_TX_HALF: bool = true;

/// The sending-half of [`oneshot::channel`](super::channel).
pub struct Sender<T, E> {
  inner: Arc<Inner<T, E>>,
}

impl<T, E> Sender<T, E> {
  #[inline]
  pub(super) fn new(inner: Arc<Inner<T, E>>) -> Self {
    Self { inner }
  }

  /// Completes this oneshot with a result.
  ///
  /// If the value is successfully enqueued, then `Ok(())` is returned. If the
  /// receiving end was dropped before this function was called, then `Err` is
  /// returned with the value provided.
  #[inline]
  pub fn send(self, data: Result<T, E>) -> Result<(), Result<T, E>> {
    self.inner.send(data)
  }

  /// Polls this [`Sender`] half to detect whether the [`Receiver`] this has
  /// paired with has gone away.
  ///
  /// # Panics
  ///
  /// Like `Future::poll`, this function will panic if it's not called from
  /// within the context of a task. In other words, this should only ever be
  /// called from inside another future.
  ///
  /// If you're calling this function from a context that does not have a task,
  /// then you can use the [`is_canceled`] API instead.
  ///
  /// [`Sender`]: Sender
  /// [`Receiver`]: super::Receiver
  /// [`is_canceled`]: Sender::is_canceled
  #[inline]
  pub fn poll_cancel(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
    self.inner.poll_half(
      cx,
      IS_TX_HALF,
      Ordering::Relaxed,
      Ordering::Release,
      Inner::take_cancel,
    )
  }

  /// Tests to see whether this [`Sender`]'s corresponding [`Receiver`] has gone
  /// away.
  ///
  /// [`Sender`]: Sender
  /// [`Receiver`]: super::Receiver
  #[inline]
  pub fn is_canceled(&self) -> bool {
    self.inner.is_canceled(Ordering::Relaxed)
  }
}

impl<T, E> Drop for Sender<T, E> {
  #[inline]
  fn drop(&mut self) {
    self.inner.close_half(IS_TX_HALF);
  }
}

impl<T, E> Inner<T, E> {
  fn send(&self, data: Result<T, E>) -> Result<(), Result<T, E>> {
    if self.is_canceled(Ordering::Relaxed) {
      Err(data)
    } else {
      unsafe { *self.data.get() = Some(data) };
      Ok(())
    }
  }
}
