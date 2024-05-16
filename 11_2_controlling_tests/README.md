# Controlling How Tests Are Run

By default `cargo test` creates a binary that contains all tests and runs them in parallel. This can be changed using command line arguments.

See `cargo test -- --help`

## Running Tests in Parallel or Consecutively

To run tests consecutively use `$ cargo test -- --test-threads=1`.

## Showing Function Output

Tests that pass do not log anything to the standard output by default. This behaviour can be change using `$ cargo test -- --show-output`.

## Running a Subset of Tests by Name

### Running Single Tests

By passing the name of a specific test to `cargo test` only this test will be run.

### Filtering to Run Multiple Tests

By providing just a part of a test name all the tests that contain this part will be run. In this example all tests that contain `add` will be run:

`cargo test add`

### Ignoring Some Tests Unless Specifically Requested

If a test is taking very long it can be excluded by default. To do so the function can be annotated with `#[ignore]`.

```rs
#[test]
#[ignore]
fn expensive_test() {
   // code that takes an hour to run
}
```

Use `cargo test -- --ignored` to also run ignored tests.
