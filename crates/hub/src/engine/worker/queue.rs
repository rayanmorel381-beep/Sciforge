use std::collections::VecDeque;

/// Unit of work to be queued.
#[derive(Debug, Clone)]
pub struct Task {
    /// Unique task identifier.
    pub id: String,
    /// Serialized task payload.
    pub payload: String,
}

/// Bounded FIFO task queue.
pub struct TaskQueue {
    inner: VecDeque<Task>,
    capacity: usize,
}

impl TaskQueue {
    /// Creates a queue with the given maximum capacity.
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    /// Pushes a task; returns `false` if the queue is full.
    pub fn enqueue(&mut self, task: Task) -> bool {
        if self.inner.len() >= self.capacity {
            return false;
        }
        self.inner.push_back(task);
        true
    }

    /// Pops the next task from the front.
    pub fn dequeue(&mut self) -> Option<Task> {
        self.inner.pop_front()
    }

    /// Returns a reference to the front task without removing it.
    pub fn peek(&self) -> Option<&Task> {
        self.inner.front()
    }

    /// Number of tasks currently queued.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns `true` if the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Returns `true` if the queue has reached capacity.
    pub fn is_full(&self) -> bool {
        self.inner.len() >= self.capacity
    }

    /// Removes all tasks.
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    /// Drains all tasks into a vector.
    pub fn drain(&mut self) -> Vec<Task> {
        self.inner.drain(..).collect()
    }

    /// Maximum number of tasks the queue can hold.
    pub fn capacity(&self) -> usize {
        self.capacity
    }
}
