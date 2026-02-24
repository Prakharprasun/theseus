# Theseus (tsus) — Implementation Plan

This document outlines the technical implementation strategy for the `tsus` CLI, based on the architectural vision and requirements specified in `PLAN.md`.

## 1. Core Technology Stack
- **Language**: Rust
- **CLI Framework**: `clap` (using the Derive API) for robust argument parsing and helpful documentation.
- **Subprocess Management**: `std::process::Command` to wrap and invoke the underlying `docker` binary. This ensures we don't need to rebuild Docker's API bindings—we just act as a smart proxy.
- **Error Handling**: `anyhow` for rich, student-friendly error context.

## 2. CLI Router & Passthrough
The CLI router will intercept the predefined 7 core commands. Any unrecognized commands will be handled via a "catch-all" or external subcommand pattern in `clap` and seamlessly proxied to the `docker` CLI.

**Passthrough Requirements:**
- Stdin/Stdout/Stderr must be fully transparent (`Stdio::inherit()`).
- Exit codes of the underlying Docker process must be preserved and bubbled up.
- Unix signals (like `SIGINT` / Ctrl+C) must be passed cleanly to the child process.

## 3. Implementation Phases

### Phase 1: MVP & Passthrough Core
1. Initialize the Rust project (`cargo new theseus`).
2. Implement the `clap` parser with the 7 core subcommands.
3. Build the catch-all execution function block.
4. Verify `tsus ps` works identically to `docker ps`.

### Phase 2: Project Identity
1. Create a utility module to determine the project name from the current working directory (`env::current_dir()`).
2. Standardize naming convention: `Image = <dir_name>-theseus`, `Container = <dir_name>-theseus`.

### Phase 3: Implement `tsus run`
1. **Dockerfile Detection**: Check if `Dockerfile` exists in the local directory.
2. **Auto-Detection (Fallback)**:
   - Check heuristics (`package.json`, `requirements.txt`, etc.).
   - If matched, construct and write a temp Dockerfile. If multiple matches occur, prompt the user.
3. **Build Step**: Execute `docker build -t <name> .` (rebuild if `Dockerfile` is newer than the image).
4. **Run Step**: Execute `docker run -it --rm -v "$PWD:/app" -w /app --name <name> <image_name>`.

### Phase 4: Implement `tsus init`
1. Define static Dockerfile templates (Node, Python, Rust, Go) within the Rust binary.
2. Accept a language parameter.
3. Check for an existing `Dockerfile` and gracefully abort or prompt to avoid overwriting.
4. Write the template to the disk and print a friendly success/instructional message.

### Phase 5: Implement Supporting Commands
- **`stop`**: Run `docker stop <name>` and `docker rm <name>`.
- **`status`**: Wrap `docker inspect <name>` and `docker images <name>` to parse out and print a friendly status summary.
- **`clean`**: Handle `docker rmi <name>` or conditionally run `docker system prune` if `--all` is passed.
- **`logs`**: Run `docker logs -f <name>`.
- **`exec`**: Run `docker exec -it <name> /bin/sh` to drop the user into the container shell.

### Phase 6: UX Polish
- Implement a logging or messaging module to ensure standard, color-coded Output (e.g., using `colored` crate).
- Ensure errors like "Docker daemon not running" are caught elegantly and explained simply to the student.

## 4. Testing & Validation
- Write inline unit tests for the deterministic components (e.g., Project naming, Language detection logic).
- Set up integration tests that invoke the compiled `tus` binary to verify argument passthrough behaves correctly.
