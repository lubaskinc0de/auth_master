up:
    docker compose up --build

infra:
    docker compose up nginx oauth2-proxy redis postgres

dev:
    set -a && source "./.env" && set +a && cargo run -p migrations
    set -a && source "./.env" && set +a && cargo run -p app_core

down:
    docker compose down

clean:
    docker compose down -v

cookie-secret:
    echo "OAUTH2_PROXY_COOKIE_SECRET=$(openssl rand -base64 32)"
