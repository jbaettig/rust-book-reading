# Unrecoverable Errors with panic!

Rust has the `panic!` macro for errors that it cannot recover from. With `RUST_BACKTRACE=1 cargo run` rust will include an exact description of what happend.

```rs
fn main() {
    panic!("crash and burn");
}
```