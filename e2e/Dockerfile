FROM rust:1.96.0-bookworm

ENV PATH=/usr/local/cargo/bin:$PATH

RUN apt-get update && apt-get install -y --no-install-recommends \
  chromium \
  chromium-driver \
  curl \
  libsqlite3-dev \
  pkg-config \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /work
COPY . .

ENV WEBDRIVER_URL=http://127.0.0.1:9515

CMD ["bash", "-lc", "chromedriver --port=9515 --allowed-ips=127.0.0.1 >/tmp/chromedriver.log 2>&1 & until curl -fsS \"$WEBDRIVER_URL/status\" >/dev/null; do sleep 0.2; done; /usr/local/cargo/bin/cargo test -p e2e --features webdriver --test smoke -- --nocapture"]
