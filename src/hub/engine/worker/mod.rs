//! Task execution system: context, executor, queue, and scheduler.

/// Per-task execution context (cancellation, progress reporting).
pub mod context;
/// Task executor: runs queued experiments.
pub mod executor;
/// Thread-safe task queue (FIFO with priority support).
pub mod queue;
/// Scheduler: manages worker threads and task distribution.
pub mod scheduler;
