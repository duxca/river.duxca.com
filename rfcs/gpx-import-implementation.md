# GPX Import Implementation RFC

**Date:** 2025-07-16 19:06:00 UTC  
**Status:** Draft  
**Author:** Analysis of existing codebase and requirements

## Overview

This RFC outlines the implementation plan for adding GPX file import functionality to the river.duxca.com application. The feature will allow users to import GPX files containing river track and waypoint data directly into the system.

## Current State Analysis

### Database Schema

The existing database schema in `db/migrations/20240817131736_rivers.sql` provides:

- **rivers** table: Core river information with representative waypoint
- **river_tracks** table: Track data stored as JSON array `[[lat, long], ...]`
- **river_waypoints** table: Individual waypoints with `[lat, long]` coordinates

### API Architecture

The current API follows a tagged union pattern:
- Request/Response types in `model/src/api/`
- Centralized handler in `service/src/`
- Permission checking via `Request::check_permission()`
- JSON-based communication via POST `/api`

### GPX File Analysis

Two distinct types of GPX files were identified in the `gpx/` directory:

#### Type 1: Geographica App Files (10 files)
- **Creator:** Geographica app
- **Format:** Track data (`<trkpt>` elements)
- **Content:** Detailed GPS tracking with timestamps, elevation, speed
- **Examples:** `20250717_天塩中川.gpx`, `20250717_天塩佐久.gpx`, etc.
- **Size:** 117-282 lines of XML

#### Type 2: NPS (Navigation Planning System) Files (5 files)
- **Creator:** new pec smart app (`jp.mappleon.nps`)
- **Format:** Route data (`<rtept>` elements)
- **Content:** Maritime navigation route planning
- **Examples:** `nps_plan_1_20250717_014758.gpx`, `nps_plan_2_20250717_014758.gpx`
- **Size:** 1,385-1,509 bytes (single-line format)

## Implementation Plan

### 1. Dependencies

Add to `server/Cargo.toml`:
```toml
gpx = "0.10"
geo-types = "0.7"
```

**Rationale:** The `gpx` crate from georust is the most mature and comprehensive GPX parsing library for Rust, supporting both GPX 1.0 and 1.1 formats.

### 2. API Definition

Create `model/src/api/import_gpx.rs`:
```rust
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub river_id: Option<i64>,  // Optional: create new river if not provided
    pub file_content: String,   // Base64 encoded GPX file content
    pub import_options: ImportOptions,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ImportOptions {
    pub import_tracks: bool,
    pub import_routes: bool,
    pub import_waypoints: bool,
    pub create_river_if_missing: bool,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub river_id: i64,
    pub imported_tracks: Vec<i64>,
    pub imported_waypoints: Vec<i64>,
    pub summary: ImportSummary,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ImportSummary {
    pub tracks_imported: u32,
    pub waypoints_imported: u32,
    pub routes_imported: u32,
}
```

### 3. Data Mapping Strategy

#### Track Data Mapping
- **GPX Tracks (`<trk>` → `<trkpt>`)**: Map to `river_tracks` table
- **GPX Routes (`<rte>` → `<rtept>`)**: Map to `river_tracks` table
- **Coordinate Format**: Convert to `Vec<(f64, f64)>` as `[[lat, long], ...]`

#### Waypoint Data Mapping  
- **GPX Waypoints (`<wpt>`)**: Map to `river_waypoints` table
- **Coordinate Format**: Convert to `(f64, f64)` as `[lat, long]`

#### River Creation Logic
- If `river_id` provided: Use existing river
- If `river_id` not provided and `create_river_if_missing=true`: Create new river
- Use GPX metadata (name, description) for river information
- Calculate representative waypoint from track/route centroid

### 4. Service Implementation

Create `service/src/import_gpx.rs`:
```rust
pub async fn import_gpx(
    db: &sqlx::SqlitePool,
    user: &model::user::User,
    request: model::api::import_gpx::Request,
) -> Result<model::api::import_gpx::Response, anyhow::Error> {
    // 1. Parse GPX file using `gpx` crate
    // 2. Extract tracks, routes, and waypoints
    // 3. Create river if needed
    // 4. Insert track data into river_tracks
    // 5. Insert waypoint data into river_waypoints
    // 6. Return summary
}
```

### 5. Permission Model

Grant import permissions to:
- **Admin users (role=0)**: Full import access
- **Regular users (role=1)**: Import to own rivers only
- **Validation**: Ensure user can modify target river

### 6. Error Handling

Handle common failure scenarios:
- Invalid GPX format
- Missing required fields
- Permission denied
- Database constraint violations
- Large file processing limits

### 7. File Upload Mechanism

Two approaches considered:

#### Option A: Multipart Upload
- Use axum multipart for file upload
- Direct file processing without base64 encoding
- More efficient for large files

#### Option B: JSON Base64 (Recommended)
- Consistent with existing API pattern
- Simpler client implementation
- Better error handling integration

## Technical Considerations

### Performance
- **Memory Usage**: Stream parsing for large GPX files
- **Database Transactions**: Atomic imports with rollback capability
- **Validation**: Pre-import validation to prevent partial imports

### Data Integrity
- **Coordinate Validation**: Ensure valid lat/long ranges
- **Duplicate Prevention**: Check for existing tracks/waypoints
- **Referential Integrity**: Ensure river_id exists before import

### Extensibility
- **Plugin Architecture**: Support for additional GPS formats
- **Metadata Preservation**: Store original GPX metadata
- **Batch Processing**: Support for multiple file imports

## Implementation Phases

### Phase 1: Core Implementation
1. Add GPX parsing dependency
2. Implement basic API endpoint
3. Create service layer logic
4. Add database operations

### Phase 2: Enhancement
1. Add comprehensive error handling
2. Implement permission validation
3. Add import options support
4. Create unit tests

### Phase 3: Integration
1. Frontend integration
2. File upload UI
3. Import progress tracking
4. User documentation

## Alternative Approaches Considered

### 1. File Storage Approach
Store GPX files in GCS and reference in database
- **Pros**: Preserves original data, supports large files
- **Cons**: Added complexity, storage costs, processing overhead

### 2. Separate GPX Table
Create dedicated table for GPX metadata
- **Pros**: Clean separation, better queryability
- **Cons**: Schema complexity, data duplication

### 3. Background Processing
Async GPX processing with job queue
- **Pros**: Better UX for large files, scalability
- **Cons**: Implementation complexity, state management

## Conclusion

The proposed implementation provides a robust foundation for GPX import functionality while maintaining consistency with the existing codebase architecture. The use of the mature `gpx` crate ensures reliable parsing of diverse GPX formats encountered in the wild.

The phased approach allows for iterative development and testing, with the core functionality delivering immediate value while advanced features can be added incrementally.