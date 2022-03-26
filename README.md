# Installation

We use `just` as the command runner. If you don't currently have it, install it:

    cargo install just


Once you have just, use it to view development commands:

    just  # Will print development commands

Use `gen` to generate a openapi-client. (Currently fixed to plaid, and not functionally complete, WIP).

    just gen


Use `client` to actually *use* that generated client to perform an API query.

    just client

Use `test` to run tests:

    just test
