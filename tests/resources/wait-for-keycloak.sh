#!/bin/bash

# Start Keycloak in the background
/opt/keycloak/bin/kc.sh "$@" &

# This script runs inside the keycloak container, so always use localhost
echo "Waiting for Keycloak to be ready at http://localhost:8080 ..."
until curl -s -o /dev/null http://localhost:8080/realms/master 2>&1; do
  echo "Keycloak not ready yet, retrying in 5s..."
  sleep 5
done
echo "Keycloak is ready!"

# Wait a bit for realm import to complete
sleep 5

/opt/keycloak/scripts/create-sirix-users.sh

wait
