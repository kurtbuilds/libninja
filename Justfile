set positional-arguments
set dotenv-load
set export

help:
    @just --list --unsorted

build:
    cargo build
alias b := build

run *args:
    cargo run -- "$@"
alias r := run

release:
    cargo build --release

example EXAMPLE:
    cargo run --example {{ EXAMPLE }}
alias e := example

install:
    cargo install --path libninja --force

check:
    cargo check
alias c := check

watch:
    cargo watch

fix:
    cargo clippy --fix

# Bump version. level=major,minor,patch
version level:
    git diff-index --exit-code HEAD > /dev/null || ! echo You have untracked changes. Commit your changes before bumping the version.
    cargo set-version --bump {{level}}
    cargo update # This bumps Cargo.lock
    VERSION=$(rg  "version = \"([0-9.]+)\"" -or '$1' Cargo.toml | head -n1) && \
        git commit -am "Bump version {{level}} to $VERSION" && \
        git tag v$VERSION && \
        git push origin v$VERSION
    git push

serve:
    cargo run --example petstore-server -F actix

publish:
    cargo publish

patch: test
    just version patch
    just publish

rust:
    cargo run -- gen --name PetStore --output-dir gen/rust --generator rust data/openapi-spec/petstore/petstore.yaml --github libninjacom/petstore-rs --version 0.1.0

generate:
    #!/bin/bash -euxo pipefail
    if [ -n "${LIBRARY:-}" ]; then
        export LIBRARY="--library $LIBRARY"
    else
        export LIBRARY=""
    fi
    REPO_DIR=$DIR/$(basename $REPO)
    # rm -rf "$REPO_DIR"/*
    cargo run -- gen --name $SERVICE --output-dir $REPO_DIR --generator $SOURCEGEN --github $REPO --version $VERSION $LIBRARY $SPEC

test *ARGS:
    cargo test -- "$ARGS"
alias t := test

# Test the library we just generated
test_lib:
    #!/bin/bash -euxo pipefail
    REPO_DIR=$DIR/$(basename $REPO)
    cd $REPO_DIR
    just bootstrap
    just check
    just test
alias tt := test_lib

clean-gen:
    #!/bin/bash -euxo pipefail
    if [ -z "$DIR" ]; then
        echo "DIR is empty"
        exit 1
    fi
    rm -rf $DIR/*