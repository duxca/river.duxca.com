# syntax=docker/dockerfile:1.7
FROM rust:1.96.0-trixie AS builder

ARG CARGO_LEPTOS_VERSION=0.3.6

WORKDIR /src

RUN \
  --mount=type=cache,target=/var/lib/apt,sharing=locked \
  --mount=type=cache,target=/var/cache/apt,sharing=locked \
  apt-get update && apt-get install -y \
  curl \
  pkg-config \
  libssl-dev \
  sqlite3 \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists/*

RUN rustup component add clippy rustfmt && rustup target add wasm32-unknown-unknown

RUN \
  --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
  --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
  if ! command -v cargo-binstall >/dev/null 2>&1; then \
    curl -L --proto '=https' --tlsv1.2 -sSf \
      https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash; \
  fi; \
  cargo leptos --version >/dev/null 2>&1 || cargo binstall -y cargo-leptos@${CARGO_LEPTOS_VERSION}

COPY . .

RUN \
  --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
  --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
  --mount=type=cache,target=/src/target \
  --mount=type=cache,target=/src/.local \
  set -eu; \
  mkdir -p .local; \
  rm -f .local/river-dev.db; \
  for migration in $(find db/migrations -maxdepth 1 -type f -name '*.sql' | sort); do \
    sqlite3 .local/river-dev.db < "${migration}"; \
  done; \
  DATABASE_URL='sqlite://.local/river-dev.db?mode=rwc' cargo leptos build --release; \
  mkdir -p /out/site; \
  cp target/release/server /out/server; \
  cp -a target/site/. /out/site/

FROM debian:trixie-slim

WORKDIR /app

RUN \
  --mount=type=cache,target=/var/lib/apt,sharing=locked \
  --mount=type=cache,target=/var/cache/apt,sharing=locked \
  apt-get update && apt-get install -y \
  ca-certificates openssl \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists/*

COPY db/litestream /app/litestream
COPY db/litestream.yml /app/litestream.yml
COPY cli/run.bash /app/run.bash
COPY --from=builder /out/server /app/server
COPY --from=builder /out/site /app/target/site

RUN chmod +x /app/litestream /app/run.bash /app/server

ENV HOST_ADDR=0.0.0.0:8080
ENV DATABASE_URL=sqlite:///app/river.db?mode=rwc
ENV BASE_URL=https://river.duxca.com
ENV LOCAL_CLIENT_ID=local
ENV LOCAL_CLIENT_SECRET=local
ENV LOCAL_BASE_URL=http://localhost:8080
ENV LEPTOS_OUTPUT_NAME=frontend
ENV LEPTOS_SITE_ROOT=target/site
ENV LEPTOS_SITE_PKG_DIR=pkg

EXPOSE 8080
CMD ["/app/run.bash"]
