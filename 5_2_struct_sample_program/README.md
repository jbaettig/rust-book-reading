# An Example Program Using Structs

This example demonstrated how structs can be used to write better programs since they structure data.

-> See `/src/main.rs`

## Adding Useful Functionality with Derived Traits

By adding `#[derive(Debug)]` on the `Rectangle` trait one can print the rectangle.

```rs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
}
```

-> The `{:?}` specifier will allow us to print the rectangle since it derives from the Debug trait. With `{:#?}` the rectangles fields would be printed on new lines.

Another way to print the rectangle would be to use the `dbg!` macro. Since this macro takes ownership we pass the rectangle as a reference.

```rs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}
```
