# Storing UTF-8 Encoded Text with Strings

Many of the same operations available with `Vec<T>` are available with `String` as well, because `String` is actually implemented as a wrapper around a vector of bytes with some extra guarantees, restrictions, and capabilities.

```rs
// new empty string
let mut s1 = String::new();

// new prefilled string
let s2 = String::from("initial contents");

// add with push_str and push
let mut s3 = String::from("foo");
let s4 = "bar";
s3.push_str(s4);
s3.push_("!");
println!("s4 is {s2}");
```

Accessing a `String` by indexing is not possible in rust because of how strings are stored in memory.

```rs
let s1 = String::from("hello");
// error!
let h = s1[0];
```

```
$ cargo run
   Compiling collections v0.1.0 (file:///projects/collections)
error[E0277]: the type `String` cannot be indexed by `{integer}`
 --> src/main.rs:3:13
  |
3 |     let h = s1[0];
  |             ^^^^^ `String` cannot be indexed by `{integer}`
  |
  = help: the trait `Index<{integer}>` is not implemented for `String`
  = help: the following other types implement trait `Index<Idx>`:
            <String as Index<RangeFrom<usize>>>
            <String as Index<RangeFull>>
            <String as Index<RangeInclusive<usize>>>
            <String as Index<RangeTo<usize>>>
            <String as Index<RangeToInclusive<usize>>>
            <String as Index<std::ops::Range<usize>>>

For more information about this error, try `rustc --explain E0277`.
error: could not compile `collections` due to previous error
```

Slicing a `String` is possible like this:

```rs
let hello = "Здравствуйте";

let s = &hello[0..4]; // Зд since one letter takes 2 bytes.
```
Iterating over a `String` can be done by using the `chars` function.

```rs
for c in "Зд".chars() {
    println!("{c}");
}
```