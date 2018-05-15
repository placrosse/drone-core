//! Fibers.

mod chain;
mod closure;
mod future;
mod generator;
mod stream_ring;
mod stream_unit;

pub use self::chain::Chain;
pub use self::closure::{add_fn, new_fn, FiberFn};
pub use self::future::{add_future, FiberFuture};
pub use self::generator::{add, new, FiberGen};
pub use self::stream_ring::{
  add_stream_ring, add_stream_ring_overwrite, add_stream_ring_skip,
  FiberStreamRing,
};
pub use self::stream_unit::{add_stream, add_stream_skip, FiberStreamUnit};

/// Lightweight thread of execution.
pub trait Fiber {
  /// The type of [`resume`](Fiber::resume) input argument.
  type Input;

  /// The type of value this fiber yields.
  type Yield;

  /// The type of value this fiber returns.
  type Return;

  /// Resumes the execution of this fiber.
  fn resume(
    &mut self,
    input: Self::Input,
  ) -> FiberState<Self::Yield, Self::Return>;
}

/// A fiber suitable for [`Chain`](Chain).
pub trait FiberRoot: Send + 'static {
  /// Resumes the execution of this fiber. Returns `true` if it's still alive.
  fn advance(&mut self) -> bool;
}

/// The result of a fiber resumption.
pub enum FiberState<Y, R> {
  /// The fiber suspended with a value.
  Yielded(Y),
  /// The fiber completed with a return value.
  Complete(R),
}

/// One of `()` or `!`.
pub trait YieldNone: Send + 'static {}

impl YieldNone for () {}
impl YieldNone for ! {}