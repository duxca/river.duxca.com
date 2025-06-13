# Server

Axum web framework backend for river.duxca.com

## OAuth Authentication

### GitHub OAuth Flow

The server implements OAuth2 authentication flow with GitHub:

#### Configuration
- GitHub client ID/secret from environment variables
- Endpoints: `/login/github` (POST), `/oauth/callback/github` (GET)
- Local development support with separate credentials

#### Implementation Files
- `src/web/login/github.rs` - GitHub OAuth implementation
- `src/web/login/mod.rs` - Authentication backend
- Database schema in `../db/migrations/20240729163101_users.sql`

#### Flow Process
1. **Login initiation** (`POST /login/github`):
   - Creates OAuth2 client with GitHub endpoints
   - Generates authorization URL with CSRF token
   - Stores CSRF token in session
   - Redirects user to GitHub

2. **Callback handling** (`GET /oauth/callback/github`):
   - Validates CSRF token from session
   - Exchanges authorization code for access token
   - Fetches user info from GitHub API
   - Creates or links user account in database
   - Establishes authenticated session

#### Security Features
- CSRF protection using state parameter validation
- Secure session management with expiration
- Support for linking multiple OAuth providers to single user
- Role-based access control (admin/user)

#### Database Schema
- `users` table: user_id, nickname, role
- `user_auths` table: links OAuth providers to users
- `identity_type`: 0=GitHub, 1=Facebook, 2=Twitter

## Development

```bash
# Run server locally
cargo run

# With database reset
./reset_local_db.bash && cargo run
```