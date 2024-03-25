# Rust as the base image with a newer version
FROM rust:1.75.0 as build

# Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY .env .env
COPY ./src ./src

# Build for release.
RUN cargo build --release

# The final base image
FROM debian:buster-slim

# Copy from the previous build
COPY --from=build /target/release/fingerprint_api /usr/src/fingerprint_api

# Run the binary
CMD ["/usr/src/fingerprint_api"]
