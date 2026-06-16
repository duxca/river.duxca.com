SHELL := /usr/bin/env bash
.SHELLFLAGS := -euo pipefail -c

CARGO_LEPTOS_VERSION := 0.3.6
SQLX_CLI_VERSION := 0.8.6
DATABASE_URL ?= sqlite://.local/river-dev.db?mode=rwc
PROJECT_ID ?= duxca-298210
REGION ?= asia-northeast1
IMAGE_REPOSITORY ?= $(REGION)-docker.pkg.dev/$(PROJECT_ID)/cloud-run-source-deploy/litestream-sandbox
IMAGE_TAG ?= latest
DEPLOY_METADATA ?= .local/deploy-image-metadata.json
DEPLOY_TERRAFORM_PLAN ?= ../.local/tfplan
DEPLOY_TERRAFORM_REFRESH ?= false

.DEFAULT_GOAL := help

.PHONY: help
help:
	@printf '%s\n' \
		'Targets:' \
		'  make setup             install local build tools' \
		'  make sqlx-db           prepare local sqlite schema for sqlx checks' \
		'  make serve             run local hot-reload server' \
		'  make build             build release server and frontend' \
		'  make test              run Rust tests' \
		'  make test-e2e          run cargo-leptos e2e lifecycle with container browser' \
		'  make fmt               format Rust, Cargo.toml, and Terraform files' \
		'  make fmt-check         check Rust, Cargo.toml, and Terraform formatting' \
		'  make clippy            run clippy like CI' \
		'  make check             run fmt-check, clippy, and test' \
		'  make check-ci          run check plus e2e and terraform validation' \
		'  make terraform-check   run terraform init, validate, and fmt check' \
		'  make deploy            build, push, and terraform apply from local main checkout' \
		'  make clean             remove build output'

.PHONY: setup
setup:
	rustup target add wasm32-unknown-unknown
	if ! command -v cargo-binstall >/dev/null 2>&1; then \
		curl -L --proto '=https' --tlsv1.2 -sSf \
			https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash; \
	fi
	cargo leptos --version >/dev/null 2>&1 || cargo binstall -y cargo-leptos@$(CARGO_LEPTOS_VERSION)
	cargo sqlx --version >/dev/null 2>&1 || cargo binstall -y sqlx-cli@$(SQLX_CLI_VERSION)

.PHONY: sqlx-db
sqlx-db:
	mkdir -p .local
	cargo sqlx database setup --source db/migrations --no-dotenv --database-url '$(DATABASE_URL)'

.PHONY: serve
serve:
	./cli/dev-local.sh

.PHONY: build
build: sqlx-db
	DATABASE_URL='$(DATABASE_URL)' cargo leptos build --release

.PHONY: test
test: sqlx-db
	DATABASE_URL='$(DATABASE_URL)' cargo test

.PHONY: test-e2e
test-e2e:
	./cli/e2e-cargo-leptos.sh

.PHONY: fmt
fmt:
	cargo fmt --all
	find . -name Cargo.toml -print0 | while IFS= read -r -d '' manifest; do \
		(cd "$$(dirname "$$manifest")" && cargo tomlfmt); \
	done
	terraform -chdir=terraform fmt -recursive

.PHONY: fmt-check
fmt-check:
	cargo fmt --all -- --check
	find . -name Cargo.toml.new -delete
	trap 'find . -name Cargo.toml.new -delete' EXIT; \
	find . -name Cargo.toml -print0 | while IFS= read -r -d '' manifest; do \
		(cd "$$(dirname "$$manifest")" && cargo tomlfmt -d); \
	done
	terraform -chdir=terraform fmt -recursive -check

.PHONY: clippy
clippy: sqlx-db
	DATABASE_URL='$(DATABASE_URL)' cargo clippy --all-targets --all-features -- -Dwarnings

.PHONY: check
check: fmt-check clippy test

.PHONY: check-ci
check-ci: check test-e2e terraform-check

.PHONY: terraform-check
terraform-check:
	terraform -chdir=terraform init
	terraform -chdir=terraform validate
	terraform -chdir=terraform fmt -check

.PHONY: deploy
deploy:
	mkdir -p .local
	gcloud auth configure-docker '$(REGION)-docker.pkg.dev'
	docker buildx build \
		--push \
		--tag '$(IMAGE_REPOSITORY):$(IMAGE_TAG)' \
		--metadata-file '$(DEPLOY_METADATA)' \
		.
	CF_TOKEN="$${CLOUDFLARE_API_TOKEN:-$$(python3 -c 'import pathlib, tomllib; print(tomllib.load(open(pathlib.Path.home() / ".cf/config.toml", "rb"))["access_token"])')}"; \
		CONTAINER_IMAGE='$(IMAGE_REPOSITORY)'@$$(python3 -c 'import json; print(json.load(open("$(DEPLOY_METADATA)"))["containerimage.digest"])'); \
		echo "Deploying $${CONTAINER_IMAGE}"; \
		CLOUDFLARE_API_TOKEN="$${CF_TOKEN}" timeout 5m terraform -chdir=terraform init; \
		CLOUDFLARE_API_TOKEN="$${CF_TOKEN}" timeout 10m terraform -chdir=terraform plan -refresh=$(DEPLOY_TERRAFORM_REFRESH) -out='$(DEPLOY_TERRAFORM_PLAN)' -var="container_image=$${CONTAINER_IMAGE}"; \
		CLOUDFLARE_API_TOKEN="$${CF_TOKEN}" timeout 20m terraform -chdir=terraform apply -refresh=$(DEPLOY_TERRAFORM_REFRESH) -auto-approve '$(DEPLOY_TERRAFORM_PLAN)'; \
		terraform -chdir=terraform output -raw cloud_run_url

.PHONY: clean
clean:
	cargo clean
	rm -rf db/target target/front target/site .docker-build
