# Customizing Builds with Release Profiles

Cargo has default settings for the `dev` and `release` profiles. One can explicitly add any `[profile.*]` sections in the project’s Cargo.toml file.

# Publishing a Crate to Crates.io

This chapter was basically a tutorial on how to get it done.

## Making Useful Documentation Comments

HTML Documentation can be created with `cargo doc`.

````rs
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
````

Sections can be created using the # within the documentation block. Here are some common ones:

- **Panics**: The scenarios in which the function being documented could panic. Callers of the function who don’t want their programs to panic should make sure they don’t call the function in these situations.
- **Errors**: If the function returns a Result, describing the kinds of errors that might occur and what conditions might cause those errors to be returned can be helpful to callers so they can write code to handle the different kinds of errors in different ways.
- **Safety**: If the function is unsafe to call (we discuss unsafety in Chapter 19), there should be a section explaining why the function is unsafe and covering the invariants that the function expects callers to uphold.

## Commenting Contained Items

With `//!` one can document a crate. Here an example:

```rs
//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
// --snip--
```

## Exporting a Convenient Public API with pub use

Re-exporting takes a public item in one location and makes it public in another location, as if it were defined in the other location instead. This can be done by using `pub use`. A publish to crates.io is permanent and the code cannot be changed! But there are versions and you can deprecate a version easily.

```rs
//! # Art
//!
//! A library for modeling artistic concepts.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    // --snip--
}

pub mod utils {
    // --snip--
}
```

# Installing Binaries with cargo install

The `cargo install` command allows you to install and use binary crates locally.

```zsh
$ cargo install ripgrep
    Updating crates.io index
  Downloaded ripgrep v13.0.0
  Downloaded 1 crate (243.3 KB) in 0.88s
  Installing ripgrep v13.0.0
--snip--
   Compiling ripgrep v13.0.0
    Finished release [optimized + debuginfo] target(s) in 3m 10s
  Installing ~/.cargo/bin/rg
   Installed package `ripgrep v13.0.0` (executable `rg`)
```
