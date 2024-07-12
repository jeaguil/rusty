# rusty

# important things to remember

**profiles** -
A profile is a set of configuration options that can be used to customize the way Rust code is compiled.

Cargo provides two built-in profiles: dev and release.
The dev profile is used every time you run `cargo build`, `cargo run` or `cargo test`. It's aimed at local development, therefore it sacrifices runtime performance in favor of faster compilation times and a better debugging experience.
The release profile, instead, is optimized for runtime performance but incurs longer compilation times. You need to explicitly request via the --release flag—e.g. `cargo build --release` or `cargo run --release`.
