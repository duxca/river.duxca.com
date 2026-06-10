# syntax=docker/dockerfile:1
FROM rust:1.96.0-bookworm AS builder

WORKDIR /app

RUN curl -o /tmp/sccache.tgz -L https://github.com/mozilla/sccache/releases/download/0.2.13/sccache-0.2.13-x86_64-unknown-linux-musl.tar.gz && \
  tar xf /tmp/sccache.tgz -C /tmp && \
  mv /tmp/sccache*/sccache /usr/local/bin && \
  rm -rf /tmp/sccache*

RUN \
  --mount=type=cache,target=/var/lib/apt,sharing=locked \
  --mount=type=cache,target=/var/cache/apt,sharing=locked \
  apt-get update && apt-get install -y \
  libsqlite3-dev

ENV CARGO_HOME=/var/cache/cargo
ENV PATH=/var/cache/cargo/bin:$PATH
ENV RUSTC_WRAPPER=/usr/local/bin/sccache
ENV SCCACHE_DIR=/var/cache/sccache

COPY . .

RUN \
  --mount=type=cache,target=/var/cache/cargo \
  --mount=type=cache,target=/var/cache/sccache \
  cargo fetch --locked

RUN rustup target add wasm32-unknown-unknown

RUN \
  --mount=type=cache,target=/var/cache/cargo \
  --mount=type=cache,target=/var/cache/sccache \
  cargo install trunk --version 0.21.14 --locked

# RUN cargo sqlx migrate run

RUN \
  --mount=type=cache,target=/app/target \
  --mount=type=cache,target=/var/cache/cargo \
  --mount=type=cache,target=/var/cache/sccache \
  SQLX_OFFLINE=true cargo build --offline --release -p server && \
  cp /app/target/release/server /app/server-bin && \
  chmod +x /app/server-bin

RUN \
  --mount=type=cache,target=/app/leptos-browser/target \
  --mount=type=cache,target=/var/cache/cargo \
  --mount=type=cache,target=/var/cache/sccache \
  cd /app/leptos-browser && \
  trunk build --release

ADD https://github.com/benbjohnson/litestream/releases/download/v0.5.12/litestream-0.5.12-linux-x86_64.tar.gz /tmp/litestream.tar.gz
RUN tar -C ./ -xzf /tmp/litestream.tar.gz

FROM debian:bookworm-slim

WORKDIR /app

RUN \
  --mount=type=cache,target=/var/lib/apt,sharing=locked \
  --mount=type=cache,target=/var/cache/apt,sharing=locked \
  apt-get update && apt-get install -y \
  ca-certificates openssl \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/db/litestream /app/litestream
COPY --from=builder /app/db/litestream.yml /app/litestream.yml
COPY --from=builder /app/cli/run.bash /app/run.bash
COPY --from=builder /app/server-bin /app/server
COPY --from=builder /app/leptos-browser/dist /app/dist

ENV HOST_ADDR=0.0.0.0:8080
ENV DATABASE_URL=sqlite://river.db
ENV BASE_URL=https://river.duxca.com
ENV LOCAL_CLIENT_ID=local
ENV LOCAL_CLIENT_SECRET=local
ENV LOCAL_BASE_URL=http://localhost:8080
ENV LOCAL_DIST_PATH=dist
ENV GCS_BUCKET_NAME=duxca-litestream-sandbox

EXPOSE 8080
CMD ["/app/run.bash"]
