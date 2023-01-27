#!/bin/bash
# Create a new repo
# ./create.sh
set -euxo pipefail

# Declare variables
REPO=$REPO
GH_TOKEN=$GH_TOKEN
HOMEPAGE=$HOMEPAGE
DIR=$DIR
SERVICE=$SERVICE
TAGS=$(echo $TAGS | tr "," "\n")

# Additional variables
WORKING_DIR=$(readlink -f $DIR)
REPO_DIR=$WORKING_DIR/$(basename $REPO)
DESCRIPTION="$SERVICE client, generated from the OpenAPI spec"

if ls "$REPO_DIR"; then 
  echo "$REPO_DIR already exists. Exiting"
  exit 1
fi

if ! $(gh repo clone $REPO $REPO_DIR); then
  cd "$WORKING_DIR"
  gh repo create --public --clone $REPO
  cd "$REPO_DIR"
  if [ -n "$HOMEPAGE" ]; then
    gh repo edit --homepage $HOMEPAGE
  fi
  for tag in $TAGS; do
    gh repo edit --add-topic $tag
  done
  gh repo edit --description "$DESCRIPTION"
fi
