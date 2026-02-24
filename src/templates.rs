pub const NODE_TEMPLATE: &str = r#"FROM node:18-alpine
WORKDIR /app
# We don't COPY here because tsus run mounts the host directory to /app
# So everything is immediately available
CMD ["npm", "start"]
"#;

pub const PYTHON_TEMPLATE: &str = r#"FROM python:3.11-slim
WORKDIR /app
# We don't COPY here because tsus run mounts the host directory to /app
# So everything is immediately available
CMD ["python", "main.py"]
"#;

pub const RUST_TEMPLATE: &str = r#"FROM rust:1.75
WORKDIR /app
# We don't COPY here because tsus run mounts the host directory to /app
# So everything is immediately available
# For development, cargo run is appropriate
CMD ["cargo", "run"]
"#;

pub const GO_TEMPLATE: &str = r#"FROM golang:1.21-alpine
WORKDIR /app
# We don't COPY here because tsus run mounts the host directory to /app
# So everything is immediately available
CMD ["go", "run", "main.go"]
"#;
