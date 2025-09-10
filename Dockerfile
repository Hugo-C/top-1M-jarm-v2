FROM rust:1.74-bullseye AS build-stage

ARG BUILD_OPTIONS="--release"

WORKDIR /app

# Create blank project
RUN USER=root cargo new project

# We want dependencies cached, so copy those first.
COPY Cargo.toml /app/project
COPY Cargo.lock /app/project

WORKDIR /app/project

# This is a dummy build to get the dependencies cached.
RUN cargo build $BUILD_OPTIONS

# Now copy in the rest of the sources
COPY . /app/project/

# This is the actual build, touch the main.rs to have newer timestamp
RUN touch /app/project/src/main.rs && cargo build $BUILD_OPTIONS

FROM debian:bullseye-slim AS run-stage
RUN apt-get update -y && apt-get install -y ca-certificates openssl  # Required to have make TLS requests
RUN mkdir /app
COPY --from=build-stage /app/project/target/release/top-1m-jarm-v2 /app
RUN chown -R 1001:1001 /app
USER 1001
WORKDIR /app
ENTRYPOINT ["/app/top-1m-jarm-v2"]