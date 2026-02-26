# Theseus (`tsus`) — Architecture Overview

A quick guide to how the codebase is organized and how each piece fits together.

---

## Project Structure

```
src/
 ├── main.rs        # Entry point, CLI routing via clap
 ├── lib.rs         # Core exports and project identity logic
 ├── run.rs         # `tsus run` — build + run orchestration
 ├── project.rs     # Project naming, Dockerfile detection, language detection
 └── templates.rs   # Embedded Dockerfile templates (Node, Python, Rust, Go)

tests/
 ├── cli.rs         # Integration tests for CLI argument handling
 └── project.rs     # Unit tests for project detection logic
```

---

## How It Works

### CLI Router (`main.rs`)

Uses `clap` with the Derive API. The 7 core commands are explicitly matched. Everything else hits a catch-all that forwards directly to `docker` with full stdin/stdout/stderr passthrough and exit code preservation.

### Project Identity (`lib.rs`, `project.rs`)

Every project is identified by its directory name:
- **Image**: `<dir_name>-theseus`
- **Container**: `<dir_name>-theseus`

No random hashes — students always know what container belongs to what project.

### `tsus run` Flow (`run.rs`)

1. Check for `Dockerfile` in the current directory
2. If missing → detect language via heuristic files (`package.json`, `Cargo.toml`, etc.)
3. If detected → generate a temporary Dockerfile from embedded templates
4. Build the image (skip if already up-to-date)
5. Run with `-it --rm -v "$PWD:/app" -w /app` for live volume mounting

### `tsus init` (`templates.rs`)

Writes a minimal, dev-focused Dockerfile for the chosen language. Refuses to overwrite existing Dockerfiles without confirmation.

### Docker Passthrough

Any command not in the core 7 is forwarded verbatim:
- `tsus ps` → `docker ps`
- `tsus images` → `docker images`

Stdin, stdout, stderr are all inherited. Exit codes propagate cleanly.

---

## Testing

```bash
cargo test
```

- **Unit tests** verify project naming and language detection logic
- **Integration tests** invoke the compiled `tsus` binary to verify passthrough and core commands

---

## Building & Releasing

**Local development:**
```bash
cargo build
cargo run -- run
```

**Release build:**
```bash
cargo build --release
```

**Automated releases:** Push a tag like `v0.1.0` and GitHub Actions cross-compiles for macOS (Intel + ARM) and Linux, then attaches binaries to the GitHub Release.
