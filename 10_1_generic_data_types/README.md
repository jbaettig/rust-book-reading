# Generic Data Types

## In Function Definitions

Functions can contain generics within their signature. The generic type parameter can have any name but mostly one uses `T`. Naming convention is short and UpperCamelCase.

```rs
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
```


## In Struct Definitions

Fields of a `struct` can also be defined as generic.


```rs
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
```

## In Enum Definitions

An example of an `enum` with generic type(s) would be `Option` or `Result`.

```rs
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

## In Method Definitions

Methods of a `struct` can use generics, too.

```rs
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```

## Performance of Code Using Generics

There is no additional runtime cost when using generics. This is done by rust with monomorphization. _Monomorphization_ is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.