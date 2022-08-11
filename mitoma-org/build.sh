#!/bin/bash
set -eux

RUN_ID="$1"
ARTIFACT_DIR=artifact
rm -rf "$ARTIFACT_DIR"
mkdir "$ARTIFACT_DIR"
REPO_ROOT=$(git rev-parse --show-toplevel)
MITOMA_ORG_VERSION=$(sver calc "$REPO_ROOT/mitoma-org")
BACKEND_VERSION=$(sver calc "$REPO_ROOT/mitoma-org/backend")
FRONTEND_VERSION=$(sver calc "$REPO_ROOT/mitoma-org/frontend")

gh run download -n "mitoma-org-backend-${BACKEND_VERSION}" --dir "$ARTIFACT_DIR/backend" "$RUN_ID"
chmod +x "$ARTIFACT_DIR/backend/backend"

gh run download -n "mitoma-org-frontend-${FRONTEND_VERSION}" --dir "$ARTIFACT_DIR/frontend" "$RUN_ID"

docker build --build-arg ARTIFACT_DIR="$ARTIFACT_DIR" -t "mitoma-org:$MITOMA_ORG_VERSION" .