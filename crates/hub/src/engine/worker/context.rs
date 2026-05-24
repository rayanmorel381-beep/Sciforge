use crate::tools::config::Config;
use crate::tools::metrics::Metrics;

/// Runtime context carrying configuration and metrics for task execution.
pub struct ExecutionContext {
    /// Key-value configuration.
    pub config: Config,
    /// Performance counters.
    pub metrics: Metrics,
    /// If `true`, skip actual computation.
    pub dry_run: bool,
    /// Enable verbose logging.
    pub verbose: bool,
}

impl ExecutionContext {
    /// Creates a default context.
    pub fn new() -> Self {
        Self {
            config: Config::new(),
            metrics: Metrics::new(),
            dry_run: false,
            verbose: false,
        }
    }

    /// Sets the configuration.
    pub fn with_config(mut self, config: Config) -> Self {
        self.config = config;
        self
    }

    /// Enables or disables dry-run mode.
    pub fn with_dry_run(mut self, dry_run: bool) -> Self {
        self.dry_run = dry_run;
        self
    }

    /// Enables or disables verbose output.
    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }
}

impl Default for ExecutionContext {
    fn default() -> Self {
        Self::new()
    }
}
