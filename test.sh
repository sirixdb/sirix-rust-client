#!/bin/bash
set -e

COMPOSE_FILE="./tests/resources/docker-compose.yml"

# Use "docker compose" (V2) if available, fall back to "docker-compose" (V1)
if docker compose version >/dev/null 2>&1; then
  COMPOSE="docker compose"
else
  COMPOSE="docker-compose"
fi

cleanup() {
  echo "Stopping and removing containers..."
  $COMPOSE -f "$COMPOSE_FILE" down -v --remove-orphans 2>/dev/null || true
}

trap cleanup EXIT

echo "Starting Docker environment..."
$COMPOSE -f "$COMPOSE_FILE" up -d --build

echo "Waiting for Keycloak to become healthy..."
timeout=300
elapsed=0
while [ $elapsed -lt $timeout ]; do
  if $COMPOSE -f "$COMPOSE_FILE" ps keycloak | grep -q "healthy"; then
    echo "Keycloak is healthy."
    break
  fi
  echo "Keycloak not ready yet (${elapsed}s elapsed), waiting..."
  sleep 5
  elapsed=$((elapsed + 5))
done

if [ $elapsed -ge $timeout ]; then
  echo "ERROR: Keycloak did not become healthy within ${timeout}s"
  $COMPOSE -f "$COMPOSE_FILE" logs keycloak
  exit 1
fi

echo "Starting SirixDB server..."
$COMPOSE -f "$COMPOSE_FILE" up -d server

echo "Waiting for SirixDB to be ready..."
timeout=120
elapsed=0
while [ $elapsed -lt $timeout ]; do
  # Use -s -o /dev/null without -f: SirixDB may return 401 at / but that still means it's up
  if curl -s -o /dev/null http://localhost:9443 2>&1; then
    echo "SirixDB is ready!"
    break
  fi
  echo "SirixDB not ready yet (${elapsed}s elapsed), waiting..."
  sleep 5
  elapsed=$((elapsed + 5))
done

if [ $elapsed -ge $timeout ]; then
  echo "ERROR: SirixDB did not become ready within ${timeout}s"
  $COMPOSE -f "$COMPOSE_FILE" logs server
  exit 1
fi

echo "Running tests..."
cargo test --all-features --verbose
