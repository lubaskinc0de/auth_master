up:
    docker compose up --build

down:
    docker compose down

clean:
    docker compose down -v

cookie-secret:
    echo "OAUTH2_PROXY_COOKIE_SECRET=$(openssl rand -base64 32)"
