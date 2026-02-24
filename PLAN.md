# Theseus (tsus) — Project Plan

> A student‑friendly, Docker‑powered container runner with npm‑style ergonomics.

---

# Vision

Theseus is a thin, opinionated CLI layer on top of Docker.

It is:

* Dev‑focused
* Single‑project oriented
* Student‑friendly
* Docker‑compatible (full passthrough)

It is NOT:

* A container runtime
* A Docker replacement
* A compose/orchestration tool
* A production deployment system

---

# Core Design Principles

1. Zero hidden state (delegate everything to Docker)
2. Deterministic behavior
3. Minimal commands (≤ 7 in v1)
4. Auto-build for convenience
5. Full Docker passthrough
6. Clear, friendly output
7. No feature creep

---

# Phase 0 — Architecture & Scope Lock

## Goals

* Finalize CLI surface
* Define command behavior
* Define project boundaries

## Decisions

### Binary Name

* Project: Theseus
* CLI binary: `tsus`

### v1 Commands

Native commands:

* `tsus init`
* `tsus run`
* `tsus stop`
* `tsus status`
* `tsus clean`
* `tsus logs`
* `tsus exec`

All other commands:
→ Forward to `docker`

### Project Scope Constraints

* Single container per project
* No multi-service orchestration
* No compose support in v1
* No background daemon

---

# Phase 1 — Minimal Working CLI (MVP Core)

## 1. CLI Parsing

* Use `clap`
* Define subcommands
* Unknown subcommands → passthrough to Docker

## 2. Docker Passthrough Layer

Implement:

* Forward arguments
* Preserve exit codes
* Stream stdout/stderr
* Preserve signals (Ctrl+C)

Validation:

* `tsus ps` behaves exactly like `docker ps`

---

# Phase 2 — `tsus run` Implementation

## Behavior

`tsus run` should:

1. Detect Dockerfile
2. If exists → use it
3. If not → attempt language detection
4. If detection succeeds → generate temporary Dockerfile
5. If detection fails → show guided message

### Language Detection (v1 only)

* `package.json` → Node
* `requirements.txt` or `pyproject.toml` → Python
* `Cargo.toml` → Rust
* `go.mod` → Go

If multiple detected → prompt user

## Auto-Build Logic

`tsus run`:

* Build if image missing
* Rebuild if Dockerfile changed
* Otherwise reuse

## Runtime Defaults

* Mount current directory → `/app`
* Working directory → `/app`
* Run in foreground
* Auto-remove container on exit

No detached mode in v1.

---

# Phase 3 — `tsus init`

## Purpose

Scaffold minimal project setup.

`tsus init python`
`tsus init node`
`tsus init rust`

Should generate:

* Minimal Dockerfile (dev-focused)
* Optional basic README hint

Rules:

* Do not overwrite existing Dockerfile without confirmation
* Keep templates minimal

---

# Phase 4 — Project Identity & Naming

## Image Naming Strategy

Derive from folder name:

`<folder>-theseus`

Example:
`myapp-theseus`

## Container Naming Strategy

Same as image name.

Avoid random hashes in student UX layer.

---

# Phase 5 — Supporting Commands

## `tus stop`

* Stop container
* Remove container

## `tus status`

Show:

* Running or not
* Bound port (if known)
* Image exists?
* Disk usage estimate

Friendly output only.

## `tus clean`

Options:

* Clean project image only
* Or `--all` → system prune

## `tus logs`

* Tail logs of project container

## `tus exec`

* Default: open shell

---

# Phase 6 — UX & Error Handling

## Friendly Errors

Example:

"No project type detected.

Theseus looked for:

* package.json
* requirements.txt / pyproject.toml
* Cargo.toml
* go.mod

Run:
tsus init <language>
"

## Transparency

When auto-generating Dockerfile:

"Detected Python project. Using temporary development container."

Never hide behavior.

---

# Phase 7 — Testing & Stability

## Test Matrix

* macOS
* Linux

Test cases:

* Existing Dockerfile
* Auto-detected project
* Missing Docker
* Container already running
* Interrupted build

Ensure:

* Exit codes propagate
* Ctrl+C behaves correctly

---

# Phase 8 — Documentation & Positioning

## README Structure

