# References and Borrowing

If for example we want to use a `String` variable used in a function call after that function has been called we can pass it as a _refernece_ to the function. A reference points to data stored at a specific address on the heap.

The action creating a reference is called _borrowing_.As in real life, if a person owns something, you can borrow it from them. When you’re done, you have to give it back. You don’t own it.

Something that is borrowed cannot be modified!

```rs
fn main() {
    let s = String::from("hello");
    change(&s);
}

fn change(some_string: &String) {
    // this line will fail at compile time
    some_string.push_str(", world");
}
```

## Mutable References

If one wants to change a reference it must be a _mutable reference_ as shown below.

```rs
let mut s = String::from("hello");
change_something(&mut s);
```

One restriction is that there only can be exactly one mutable reference to a value in a given scope. The code below would fail.

```rs
let mut s = String::from("hello");
let r1 = &mut s;
let r2 = &mut s;
```

But this code would work:

```rs
let mut s = String::from("hello");
{
    let r1 = &mut s;
} // r1 goes out of scope here, so we can make a new one
let r2 = &mut s;
```

-> This restriction prevents data races!

A data race is similar to a race condition and happens when these three behaviors occur:

- Two or more pointers access the same data at the same time.
- At least one of the pointers is being used to write to the data.
- There’s no mechanism being used to synchronize access to the data.

It is also not possible to have a mutable reference to a variable if there is already a reference to that variable!

```rs
let mut s = String::from("hello");
let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // BIG PROBLEM
println!("{}, {}, and {}", r1, r2, r3);
```

## Dangling References

A _dangling pointer_ is a pointer to a location in memory that has already been given to someone else. In Rust, by contrast, the compiler guarantees that references will never be dangling references!

The compiler would throw an error.

```rs
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
```
