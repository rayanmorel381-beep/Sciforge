/// Task priority level.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Priority {
    Low,
    Normal,
    High,
    Critical,
}

impl Priority {
    /// Returns a numeric weight (higher = more important).
    pub fn weight(&self) -> u8 {
        match self {
            Self::Low => 0,
            Self::Normal => 1,
            Self::High => 2,
            Self::Critical => 3,
        }
    }
}

/// Task with priority and dependency information.
pub struct ScheduledTask {
    /// Unique task identifier.
    pub id: String,
    /// Execution priority.
    pub priority: Priority,
    /// IDs of tasks that must complete first.
    pub dependencies: Vec<String>,
}

/// Priority-aware dependency scheduler.
pub struct Scheduler {
    tasks: Vec<ScheduledTask>,
}

impl Scheduler {
    /// Creates an empty scheduler.
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    /// Registers a task for scheduling.
    pub fn add(&mut self, task: ScheduledTask) {
        self.tasks.push(task);
    }

    /// Computes a topologically-sorted execution order respecting priorities.
    pub fn schedule(&mut self) -> Vec<String> {
        self.tasks
            .sort_by(|a, b| b.priority.weight().cmp(&a.priority.weight()));

        let mut order = Vec::new();
        let mut completed: Vec<String> = Vec::new();
        let mut remaining: Vec<&ScheduledTask> = self.tasks.iter().collect();

        while !remaining.is_empty() {
            let mut progress = false;
            let mut next_remaining = Vec::new();
            for task in &remaining {
                if task.dependencies.iter().all(|d| completed.contains(d)) {
                    order.push(task.id.clone());
                    completed.push(task.id.clone());
                    progress = true;
                } else {
                    next_remaining.push(*task);
                }
            }
            remaining = next_remaining;
            if !progress {
                break;
            }
        }
        order
    }

    /// Number of registered tasks.
    pub fn count(&self) -> usize {
        self.tasks.len()
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}
