# syntax=docker/dockerfile:1
FROM debian:bookworm-slim

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
COPY .docker-build/server /app/server
COPY .docker-build/site /app/target/site

RUN chmod +x /app/litestream /app/run.bash /app/server

ENV HOST_ADDR=0.0.0.0:8080
ENV DATABASE_URL=sqlite://river.db
ENV BASE_URL=https://river.duxca.com
ENV LOCAL_CLIENT_ID=local
ENV LOCAL_CLIENT_SECRET=local
ENV LOCAL_BASE_URL=http://localhost:8080
ENV LEPTOS_OUTPUT_NAME=leptos-browser
ENV LEPTOS_SITE_ROOT=target/site
ENV LEPTOS_SITE_PKG_DIR=pkg

EXPOSE 8080
CMD ["/app/run.bash"]
