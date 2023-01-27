#!/bin/bash
# Create a new repo
# ./create.sh
set -euxo pipefail

# Declare variables
REPO=$REPO
GH_TOKEN=$GH_TOKEN
DIR=$DIR
VERSION=$VERSION

# Additional variables
WORKING_DIR=$(readlink -f $DIR)
REPO_DIR=$WORKING_DIR/$(basename $REPO)

cd $REPO_DIR
git add .
git commit -m "Automatic update of client code." --allow-empty
git tag -a v$VERSION -m "Update version to $VERSION"
git push --set-upstream origin master --tags