1. What is Theseus?
2. Why not Docker directly?
3. Philosophy
4. Install
5. Quickstart
6. Command reference
7. Passthrough explanation

Positioning line:

"Theseus gives Docker an npm-style workflow for students."

---

# Phase 9 — v1 Release Criteria

Before tagging v1.0:

* ≤ 7 core commands
* No multi-service support
* No config file required
* Full Docker passthrough verified
* Clean error messages
* No hidden state

---

# Future (Post v1 — Only If Needed)

* Hot reload integration
* Optional `theseus.toml`
* Port auto-detection
* Dev profiles

Only add if real users request.

---

# Guardrails (Read Before Adding Features)

Do NOT add:

* Compose replacement
* Multi-container graphs
* Kubernetes support
* Production deployment modes
* Hidden background services

If complexity increases, reconsider.

Theseus must remain thin.

---

# Definition of Success

A student can:

1. Install Docker
2. Install Theseus
3. Run `tsus run`
4. See their project working

Without reading Docker documentation.

That’s it.

---

# Phase 10 — Distribution & Installer

## Strategy: Curl Installer First

Homebrew comes later.
First priority is a simple, reliable curl-based installer.

Goal:

Proper Curl Installer Without Buying a Domain
Option 1 (Most Common for Indie Tools)

Host installer script in your repo:

https://raw.githubusercontent.com/<username>/theseus/main/install.sh

Users run:

curl -fsSL https://raw.githubusercontent.com/<username>/theseus/main/install.sh | sh

Zero cost.
Standard pattern.
Works perfectly.

Even Cleaner (Version-Pinned Installer)

Better long-term:

https://raw.githubusercontent.com/<username>/theseus/v0.1.0/install.sh

That prevents accidental breaking changes.

Where Binaries Live

Your installer should fetch from GitHub Releases:

https://github.com/<username>/theseus/releases/latest/download/tus-macos-arm64

That’s industry-standard.

No domain needed.

Even Better UX (Optional Later)

You can shorten it with:

curl -fsSL https://git.io/theseus | sh

But that’s polish. Not needed early.

Important Security Note

Never recommend:

curl | sh

without making the script readable.

In your README, always show:

curl -fsSL ... -o install.sh
cat install.sh
sh install.sh

Even if most users won’t inspect it.

It builds trust.

So Your Real Installer URL Should Be
https://raw.githubusercontent.com/<username>/theseus/main/install.sh

Simple.
Free.
Professional.
No domain.

Recommended Install Flow
Step 1 — Detect OS + Architecture
uname -s   # Darwin or Linux
uname -m   # arm64, x86_64

Map to correct binary.

Step 2 — Download Binary

From GitHub releases:

https://github.com/<username>/theseus/releases/latest/download/tus-<os>-<arch>

Save temporarily.

Step 3 — Choose Install Location

Logic:

if [ -w "/usr/local/bin" ]; then
    install_path="/usr/local/bin"
else
    install_path="$HOME/.local/bin"
fi

If $HOME/.local/bin doesn’t exist:

mkdir -p "$HOME/.local/bin"
Step 4 — Move + chmod
mv tus "$install_path/"
chmod +x "$install_path/tus"
Step 5 — PATH Check

If install path not in $PATH:

Print:

Add this to your shell config:

export PATH="$HOME/.local/bin:$PATH"

Do not auto-edit.

Step 6 — Print Success
✔ Theseus installed successfully.
Run: tus doctor
Why This Is the True “Least Resistance”

Works without sudo in most cases.

Doesn’t break system.

Doesn’t modify shell silently.

Works on macOS + Linux.

Matches how many CLI tools install.

Students won’t care about elegance.
They care that tus works after install.

One More Important Decision

Do you want the installer to:

A) Always install latest version
B) Allow version pinning via env variable

Example:

TUS_VERSION=v0.1.0 curl ...

For v1, you can default to latest.
Keep it simple.

It must be:

* Deterministic
* Transparent
* Safe
* Cross-platform (macOS + Linux)

---

## Phase 10A — Release Infrastructure

Before installer:

1. GitHub Releases must publish:

   * macOS (Intel)
   * macOS (ARM)
   * Linux (x86_64)

