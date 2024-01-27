FROM rustlang/rust:nightly-bullseye as builder
RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN mkdir -p /app
WORKDIR /app
COPY . .
RUN cargo update
RUN cargo build --release --no-default-features --features=ssr
RUN ls -l /app/target

FROM rustlang/rust:nightly-bullseye as runner
COPY --from=builder /app/target/release/hackernews_islands_axum /app/
COPY --from=builder /app/pkg /app
COPY --from=builder /app/Cargo.toml /app/
WORKDIR /app
ENV RUST_LOG="info"
ENV LEPTOS_OUTPUT_NAME="hackernews_islands_axum"
ENV APP_ENVIRONMENT="production"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 8080
CMD ["/app/hackernews_islands_axum"]
