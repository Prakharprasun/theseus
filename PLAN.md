# Theseus (`tsus`) — Roadmap

> A student-friendly, Docker-powered container runner with npm-style ergonomics.

---

## Philosophy

Theseus is a thin, opinionated CLI layer on top of Docker.

**It is:**
- Dev-focused & single-project oriented
- Student-friendly — no Docker knowledge required
- Fully Docker-compatible (unknown commands passthrough)

**It is NOT:**
- A container runtime or Docker replacement
- A compose/orchestration tool
- A production deployment system

---

## What's Done (v0.1.0)

Everything below is implemented, tested, and shipping:

| Feature | Description |
|---|---|
| **`tsus run`** | Detects project type, auto-generates Dockerfiles, builds and runs containers with live volume mounts |
| **`tsus init <lang>`** | Scaffolds minimal dev Dockerfiles for Node, Python, Rust, Go |
| **`tsus stop`** | Cleanly stops and removes the project container |
| **`tsus status`** | Shows container state, image info, and disk usage |
| **`tsus clean`** | Removes project images, or `--all` for full system prune |
| **`tsus logs`** | Tails container logs in real-time |
| **`tsus exec`** | Drops into the container shell |
| **`tsus doctor`** | Verifies Docker is installed, daemon running, and `tsus` is in PATH |
| **Passthrough** | Any unknown command (e.g. `tsus ps`) forwards directly to `docker` |
| **Language Detection** | Auto-detects `package.json`, `requirements.txt`, `Cargo.toml`, `go.mod` |
| **Friendly Errors** | Clear guidance when Docker is missing, no project detected, etc. |
| **Curl Installer** | One-line install for macOS (Intel + Apple Silicon) and Linux |
| **CI/CD Releases** | GitHub Actions cross-compiles and publishes binaries on tag push |

---

## What's Planned (Post v1)

These are only considered if real users request them:

- **Hot reload integration** — watch mode for auto-rebuilding on file changes
- **`theseus.toml` config** — optional project config for port mappings, env vars
- **Port auto-detection** — infer and expose common ports per language
- **Dev profiles** — named configs for different workflows
- **`tsus upgrade`** — self-update command

---

## Design Guardrails

These will **never** be added:

- Compose replacement or multi-container graphs
- Kubernetes support
- Production deployment modes
- Hidden background services or daemons

Theseus must remain thin. If complexity increases, reconsider.

---

## Definition of Success

A student can:

1. Install Docker
2. Run `curl -fsSL ... | sh` to install Theseus
3. Run `tsus run`
4. See their project working

Without reading Docker documentation.

---

## Tech Stack

| Component | Choice | Rationale |
|---|---|---|
| Language | Rust | Single static binary, zero runtime deps, cross-platform |
| CLI Parsing | `clap` | Derive macros, great help output, de facto standard |
| Error Handling | `anyhow` | Clean, student-friendly error context |
| Process Execution | `std::process::Command` | Synchronous Docker calls, no async overhead |
| Installer | Bash | Plain, minimal, cross-platform |
| CI/CD | GitHub Actions | Auto-compile and release on tag push |