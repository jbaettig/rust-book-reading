# Method Syntax

Methods can be defined within a struct, an enum or a trait object. Like functions they can have parameters and a return value.

## Defining Methods

Methods need to be defined within an `impl`(implementation) block. The first parameter of the area function is always `self`. A struct can have multiple impl blocks.

```rs
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // &self is the same as self: &Self
    // Self is an alias for type wihtin the impl block
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

It is also possible to have a method that has the same name as a field within the struct. Often this is used for _getters_ which rust does **not** implement automatically for structs.

```rs
impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}
```

## Associated Functions

All functions defined within an impl block are called _associated functions_ because they’re associated with the type named after the impl. It is also possible to have associated functions that do not need a `self` like the one below.

```rs
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// call it with
let sq = Rectangle::square(3);
```
