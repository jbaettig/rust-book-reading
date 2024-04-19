# Bringing Paths into Scope with the use Keyword

The `use` keyword can be used to bring paths into scope so one does not have to write the entiere path all the time.

```rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// bring path into scope
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

## Creating Idiomatic use Paths

It is possible to directly bring functions into scope by specifying the whole path to that function. This however only makes sense for structs or enums.

```rs
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```

## Providing New Names with the as Keyword

With the `as` keyword a new name can be defined as an alsias for the type.

```rs
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

## Re-exporting Names with pub use

When one adds the `pub` keyword infront of a `use` the code that calls ones code also has access to that imported type. This is called _re-exporting_.

## Using External Packages

External packages that have been imported using _Cargo.toml_ need to be called by their name within a `use` statement.

```rs
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}
```

## The Glob Operator

If one wishes to bring all public items defined in a path into scope once can use the `*` glob operator.

```rs
use std::collections::*;
```
