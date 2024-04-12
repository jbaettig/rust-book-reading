# The Slice Type

_Slices_ let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it does not have ownership.

## String Slice

A _string slice_ is a reference to a part of a string starting and ending at a valid UTF-8 character.

```rs
let s = String::from("hello world");
let hello = &s[0..5];
let world = &s[6..11];
```

A slice is stored on the stack and has a pointer to the first relevant data on the heap. It also has a length propert.

Have a look at the `first_word()` function within `/src/main.rs`.

## String Literals as Slices

```rs
let s = "Hello, world!";
```

The type of s here is &str: it’s a slice pointing to that specific point of the binary. This is also why string literals are immutable; &str is an immutable reference!

## Other Slices

Arrays can also be sliced. Consider the following:

```rs
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3]; // has type &[i32]
assert_eq!(slice, &[2, 3]);
```
