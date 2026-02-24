# Theseus (`tsus`)

**Theseus** is a student-friendly, ergonomic wrapper around Docker designed to eliminate environment setup pain for beginners. It provides a simple, Heroku-like developer experience (`tsus run`) while remaining entirely transparent: it simply orchestrates standard Docker commands under the hood.

## Why Theseus?

Learning Docker is hard. Learning your first programming language is hard enough.
Theseus aims to bridge the gap by automatically generating Dockerfiles for common languages (`Node.js`, `Python`, `Rust`, `Go`) and mapping your local directory into the container, without you needing to write a single line of Dockerfile or `docker run` command.

When you're ready to learn Docker, Theseus doesn't hide anything. Any command it doesn't recognize is passed directly through to the Docker CLI.

---

## Installation

```bash
cargo install --path .
```

*Prerequisite: You must have Docker installed and running on your system.*

---

## Commands

Theseus provides 7 core commands. All other commands are forwarded directly to Docker.

### `tsus run`

The core of Theseus. Simply navigate to your project directory and type `tsus run`.

1. If a `Dockerfile` exists, `tsus` will use it.
2. If no `Dockerfile` exists, `tsus` detects the language of your project (by looking for `package.json`, `Cargo.toml`, etc.) and automatically generates a temporary development `Dockerfile`.
3. `tsus` builds the image and runs it interactively.
4. Your current working directory is dynamically mounted to `/app` inside the container, meaning any code changes you make locally are reflected immediately.

### `tsus init <language>`

Generates a minimal, language-specific `Dockerfile` in the current directory for you to customize.
Supported languages: `node`, `python`, `rust`, `go`.

### `tsus stop`

Stops the currently running project container cleanly.

### `tsus status`

Shows whether the project container is currently running, its active ports, and the size of the built image.

### `tsus clean`

Removes the project's Docker image to save disk space.
Use `tsus clean --all` to trigger a safe `docker system prune -f` in addition to cleaning the project image.

### `tsus logs`

Tails the logs of the running project container.

### `tsus exec`

Opens an interactive shell (`/bin/sh`) inside the running project container for live debugging.

### Passthrough

Any command that isn't one of the above 7 is passed right through to `docker`.

```bash
# This behaves exactly like `docker ps`
tsus ps

# This behaves exactly like `docker images`
tsus images
```

---

## Architecture & Identity

Theseus creates 1 Image and 1 Container per directory. It relies on the *name of your current directory* to establish identity.
If your folder is named `my-web-app`, Theseus will predictably name the Docker image and container `my-web-app-theseus`.

It has **no hidden state** or configuration files. It derives everything dynamically, ensuring you can delete the folder or `tsus clean` at any time without leaving debris behind.

---

## Credits

Theseus was built using AntiGravity.
