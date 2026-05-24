## 1️⃣ Current State

| Building Block | Location | Description |
|---|---|---|
| Scheduler | `src/hub/engine/worker/scheduler.rs` | Topological sort by priority, sequential execution |
| Executor | `src/hub/engine/worker/executor.rs` | Executes a closure F sequentially with retry |
| TaskQueue | `src/hub/engine/worker/queue.rs` | Simple VecDeque, enqueue/dequeue FIFO |
| ExecutionContext | `src/hub/engine/worker/context.rs` | config, metrics, dry_run, verbose |
| Metrics | `src/hub/tools/metrics.rs` | `Mutex<HashMap>` for counters and timings — only Mutex usage |
| LOG_LEVEL | `src/hub/` | AtomicU8 — only atomic usage |

No thread::spawn, no thread::scope, no rayon, no Arc in src/. SimState run() entirely sequential. Zero external dependency.

## 2️⃣ Thread Pool — `src/hub/engine/worker/pool.rs`

| Component | Description |
|---|---|
| `ThreadPool` | `{ workers: Vec<Worker>, sender: Sender<Job> }` |
| `Worker` | `{ id: usize, thread: Option<JoinHandle<()>> }` |
| `Job` | `Box<dyn FnOnce() + Send + 'static>` |
| `ThreadPool::new(size)` | Creates N threads waiting on a `Receiver<Job>` |
| `ThreadPool::execute(f)` | Sends F into the channel |
| `impl Drop` | Joins all threads cleanly |
| Channel | `std::sync::mpsc` for task distribution |

Reusable pool: threads persist as long as the pool exists.

## 3️⃣ Parallel Scoped — `src/hub/engine/worker/scoped.rs`

| Function | Signature | Description |
|---|---|---|
| parallel_for | `parallel_for(items: &[T], f: Fn(&T) → R) → Vec<R>` | Via `std::thread::scope` (stable Rust 1.63) |
| parallel_map | `parallel_map(items: Vec<T>, f: Fn(T) → R) → Vec<R>` | Parallel map by ownership |
| parallel_reduce | `parallel_reduce(items: &[T], identity: T, f: Fn(T, &T) → T) → T` | Parallel reduction |

Automatic chunking: `items.len() / num_cpus` (rounded). Each thread processes its chunk sequentially.

## 4️⃣ Parallel Scheduler — extension

| Component | Description |
|---|---|
| `schedule_parallel(&self, pool, exec)` | → `Vec<TaskResult>` — tasks without dependencies launched in parallel |
| Dependency enforcement | Tasks with dependencies wait via WaitGroup or barrier |
| `WaitGroup` | `{ counter: Arc<AtomicUsize>, condvar: Condvar }` |

Preserves the existing Scheduler (topological sort).

## 5️⃣ CPU Detection — `src/hub/engine/worker/num_cpus.rs`

| Function | Description |
|---|---|
| `available_parallelism() → usize` | Wrapper around `std::thread::available_parallelism()` (stable Rust 1.59) |
| Fallback | Returns 1 if detection fails |
| `recommended_threads() → usize` | available_parallelism, capped if necessary |

## 6️⃣ Simulation Parallelization

| Component | Description |
|---|---|
| Force computation | O(N²) between N bodies — parallelizable per body |
| `parallel_step(state, step_fn, dt)` | Splits positions/velocities into chunks, parallel computation via thread::scope |
| Synchronization | Barrier after each step |

## 7️⃣ Benchmark Parallelization

| Component | Description |
|---|---|
| Independent experiments | `run_experiments_parallel(experiments, pool) → Vec<RunOutput>` |
| Trivial | Each experiment is independent → direct parallelization |
| Metrics | `Mutex<HashMap>` already thread-safe |

## 8️⃣ Export Parallelization

| Component | Description |
|---|---|
| By format | CSV/JSON/YAML/TOML/HTML/MD are independent → parallelize by format |
| By file | File writes are independent → parallelize by file |

## 9️⃣ Module Structure

| File | Description |
|---|---|
| `src/hub/engine/worker/pool.rs` | ThreadPool, Worker, Job |
| `src/hub/engine/worker/scoped.rs` | parallel_for, parallel_map, parallel_reduce |
| `src/hub/engine/worker/num_cpus.rs` | available_parallelism, recommended_threads |
| `src/hub/engine/worker/mod.rs` | Re-exports everything |

## 🔟 Constraints

| Constraint | Description |
|---|---|
| Zero dependency | Only std::thread, std::sync::{mpsc, Arc, Mutex, Condvar, atomic} |
| Send + Sync | All shared types must be Send + Sync |
| No unsafe | No unsafe blocks |
| Clean shutdown | Pools drain their tasks before drop |
| Configurable limit | max_workers in ExecutionContext |

## 1️⃣1️⃣ Tests

| Test | Description |
|---|---|
| ThreadPool | Launch N tasks, verify all completed |
| parallel_for | Results identical to sequential version |
| Parallel scheduler | Dependencies enforced (A before B if B depends on A) |
| Benchmark | Numerical results identical sequential vs parallel |
| Stress test | 1000 concurrent tasks, no deadlock |
| Fallback single thread | If available_parallelism() = 1, everything remains sequential |
