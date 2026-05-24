use crate::domain::common::errors::HubResult;

/// Task execution status.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

/// Result of a single task execution.
pub struct TaskResult {
    /// Unique task identifier.
    pub task_id: String,
    /// Final status.
    pub status: TaskStatus,
    /// Output string on success.
    pub output: Option<String>,
    /// Error message on failure.
    pub error: Option<String>,
    /// Wall-clock time in milliseconds.
    pub elapsed_ms: f64,
}

/// Runs tasks with optional retry logic.
pub struct Executor {
    max_retries: usize,
}

impl Executor {
    /// Creates an executor with no retries.
    pub fn new() -> Self {
        Self { max_retries: 0 }
    }

    /// Sets the maximum number of retries on failure.
    pub fn with_retries(mut self, retries: usize) -> Self {
        self.max_retries = retries;
        self
    }

    /// Executes a closure as a named task, retrying on failure.
    pub fn execute<F>(&self, task_id: &str, f: F) -> TaskResult
    where
        F: Fn() -> HubResult<String>,
    {
        let start = std::time::Instant::now();
        let mut last_err = None;

        for _ in 0..=self.max_retries {
            match f() {
                Ok(output) => {
                    return TaskResult {
                        task_id: task_id.to_string(),
                        status: TaskStatus::Completed,
                        output: Some(output),
                        error: None,
                        elapsed_ms: start.elapsed().as_secs_f64() * 1000.0,
                    };
                }
                Err(e) => {
                    last_err = Some(e);
                }
            }
        }

        TaskResult {
            task_id: task_id.to_string(),
            status: TaskStatus::Failed,
            output: None,
            error: last_err.map(|e| e.to_string()),
            elapsed_ms: start.elapsed().as_secs_f64() * 1000.0,
        }
    }
}

impl Default for Executor {
    fn default() -> Self {
        Self::new()
    }
}
