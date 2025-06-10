# CLAUDE.md

This file provides guidance to Claude Code when working with this repository.

## 🔨 Most Important Rule - Process for Adding New Rules

When receiving instructions from a user that seem to require consistent application beyond a one-time request:

1. Ask, "Would you like this to be a standard rule?"
2. If the response is YES, add it as an additional rule in `CLAUDE.md`.
3. Apply it consistently as a standard rule thereafter.

This process allows for continuous improvement of the project's rules.
## Project Overview

River.duxca.com is a web application for processing river map information. It provides a platform for displaying information useful for river activities like canoeing, kayaking, and SUP on maps, allowing users to add and share information.

## Architecture

### Backend (`server/`)
- Axum web framework in Rust
- OAuth authentication (GitHub, Facebook, Twitter)
- RESTful API endpoints
- Admin management interface

### Frontend (`browser/`)
- Yew framework SPA (Single Page Application) in Rust
- Leaflet.js map display with Rust wrapper
- Multiple map layers (GSI tiles, OpenStreetMap)
- River information, waypoints, and track display/editing

### Database (`db/`)
- SQLite3 with sqlx ORM
- Litestream for GCS replication
- Schema management via migrations in `db/migrations/`
- Migration commands: `cd db && sqlx migrate run`
- Database reset: `cd db && sqlx database reset`

### Domain Model (`model/`)
- Shared types between server, browser, and database
- Core structs: River, RiverWaypoint, RiverTrack
- API request/response types
- User authentication types

### Service Layer (`service/`)
- API implementation logic
- User permission checks
- Database integration
- Business logic

## Core Features

- **Map Display**: Multiple layers (GSI tiles, OpenStreetMap, aerial photos)
- **River Management**: Name, location, description registration/display
- **Waypoint Management**: Specific points (launch areas, hazards) registration/display
- **Track Management**: River route information registration/display
- **Authentication**: OAuth (GitHub, Facebook, Twitter)
- **Authorization**: User and admin role separation
- **Access Logging**: User API access tracking

## Tech Stack

- **Language**: Rust
- **Backend**: Axum
- **Frontend**: Yew (WebAssembly)
- **Maps**: Leaflet.js (Rust wrapper)
- **Database**: SQLite3, sqlx
- **Backup**: Litestream
- **Authentication**: axum-login, OAuth
- **Templates**: askama
- **Build**: trunk (WebAssembly)
- **Cloud**: Google Cloud Run, Google Cloud Storage

## Development Commands

### Local Development
```bash
# Start both frontend and backend
./run_local.bash

# Code quality checks
cargo clippy -- -D warnings
make fmt
make check

# Database operations
cd db && sqlx migrate run
./reset_local_db.bash
```

### Database Management
```bash
# Reset database and apply migrations
cd db && sqlx database reset -y

# Inspect schema
sqlite3 river.db
> .mode line
> .schema
```

### Frontend Development
```bash
cd browser
trunk watch --features=local  # Hot reload
trunk build --release        # Production build
```

### Environment Setup

For local development, start the fake-gcs-server:
```bash
docker-compose up -d
```
Server available at http://localhost:4443 for GCS operations testing.

## Code Quality Guidelines

### Efficient Code Investigation
- Use targeted searches instead of reading entire files
- Run `cargo modules structure -p <crate_name>` before reading files
- Check README.md files first
- Use search tools:
  - `git grep "pub fn function_name"` for public Rust functions
  - `rg function_name` for general searches
  - `git ls-files` for file listings

### Rust Standards
- **Scoped imports**: Place `use` statements inside functions, not at file top
  - Prevents namespace pollution
  - Use fully qualified paths (e.g., `sqlx::sqlite::SqliteConnection`)
  - Prefer `std::result::Result` over `anyhow::Result`
- **Async functions**: Functions with side effects must be `async fn` or return `impl Future`

### Database Operations
- Use `sqlx::query!` macros for compile-time SQL checking
- Available MCP tools: `list_tables`, `read_query`, `write_query`, `create_table`, `describe_table`

## Data Model

Core entities:
- **Rivers**: Geographic features with name, location, description
- **RiverWaypoints**: Point locations (launch spots, hazards)
- **RiverTracks**: GPS route data as JSON arrays
- **Users**: Authentication with role-based permissions
- **Files**: Metadata for GCS-stored content

## Deployment

- Google Cloud Run containerized deployment
- Litestream SQLite backup to GCS
- GitHub Actions CI/CD
- Custom domain configuration

### Cloud Setup Requirements

Two service accounts needed:
1. **Cloud Run execution**: `roles/secretmanager.secretAccessor`
2. **Litestream**: Storage admin permissions for GCS operations

### OAuth Configuration

Set up OAuth applications:
- GitHub: https://github.com/settings/applications/
- Facebook: https://developers.facebook.com/
- Twitter: https://developer.twitter.com/

## Development Patterns

### Adding Features
- Frontend forms in `add_river.rs`, `add_waypoint.rs`
- API endpoints in `server/src/web/api.rs`
- Business logic in `service/`
- Database operations with sqlx macros

### Map Integration
- Leaflet instance in `map.rs` component
- Multiple layer support
- Coordinate precision handling

### File Storage
- Google Cloud Storage integration
- Database metadata in `files` table
- Service account authentication