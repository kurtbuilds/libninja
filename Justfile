set positional-arguments
set dotenv-load := true

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
    just clean release
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

clean MODE='debug':
    checkexec ${CARGO_TARGET_DIR:-target}/{{MODE}}/libninja $(fd . -H core/template) -- cargo clean --package libninja

rust: clean
    cargo run -- gen --name PetStore --output-dir gen/rust --generator rust data/openapi-spec/petstore/petstore.yaml --github libninjacom/petstore-rs --version 0.1.0

python: clean
    cargo run -- gen --name PetStore --output-dir gen/python --generator python --version 0.1.0 --github libninjacom/petstore-py spec/petstore.yaml

python-example:
    #!/bin/bash -euxo pipefail
    cd gen/python
    eval "$(pdm --pep582)"
    python3 -m examples.list_pets

typescript: clean
    cargo run -- gen --name PetStore --output-dir gen/typescript --generator typescript data/openapi-spec/petstore/petstore.yaml

java:
    just gen/java/build
    just gen/java/run

go:
    rm -rf gen/petstore-go
    checkexec ${CARGO_TARGET_DIR:-target}/debug/ocg $(fd . ocg/template) -- cargo clean --package ocg
    cargo run -- gen --name PetStore --output-dir gen/petstore-go --generator go spec/petstore.yaml --github libninjacom/petstore-go --version 0.1.0

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
    checkexec commercial -- just dummy_commercial
    cargo test
alias t := test

integration *ARGS:
    cd libninja && cargo test -F integration -- "$@"
alias int := integration

# Test the library we just generated
test_lib:
    #!/bin/bash -euxo pipefail
    REPO_DIR=$DIR/$(basename $REPO)
    cd $REPO_DIR
    just bootstrap
    just check
    just test
alias tt := test-lib

clean-gen:
    #!/bin/bash -euxo pipefail
    if [ -z "$DIR" ]; then
        echo "DIR is empty"
        exit 1
    fi
    rm -rf $DIR/*

delete *ARG:
    gh repo delete $REPO {{ARG}}

commercial:
    rm -rf commercial
    git clone https://github.com/kurtbuilds/libninja-commercial commercial

# Create a dummy commercial repo that lets the workspace work
# without the commericial code
dummy_commercial:
    cargo new --lib commercial --name libninja_commercial
