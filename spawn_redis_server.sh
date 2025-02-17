exec cargo run \
    --quiet \
    --release \
    --manifest-path $(dirname $0)/Cargo.toml \
    -- "$@"

# A Wrapper around cargo run, useful to run tests