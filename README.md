# rust_rest_test

Tests written in Rust using the Rust HTTP client ureq to call Speculos Ledger Emulator.

Speculos shoulld be running on http://127.0.0.1:5001/  (change the url in the tests otherwise)

Testing supported Tezos operations.

To run tests:

```
cargo +nightly test tests
```
