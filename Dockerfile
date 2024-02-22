# -----------------------------------------------
# Builder

# Get started with a build env with Rust nightly
FROM rustlang/rust:nightly-bullseye as builder

# Install cargo-binstall, which makes it easier to install other
# cargo extensions like cargo-leptos
RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN cp cargo-binstall /usr/local/cargo/bin

# Install cargo-leptos
RUN cargo binstall cargo-leptos -y

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

# Make an /app dir, which everything will eventually live in
RUN mkdir -p /app
WORKDIR /app
COPY Cargo.toml Cargo.lock build.rs diesel.toml rust-toolchain.toml ./
COPY migrations migrations/
COPY src src/
COPY style style/
COPY public public/

# Build the DB migration excutable
RUN cargo build --bin migrate-db --features ssr --release -vv

# Build the app
RUN cargo leptos build --release -vv

# -----------------------------------------------
# Runner

FROM rustlang/rust:nightly-bullseye-slim as runner
WORKDIR /app

# Install LiteFS
RUN apt-get update -y && apt-get install -y ca-certificates fuse3 sqlite3
COPY --from=flyio/litefs:0.5 /usr/local/bin/litefs /usr/local/bin/litefs
COPY litefs.yml /etc/litefs.yml

# Copy the DB migration binary to the /app directory
COPY --from=builder /app/target/release/migrate-db /app/

# Copy the server binary to the /app directory
COPY --from=builder /app/target/release/moderato /app/

# /target/site contains our JS/WASM/CSS, etc.
COPY --from=builder /app/target/site /app/site
# Copy Cargo.toml if it’s needed at runtime
COPY --from=builder /app/Cargo.toml /app/

# Set any required env variables and
ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT="site"
ENV DATABASE_URL="/litefs/moderato.sqlite3"
EXPOSE 8080

# Run the server
ENTRYPOINT ["litefs", "mount"]
