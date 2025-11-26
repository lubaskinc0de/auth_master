# Auth Master

> [!CAUTION]
> This project is not ready for production, it is still in development.

**Universal self-hosted blazingly fast authentication service**

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

<img src="https://imgfy.ru/ib/sQbCMYkbOj3Hs4z_1764066718.png" alt="image.png"/>

### Configuration Example

**NGINX configuration:**

```conf
events {
    worker_connections 1024;
}

http {
    upstream auth_master {
        server auth_master:3000;
    }

    upstream oauth2_proxy {
        server oauth2-proxy:8000;
    }

    upstream api {
        server example_api:8080;
    }

    server {
        listen 80;
        server_name localhost;

        location /oauth2/ {
            proxy_pass http://oauth2_proxy;
        }

        location /health {
            proxy_pass http://auth_master;
        }

        location / {
            auth_request /auth;
            auth_request_set $user $upstream_http_x_auth_user;
            auth_request_set $redirect $upstream_http_www_authenticate;

            proxy_pass http://api;
            proxy_set_header X-Auth-User $user;
            error_page 401 = @handle_auth_redirect;
        }

        location = /auth {
            internal;
            proxy_pass http://auth_master/;
        }

        location @handle_auth_redirect {
            if ($redirect) {
                return 302 $redirect;
            }

            return 401 '{"error": "Authentication required", "error_code": "UNAUTHORIZED""}';
        }
    }
}
```

### Client Integration Examples

**Web Client Flow:**

1. User visits `https://yourapp.com`
2. NGINX checks authentication via Auth Master
3. Auth Master detects `X-Client-Type: Web` and delegates to OAuth2 Proxy
4. If not authenticated: 401 → redirect to OAuth2 sign-in
5. If authenticated: OAuth2 Proxy provides user info → Auth Master maps to internal user ID
6. NGINX forwards `X-Auth-User: internal-user-123` to your backend services

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

# Contributing

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
```

### 2. Build and Run

```bash
just up
```

The service will be available at `http://localhost`

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
