//! Command-line interface adapter.
//!
//! Parses CLI arguments into a structured [`Command`] enum that the
//! rest of the hub can dispatch.

/// CLI command variant parsed from user arguments.
#[derive(Debug, Clone)]
pub enum Command {
    Compute {
        domain: String,
        function: String,
        params: Vec<String>,
    },
    Simulate {
        model: String,
        duration: f64,
        dt: f64,
    },
    ListDomains,
    ListFunctions {
        domain: String,
    },
    Serve {
        port: u16,
    },
    Version,
    Help,
}

/// Parsed CLI arguments.
#[derive(Debug, Clone)]
pub struct CliArgs {
    /// The resolved command to execute.
    pub command: Command,
    /// Whether verbose output is enabled.
    pub verbose: bool,
}

/// Parses raw command-line arguments into structured [`CliArgs`].
pub fn parse(args: &[String]) -> CliArgs {
    let verbose = args.iter().any(|a| a == "--verbose" || a == "-V");
    let filtered: Vec<&String> = args.iter().filter(|a| !a.starts_with('-')).collect();

    if filtered.len() <= 1 {
        return CliArgs {
            command: Command::Help,
            verbose,
        };
    }

    let command = match filtered[1].as_str() {
        "compute" if filtered.len() >= 4 => Command::Compute {
            domain: filtered[2].clone(),
            function: filtered[3].clone(),
            params: filtered[4..].iter().map(|s| s.to_string()).collect(),
        },
        "simulate" if filtered.len() >= 3 => Command::Simulate {
            model: filtered[2].clone(),
            duration: filtered.get(3).and_then(|s| s.parse().ok()).unwrap_or(10.0),
            dt: filtered.get(4).and_then(|s| s.parse().ok()).unwrap_or(0.01),
        },
        "list" => {
            if let Some(domain) = filtered.get(2) {
                Command::ListFunctions {
                    domain: domain.to_string(),
                }
            } else {
                Command::ListDomains
            }
        }
        "serve" => {
            let port = filtered.get(2).and_then(|s| s.parse().ok()).unwrap_or(8080);
            Command::Serve { port }
        }
        "version" => Command::Version,
        _ => Command::Help,
    };

    CliArgs { command, verbose }
}

/// Returns the help/usage text.
pub fn usage() -> &'static str {
    "sciforge-hub\n\n\
     USAGE:\n  \
       sciforge <command> [args...]\n\n\
     COMMANDS:\n  \
       compute <domain> <function> [params...]  Run a computation\n  \
       simulate <model> [duration] [dt]         Run a simulation\n  \
       list [domain]                            List domains or functions\n  \
       serve [port]                             Start HTTP server\n  \
       version                                  Show version\n  \
       help                                     Show this message\n\n\
     OPTIONS:\n  \
       --verbose, -V    Enable verbose output"
}
