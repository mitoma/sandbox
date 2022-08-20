#!/bin/bash
set -eux

IMAGE_URI="asia-northeast1-docker.pkg.dev/mitoma-org/mitoma-org/app"
ARTIFACT_DIR=artifact
REPO_ROOT=$(git rev-parse --show-toplevel)
MITOMA_ORG_VERSION=$(sver calc "$REPO_ROOT/mitoma-org")
BACKEND_VERSION=$(sver calc "$REPO_ROOT/mitoma-org/backend")
FRONTEND_VERSION=$(sver calc "$REPO_ROOT/mitoma-org/frontend")

if [ ! -d "$ARTIFACT_DIR" ]; then
  mkdir "$ARTIFACT_DIR"
fi

if [ ! -d "$ARTIFACT_DIR/backend" ]; then
  gh run download -n "mitoma-org-backend-${BACKEND_VERSION}" --dir "$ARTIFACT_DIR/backend"
fi
chmod +x "$ARTIFACT_DIR/backend/backend"
  
if [ ! -d "$ARTIFACT_DIR/frontend" ]; then
  gh run download -n "mitoma-org-frontend-${FRONTEND_VERSION}" --dir "$ARTIFACT_DIR/frontend"
fi

docker build --build-arg ARTIFACT_DIR="$ARTIFACT_DIR" -t "$IMAGE_URI:$MITOMA_ORG_VERSION" .
docker push "$IMAGE_URI:$MITOMA_ORG_VERSION"
