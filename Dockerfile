FROM rust:1.98-bookworm AS build
WORKDIR /app
COPY Cargo.toml ./
COPY src ./src
COPY static ./static
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --create-home --uid 10001 appuser
WORKDIR /app
COPY --from=build /app/target/release/gyliber-command-center /usr/local/bin/gyliber-command-center
COPY static ./static
USER appuser
ENV PORT=3000
EXPOSE 3000
CMD ["/usr/local/bin/gyliber-command-center"]
