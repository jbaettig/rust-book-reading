# Separation of Concerns for Binary Projects

Rust community has developed guidelines for splitting the separate concerns of a binary program when `main` starts getting large.

- Split your program into a _main.rs_ and a _lib.rs_ and move your program’s logic to _lib.rs_.
- As long as your command line parsing logic is small, it can remain in _main.rs_.
- When the command line parsing logic starts getting complicated, extract it from _main.rs_ and move it to _lib.rs_.

Responsibilities of _main.rs_ after this process:

- Calling the command line parsing logic with the argument values
- Setting up any other configuration
- Calling a `run` function in _lib.rs_
- Handling the error if `run` returns an error

# Setting an environment variable

```
IGNORE_CASE=1 cargo run -- to poem.txt
```

# Printing to stdout

`stdout` can be set using the `>` operator when calling `cargo run`.

```
cargo run > output.txt
```

# Printing to stderr

To print to `stderr` instead of `stdout` use the `eprintln!` macro.