2. Each release must include:

   * Versioned binary
   * SHA256 checksums file

3. Tag format:

   * v0.1.0
   * v0.1.1

Installer should always fetch latest stable release.

---

## Phase 10B — Installer Script Design

### Responsibilities

The install script should:

1. Detect OS (Darwin / Linux)
2. Detect architecture (x86_64 / arm64)
3. Fetch correct binary
4. Verify checksum (strongly recommended)
5. Move binary to:

   * `/usr/local/bin` (if writable)
   * Else `$HOME/.local/bin`
6. Make executable
7. Print success message
8. Suggest running:

`tus doctor`

---

## Safety Rules

The script must:

* Use `set -e`
* Avoid sudo unless required
* Print what it is doing
* Fail clearly

Never silently modify shell configs.

---

## Phase 10C — `tus doctor`

Add a simple diagnostic command.

Checks:

* Docker installed?
* Docker daemon running?
* Docker version ≥ minimum?
* PATH contains tus?

Output example:

✔ Docker installed
✔ Docker daemon running
✔ Theseus ready

If failure:

✖ Docker not detected.
Install Docker from: [https://docs.docker.com/get-docker/](https://docs.docker.com/get-docker/)

---

## Phase 10D — Version Upgrades

Installer behavior:

If tus already installed:

* Overwrite with latest version
* Print old → new version

Later enhancement:

`tus upgrade`

Which re-runs installer logic.

---

## When to Add Homebrew

Only after:

* Stable v1 release
* Verified macOS adoption
* Installer proven stable
* Clear upgrade path

Homebrew is distribution polish.
Curl installer is validation phase.

---

## Success Criteria for Installer

A student can run:

`curl -fsSL https://theseus.sh/install | sh`

Then:

`tus run`

With no additional configuration.

Installation must take < 30 seconds.

If installation friction exists, fix installer before adding features.

Rust is the correct choice for this project.

Not because it’s trendy. Because it aligns with what Theseus actually is:

CLI-first

Systems-adjacent

Process-spawning heavy

Cross-platform

Single static binary

Zero runtime dependency

That’s Rust territory.

Now let’s define the stack properly so you don’t over-engineer it.

Core Stack (Keep It Lean)
1️⃣ CLI Parsing — clap

Why:

Mature

Clean derive macros

Great help output

De facto Rust CLI standard

You do NOT need:

fancy TUI

interactive wizard libraries

nested command frameworks

Just clap.

2️⃣ Error Handling — anyhow

For a CLI tool:

You don’t need complex typed errors early.

anyhow::Result is clean and pragmatic.

Later, if it grows, you can move to thiserror.

For now: keep it simple.

3️⃣ Process Execution — std::process::Command

Do not bring in:

async runtime

tokio

complex subprocess crates

Docker calls are synchronous.
Blocking is fine.
Keep it boring.

4️⃣ Optional (Maybe Later)
serde + toml

Only if you introduce a config file.

You said:

behave like Docker by default

So you may not even need config parsing in v1.

5️⃣ Installer Script

Bash.
Plain.
Minimal.
No Rust involvement.

Architecture Structure

You want something like:

src/
 ├── main.rs
 ├── cli.rs
 ├── docker.rs
 ├── commands/
 │     ├── run.rs
 │     ├── init.rs
 │     ├── stop.rs
 │     ├── status.rs
 │     ├── clean.rs
 │     ├── logs.rs
 │     └── exec.rs
 └── detect.rs

Keep commands separated.
Keep docker passthrough isolated.
Keep language detection isolated.

No globals.
No hidden state.

Avoid These Early

Async runtime

Multi-threading

Fancy spinners

Color libraries (unless minimal)

Config managers

Background daemon

Logging frameworks

You’re building a thin orchestrator.

The thinner it stays, the more durable it is.

Performance Concerns?

None.

Docker dominates runtime cost.
Rust overhead is negligible.

Long-Term Scalability

Rust also gives you:

Easy static linking

Cross-compilation

Strong binary distribution story

Predictable memory behavior

Good signal for infra credibility

Final Stack Summary

Rust

clap

anyhow

std::process::Command

Bash install script

GitHub Actions for release

That’s it.

If your dependency list exceeds 5 crates in v1, you’re probably drifting.