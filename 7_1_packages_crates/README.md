# Packages and Crates

A `crate` is the smallest amount of code that the Rust compiler considers at a time.A crate can come in one of two forms: a binary crate or a library crate. When Rustaceans say "crate" they mean library crate.

- Binary crate: Programs you can compile to an executable. They have a `main` function.
- Library crates: This kind of crate does not compile to an executable and does not have a `main` function. They contain functionality intended to be shared with multiple projects.

A `package` is a bundle of one or more crates that provides a set of functionality. It contains a _Cargo.toml_ file that describes how to build those crates. It can create many binary crates but only one library crate.
