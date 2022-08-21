#!/bin/bash
set -eux

IMAGE_URI="asia-northeast1-docker.pkg.dev/mitoma-org/mitoma-org/app"
REPO_ROOT=$(git rev-parse --show-toplevel)
MITOMA_ORG_VERSION=$(sver calc "$REPO_ROOT/mitoma-org")

IMAGE_TAG="$IMAGE_URI:$MITOMA_ORG_VERSION"
ALREADY_EXISTS=$(docker manifest inspect $IMAGE_TAG > /dev/null ; echo $?)
if [ 1 -eq $ALREADY_EXISTS ] ;
then
  echo "image is not found"
  exit 0
fi

gcloud run deploy mitoma-org \
  --image=$IMAGE_TAG \
  --region=asia-northeast1 \
  --project=mitoma-org
gcloud run services update-traffic mitoma-org \
  --region=asia-northeast1 \
  --project=mitoma-org \
  --to-latest
