 # Pathfinding Repository - Copilot Coding Agent Instructions

 ## Repository Overview

 This is a Rust library crate that implements pathfinding, flow, and graph algorithms. It provides generic implementations of algorithms like A*, Dijkstra, BFS, DFS, Kruskal, Prim, and many others for directed graphs, undirected graphs, and matching problems.

 **Key Facts:**
 - **Language:** Rust (edition 2024)
 - **MSRV:** 1.87.0 (Minimum Supported Rust Version)
 - **Size:** ~6,000 lines of source code, ~4,400 lines of tests
 - **Type:** Library crate (no binary)
 - **Dependencies:** Minimal (num-traits, indexmap, rustc-hash, integer-sqrt, thiserror, deprecate-until)
 - **Repository:** https://github.com/evenfurther/pathfinding

 ## Build and Validation Commands

 ### Prerequisites
 All commands should be run from the repository root. The project requires Rust toolchain (stable, beta, nightly, or MSRV 1.87.0).

 ### Essential Commands (In Order)

 **1. Check Code (Fast):**
 ```bash
 cargo check --all-targets
 ```
 Time: ~35 seconds (first run), ~1 second (incremental)

 **2. Run Tests:**
 ```bash
 # Unit and integration tests (ALWAYS run this)
 cargo test --tests --benches

 # Documentation tests (ALWAYS run this)
 cargo test --doc
 ```
 Time: ~25 seconds for compilation + ~1 second for test execution

 **3. Format Check:**
 ```bash
 cargo +stable fmt --all -- --check
 ```
 Time: ~1 second. If this fails, run `cargo +stable fmt --all` to auto-fix.

 **4. Lint with Clippy:**
 ```bash
 # First install nightly if needed
 rustup install --profile default nightly

 # Then run clippy
 cargo +nightly clippy --all-targets -- -D warnings
 ```
 Time: ~15 seconds. Clippy MUST run with nightly and MUST pass with no warnings.

 **5. Pre-commit Checks (Optional but Recommended):**
 Pre-commit is a Python program that runs various checks. Install with:
 ```bash
 pip install pre-commit
 pre-commit install --hook-type commit-msg
 ```
 Then run manually with:
 ```bash
 pre-commit run --all-files
 ```
 Pre-commit checks include: trailing whitespace, TOML/YAML validation, codespell, and conventional commit message format.

 **6. MSRV Consistency Check:**
 ```bash
 sh tests/check-msrv-consistency.sh
 ```
 This verifies that the MSRV in Cargo.toml matches the documentation in src/lib.rs.

 **7. Cargo Deny (License and Security Checks):**
 ```bash
 # Install once
 cargo install cargo-deny

 # Run checks
 cargo deny check
 ```
 ### Release Build
 ```bash
 cargo build --release
 ```
 Time: ~3 seconds (incremental)

 ### Run Examples
 ```bash
 cargo run --example sliding-puzzle
 cargo run --example bfs_bidirectional
 ```

 ### Benchmarks
 ```bash
 # Build benchmarks (don't run them, they take a long time)
 cargo bench --no-run
 ```
 Time: ~50 seconds

 ## Project Structure

 ### Root Directory Files
 - `Cargo.toml` - Project manifest with dependencies, MSRV (rust-version = "1.87.0"), and linting configuration
 - `Cargo.lock` - Locked dependency versions
 - `README.md` - User-facing documentation
 - `CHANGELOG.md` - Version history
 - `.gitignore` - Ignores: target/, mutants.out*, flamegraph.svg, perf.data*
 - `deny.toml` - Configuration for cargo-deny (license and security checks)
 - `.pre-commit-config.yaml` - Pre-commit hooks configuration
 - `.gitlab-ci.yml` - GitLab CI configuration (legacy)
 - `release.sh` - Release script (requires git-extras and gh CLI)

 ### Source Code Structure
 ```
 src/
 ├── lib.rs                 # Main library entry point with module exports
 ├── directed/              # Directed graph algorithms
 │   ├── mod.rs
 │   ├── astar.rs          # A* pathfinding
 │   ├── bfs.rs            # Breadth-first search
 │   ├── dfs.rs            # Depth-first search
 │   ├── dijkstra.rs       # Dijkstra's algorithm
 │   ├── edmonds_karp.rs   # Maximum flow
 │   ├── fringe.rs         # Fringe search
 │   ├── idastar.rs        # Iterative deepening A*
 │   ├── iddfs.rs          # Iterative deepening DFS
 │   ├── count_paths.rs    # Path counting in DAGs
 │   ├── cycle_detection.rs # Floyd and Brent algorithms
 │   ├── strongly_connected_components.rs
 │   ├── topological_sort.rs
 │   └── yen.rs            # K-shortest paths
 ├── undirected/            # Undirected graph algorithms
 │   ├── mod.rs
 │   ├── cliques.rs        # Bron-Kerbosch algorithm
 │   ├── connected_components.rs
 │   ├── kruskal.rs        # Minimum spanning tree
 │   └── prim.rs           # Minimum spanning tree
 ├── grid.rs                # Grid data structure
 ├── matrix.rs              # Matrix data structure
 ├── kuhn_munkres.rs        # Hungarian algorithm for matching
 ├── noderefs.rs            # Node reference utilities
 └── utils.rs               # Utility functions
 ```

 ### Tests and Examples
 ```
 tests/                     # Integration tests (35+ test files)
 examples/                  # Example programs (bfs_bidirectional.rs, sliding-puzzle.rs)
 benches/                   # Benchmark suites (algos.rs, edmondskarp.rs, matrices.rs, etc.)
 ```

 ### GitHub Actions Workflows
 Located in `.github/workflows/`:

 1. **tests.yml** (Runs on PRs and merge groups):
    - `check` job: Runs MSRV check and `cargo check --all-targets` with nightly
    - `cargo-deny` job: Runs license and security checks
    - `test` job: Runs tests on stable, beta, nightly, and MSRV toolchains
    - `test-release` job: Runs tests in release mode with nightly
    - `test-minimal-versions` job: Tests with minimal dependency versions
    - `fmt` job: Checks formatting with stable rustfmt
    - `clippy` job: Runs clippy with nightly, treating warnings as errors

 2. **pre-commit.yaml** (Runs on PRs and merge groups):
    - Runs pre-commit hooks (trailing whitespace, TOML/YAML validation, codespell, conventional commits)

 3. **codspeed.yml** (Runs on main branch pushes and PRs):
    - Runs performance benchmarks

 ## MSRV (Minimum Supported Rust Version)

 When updating the MSRV, you must update it in the following locations:
 1. **Cargo.toml** - The `rust-version` field (line 14)
 2. **src/lib.rs** - Documentation comment stating "The minimum supported Rust version (MSRV) is Rust X.Y.Z" (line 89)
 3. **.github/copilot-instructions.md** - Three locations:
    - "MSRV: X.Y.Z" in Key Facts section (line 9)
    - "MSRV X.Y.Z" in Prerequisites section (line 18)
    - "rust-version = X.Y.Z" in Root Directory Files section (line 102)

 **Note:** The GitHub Actions workflow (.github/workflows/tests.yml) automatically reads the MSRV from Cargo.toml, so it does not need manual updates.

 After updating the MSRV, always run `sh tests/check-msrv-consistency.sh` to verify that Cargo.toml and src/lib.rs are in sync.

 ## Critical Validation Rules

 ### Before Committing
 1. **ALWAYS run tests first:** `cargo test --tests --benches && cargo test --doc`
 2. **ALWAYS run formatting:** `cargo +stable fmt --all -- --check` (auto-fix with `cargo +stable fmt --all`)
 3. **ALWAYS run clippy with nightly:** `cargo +nightly clippy --all-targets -- -D warnings`
 4. **Check MSRV consistency:** `sh tests/check-msrv-consistency.sh` if you modify Cargo.toml or src/lib.rs
 5. **Remove trailing spaces:** All files must have trailing whitespace removed (pre-commit checks enforce this)
 6. **Unix line terminators:** Unix regular \n terminators must be used

 ### Commit Message Format
 This repository uses **conventional commits**. Every commit message must follow this format:
 ```
 <type>(<scope>): <description>

 Examples:
 - feat(matrix): add Matrix::transpose()
 - fix(tests): remove unused imports
 - chore(changelog): prepare for next release
 ```
 Valid types: `feat`, `fix`, `chore`, `test`

 ### Common Pitfalls
 1. **Clippy must run with nightly**, not stable. The CI will fail if you only test with stable clippy.
 2. **Formatting must use stable rustfmt**, not nightly. Use `cargo +stable fmt`.
 3. **All warnings are errors in clippy**. The build will fail if clippy reports any warnings.
 4. **Tests must pass in both debug and release modes** on multiple toolchains (stable, beta, nightly, MSRV).
 5. **Documentation tests are separate** from regular tests. Always run both `cargo test --tests` and `cargo test --doc`.
 6. **Benchmarks are tests too**. Use `--benches` flag when running tests to include benchmark tests.

 ## Development Workflow

 ### Typical Change Process
 1. Make your code changes in the appropriate module (src/directed/, src/undirected/, etc.)
 2. If you modify public APIs, update documentation with examples
 3. Run `cargo test --tests --benches && cargo test --doc` to verify functionality
 4. Run `cargo +stable fmt --all` to format code
 5. Run `cargo +nightly clippy --all-targets -- -D warnings` to check for issues
 6. If you changed Cargo.toml or src/lib.rs MSRV documentation, run `sh tests/check-msrv-consistency.sh`
 7. Commit with a conventional commit message

 ### Where to Make Changes
 - **Adding a new algorithm:** Create a new file in src/directed/ or src/undirected/, add module to mod.rs, export from lib.rs
 - **Modifying existing algorithm:** Edit the corresponding file in src/directed/ or src/undirected/
 - **Adding utility functions:** Add to src/utils.rs
 - **Adding data structures:** Create a new file in src/ (like grid.rs or matrix.rs)
 - **Adding tests:** Create a new file in tests/ directory
 - **Adding examples:** Create a new .rs file in examples/ directory

 ### Linting Configuration
 Clippy lints are configured in Cargo.toml under `[lints.clippy]`:
 - `pedantic = "deny"` - All pedantic lints are errors
 - `missing_const_for_fn = "deny"` - Functions that could be const must be const
 - `redundant_clone = "deny"` - Redundant clones are errors
 - `module_name_repetitions = "allow"` - Exception for module name repetitions
 - `too_long_first_doc_paragraph = "allow"` - Temporary exception for long doc paragraphs

 ## Quick Reference

-**Test everything:** `cargo test --tests --benches && cargo test --doc`
-**Format code:** `cargo +stable fmt --all`
-**Lint code:** `cargo +nightly clippy --all-targets -- -D warnings`
-**Check quickly:** `cargo check --all-targets`
-**Build release:** `cargo build --release`
+**Test everything:** `cargo test --tests --benches && cargo test --doc`
+**Format code:** `cargo +stable fmt --all`
+**Lint code:** `cargo +nightly clippy --all-targets -- -D warnings`
+**Check quickly:** `cargo check --all-targets`
+**Build release:** `cargo build --release`
 **Run example:** `cargo run --example <name>`

 ## Trust These Instructions

 These instructions have been validated by running all commands and observing their behavior. If you encounter discrepancies, verify first before searching extensively. The most common issues are:
 1. Using wrong Rust toolchain (stable vs nightly) for clippy or fmt
 2. Forgetting to run both `--tests` and `--doc` test suites
 3. Not including `--benches` flag when running tests
