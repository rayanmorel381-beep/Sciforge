# Contributing to SciForge

**Email:** The email associated with my [crates.io] account : https://crates.io/users/rayanmorel381-beep
**Discord:** The official discod's servor ascociated wiith the SciForge project : https://discord.com/channels/1485418062718173226/1489547778286813244
## Project Overview

SciForge is a pure Rust (edition 2024) scientific computing library with zero external dependencies. It currently spans 11 modules, 575 source files, and 48 600+ lines of code covering mathematics, physics, chemistry, biology, geology, astronomy, meteorology, benchmarking, parsing, and a central hub.

## How to Contribute

1. **Propose your change or idea** — describe the problem you want to solve or the improvement you envision.
2. **Include your suggestions for APIs** — indicate what functions, structs, or modules you think are necessary or redundant.
3. **Provide examples or code snippets** where possible — even small snippets or pseudocode help.

I will review your proposal and provide feedback. If approved, we can coordinate how I'll integrate it.

## Types of Contributions

- **Bug fixes** — identify and propose solutions to issues.
- **Optimizations** — suggest ways to improve performance, memory usage, or runtime behavior.
- **New features** — propose additional analysis capabilities, APIs, or tools.
- **Documentation** — help clarify usage, examples, or internal logic.
- **Testing** — provide sample files, expected outputs, or validation scripts.
- **Logo & visual identity** — if you can improve or refine the SciForge logo, your contribution is very welcome.

## Code Conventions

SciForge follows strict conventions. All contributions must adhere to:

- **Zero dependencies** — no external crates allowed.
- **Zero comments** — code must be self-explanatory through clear naming and structure.
- **No `#[allow]` directives** — all clippy and compiler warnings must be resolved, not suppressed.
- **Edition 2024** — use current Rust idioms and features.
- **Zero clippy warnings** — `cargo clippy` must pass cleanly.
- **All tests pass** — `cargo test` must succeed (currently 94 tests: 28 benchmark + 66 parser).

## Proposal Guidelines

When proposing improvements, please include:

- A clear description of the problem or opportunity
- The API surface you envision (functions, structs, modules)
- Any considerations about backward compatibility or potential side effects
- Optional: performance trade-offs or optimizations you foresee

Even rough ideas or pseudocode are welcome — detailed implementation can come later.

## API Discussion

I value input on:

- Which APIs should be public and necessary
- Which APIs are redundant or unnecessary
- Any abstractions that could simplify usage or improve performance

Your suggestions help shape a stable and ergonomic API for SciForge.

## Testing & Validation

- Whenever possible, provide example files and expected outputs
- Suggest how proposed changes can be validated in sandbox or test environments
- Testing helps ensure new contributions integrate safely and correctly

## Reporting Issues

If you encounter a bug or unexpected behavior:

1. Describe the steps to reproduce it
2. Provide expected vs. actual behavior
3. Include relevant environment details (OS, Rust version, platform)

Please send reports via email or the discord.

## Code of Conduct

I value respect, collaboration, and constructive feedback. Keep discussions professional and welcoming.

Thank you for helping make SciForge better. Your contributions — big or small — are always appreciated.
