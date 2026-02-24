FROM rust:1.75
WORKDIR /app
# We don't COPY here because tus run mounts the host directory to /app
# So everything is immediately available
# For development, cargo run is appropriate
CMD ["cargo", "run"]
