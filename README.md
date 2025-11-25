# Auth Master

> [!CAUTION]
> This project is not ready for production, it is still in development.

**Universal self-hosted blazingly fast authentication service written in Rust**

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

## Supported client types

1. **Web** (X-Client-Type: Web)
2. **Telegram** (X-Client-Type: Telegram, planned)
3. **Mobile** (X-Client-Type: Mobile, planned)

## Supported authentication methods

1. **Web**
    1. [OAuth2-Proxy](https://github.com/oauth2-proxy/oauth2-proxy)
    2. Simple login and password (planned)
2. **Telegram (planned)**
    1. By secret-token and telegram user id (planned)
3. **Mobile (planned)**
    1. Simple access tokens (planned)

# Real World Example

## Problem Scenario

Imagine you're building a system with multiple client types:

-   **Web frontend** - needs OAuth2 authentication (Google, GitHub, etc.)
-   **Telegram bot** - needs simple authentication via Telegram user ID
-   **Future mobile app** - will need token-based authentication

Each service in your backend currently needs to:

-   Understand different authentication methods
-   Handle multiple identity providers
-   Maintain separate user mapping logic

## Solution with Auth Master

### Infrastructure Setup

<img  src="https://imgfy.ru/ib/sQbCMYkbOj3Hs4z_1764066718.png" alt="image.png"/>

### Configuration Example

**NGINX configuration:**
soon...

### Client Integration Examples

**Web Client Flow:**

1. User visits `your-app.com`
2. NGINX checks authentication via Auth Master
3. Auth Master detects `X-Client-Type: Web` and delegates to OAuth2 Proxy
4. If not authenticated: 401 → redirect to OAuth2 sign-in
5. If authenticated: OAuth2 Proxy provides user info → Auth Master maps to internal user ID
6. NGINX forwards `X-Auth-User: internal-user-123` to backend services

**Telegram Bot Flow:**

```python
# Telegram bot sends request to your API
headers = {
    'X-Client-Type': 'Telegram',
    'X-Telegram-Id': '123456789',
    'X-Telegram-Key': 'verified_signature'
}

response = requests.get('https://your-app.com/api/user-data', headers=headers)
# Backend receives: X-Auth-User: internal-user-456
```

### Backend Service Benefits

**Before Auth Master:**

```python
# Each service handles multiple auth methods
def get_current_user(request):
    if 'Authorization' in request.headers:
        # Handle OAuth2 token
        user_id = verify_oauth2_token(request.headers['Authorization'])
    elif 'X-Telegram-Id' in request.headers:
        # Handle Telegram auth
        user_id = verify_telegram_user(request.headers['X-Telegram-Id'])
    else:
        raise Unauthorized("No authentication method")

    return user_id
```

**After Auth Master:**

```python
# All services use consistent approach
def get_current_user(request):
    user_id = request.headers.get('X-Auth-User')
    if not user_id:
        raise Unauthorized("User not authenticated")
    return user_id
```

### Result

-   **Backend services** focus on business logic, not authentication
-   **New auth methods** can be added without changing backend code
-   **User identity** is consistent across all authentication methods
-   **Infrastructure** is centralized and maintainable

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
