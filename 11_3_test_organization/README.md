# Test Organization

_Unit tests_ are small and focused tests that test one module in isolation. _Integration tests_ are entirely external to your library and use your code in the same way any other external code would.

## Unit Tests

The convention for unit tests is to create a module named tests in each file to contain the test functions and to annotate the module with `cfg(test)`.

### Testing Private Functions

Rust's privacy rules do allow the testing of private functions.

```rs
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
```

## Integration Tests

In Rust, integration tests are entirely external to your library. To create integration tests, you first need a `tests` directory.

### The _tests_ Directory

The `tests` directory can be a top level directory next to `src`. The functions within the `test` directory do not need to be annotated with `#[cfg(test)]` since cargo treats them specially.

```
adder
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_test.rs
```

## Submodules in Integration Tests

Cargo treats every file in the `tests` directory as a test file. To create submodules they need to be in their own subdirectories.

```
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    ├── common
    │   └── mod.rs
    └── integration_test.rs
```

The module in `mod.rs` can be used as follows:

```rs
use adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
```
