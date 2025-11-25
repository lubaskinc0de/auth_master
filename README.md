# Auth Master

**Universal authentication service with multiple identity provider support**

Auth Master is a centralized authentication service that provides a unified interface for various authentication methods (OAuth2, Telegram, etc.) and maps external identities to internal users.

## Core Concept

The service solves the problem of multiple authentication methods in a distributed system:

-   **Web authentication** via OAuth2-proxy
-   **Telegram authentication** by user ID
-   **Future providers** (mobile tokens, login/password, etc.)

All backend services receive a standardized `X-Auth-User` header containing the internal user ID, decoupling them from the specific authentication method.

## Benefits

-   **Unified User Identity**: Multiple auth methods map to single internal users
-   **Backend Decoupling**: Services don't need to understand auth methods
-   **Extensible**: Easy to add new authentication providers
-   **Standardized**: Consistent `X-Auth-User` header across all services

# Integration

soon..

# Internal project structure

## Architecture

### Components

-   **NGINX**: Reverse proxy with `auth_request` module for auth handling
-   **Auth Service**: Core Rust service that handles user mapping and authentication logic
-   **OAuth2 Proxy**: Handles web OAuth2 flows (Google, GitHub, etc.)
-   **PostgreSQL**: Primary database for user and identity storage
-   **Redis**: Session storage for OAuth2 Proxy

### Authentication Flow

#### Web Authentication (via OAuth2-Proxy)

```
GET /auth/
→ Auth Service calls OAuth2-Proxy userinfo endpoint
← 401: Redirect to OAuth2 sign-in
← 200: Extract external_user_id from headers
→ Find/Create user with external_user_id (type: WEB)
← Return user id
```

#### Telegram Authentication

```
GET /auth/
→ Auth Service checks X-Telegram-Key signature
← Invalid: 401 Unauthorized
← Valid: Extract external_user_id from X-Telegram-Id
→ Find/Create user with external_user_id (type: TELEGRAM_USER)
← Return user id
```

## Prerequisites

-   [Docker](https://www.docker.com/get-started)
-   [just](https://github.com/casey/just)

## Getting Started

### 1. Configure Environment Variables

**PostgreSQL Configuration:**

```bash
cp .env.postgres.example .env.postgres
```

**OAuth2 Proxy Configuration:**

```bash
cp .env.oauthproxy.example .env.oauthproxy
just cookie-secret  # Generate cookie secret
# Edit .env.oauthproxy with your OAuth2 credentials
```

**Application Configuration:**

```ini
# Database
AUTHMASTER_DB_HOST=postgres
AUTHMASTER_DB_PORT=5432
AUTHMASTER_DB_NAME=postgres
AUTHMASTER_DB_USER=postgres
AUTHMASTER_DB_PASSWORD=postgrespassword

# External Auth Endpoints
AUTHMASTER_WEB_ENDPOINT_USERINFO=http://oauth2-proxy:8000/oauth2/userinfo
AUTHMASTER_WEB_ENDPOINT_SIGN_IN=http://localhost/oauth2/sign_in

# Telegram Auth (future)
AUTHMASTER_TELEGRAM_BOT_TOKEN=your_bot_token
```

### 2. Build and Run

```bash
just up
```

The service will be available at `http://localhost`

## Usage Examples

### Protected API Call

```bash
# Web flow (handled by OAuth2-Proxy + Auth Service)
curl -H "X-Client-Type: WEB" http://localhost/api/protected

# Telegram flow
curl -H "X-Client-Type: TELEGRAM" \
     -H "X-Telegram-Id: 123456789" \
     -H "X-Telegram-Key: verified_signature" \
     http://localhost/api/protected
```

### Backend Services

All backend services simply check the `X-Auth-User` header:

```rust
let user_id = headers.get("X-Auth-User")
    .except("User not authenticated");
```

## Just Commands

-   `just up` - Build and start all services
-   `just down` - Stop all services
-   `just clean` - Stop services and remove volumes
-   `just infra` - Start only infrastructure
-   `just cookie-secret` - Generate OAuth2 cookie secret

## Project Structure

Rust workspace with three crates:

-   `crates/auth_service` - Main application (clean architecture)
-   `crates/migrations` - Database migrations
-   `crates/shared` - Shared utilities and configs
