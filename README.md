# rust-todolist
Modern command line to-do list app in Rust.

```
/// add to-do
cargo run -- -j test-journal.json add "buy milk"
cargo run -- -j test-journal.json add "water the plants"

/// remove to-do from list
cargo run -- -j test-journal.json done 2

/// show to-do list
cargo run -- -j test-journal.json list
```
