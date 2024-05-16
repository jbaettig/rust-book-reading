# How to Write Tests

Tests in rust are annotated with the `test` attribute. To change a function into a test function just add `#[test]` on top of it. The test functions can then be run with the `cargo test` command.

See `/adder/src/lib.rs`

## Checking Results with the assert! Macro

The `assert!` macro can be used to ensure that some condition evaluates to true. If it does not evaluate to true it will panic.

## Checking Results with the assert! Macro

With the `assert_eq!` and `assert_ne!` one can test for equality or inequality and prints the values if the comparison fails.

## Adding Custom Failure Messages

All macros describe accept a custom error message.

```rs
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }
```

## Checking for Panics with should_panic

To test if the code handles errors correctly one can write tests that should raise an error and annotate them with `should_panic`. It is also possible to specify the error text.

```rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

## Using Result<T, E> in Tests

With `Result<T, E>` it is possible to write tests that do not panic. The `#[should_panic]` annotation is not needed in this case.

```rs
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
```
