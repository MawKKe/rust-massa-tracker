# rust-massa-tracker

Tool for recording daily weights.

(My first rust program, yay!)

# Build

Clone the repo, enter the directory and run:

    $ cargo build

This should produce an executable binary called `massaa-rs` in the build
output directory. You may run it directly, or using `cargo run`.

# Usage

**NOTE**: The program stores data in an SQLite database. The database location
must be specified via `DATABASE_URL` environment variable, for example:

    $ export DATABASE_URL=$HOME/massaa.db

The database will be created and initialized if it did not already exist.
You may delete the database at any time, it will be recreated on the next
program invocation.

## Show help to get started:

    $ massaa-rs -h

## Add a record

    $ massaa-rs add 123.0

## Show all records:

    $ massaa-rs show

