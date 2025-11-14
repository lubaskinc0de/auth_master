up:
    docker compose up --build

infra:
    docker compose up nginx oauth2-proxy redis postgres

down:
    docker compose down

clean:
    docker compose down -v

cookie-secret:
    echo "OAUTH2_PROXY_COOKIE_SECRET=$(openssl rand -base64 32)"
