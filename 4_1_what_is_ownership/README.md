# What Is Ownership?

Ownership is a set of rules that govern how a Rust program manages memory. As a result rust does not need a garbage collector.

In Rsut a variable _owns_ some memory. That means tat in Rust the memory is automatically returned (freed) once the variable that owns it goes out of scope.

## Stack and Heap

When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function’s local variables get pushed onto the stack. When the function is over, those values get popped off the stack.

## Ownerhsip Rules

- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

## String Type

The string type has a variable length an must be stored on the heap. The `String` type manages data allocation on the heap an can store an amount of text that is unknown at compile time.

```rs
// create a sting from a string literal
// -> 'String::from' requests the memory it needs
let s = String::from("hello");
```

In Rust the memory is automatically returned (freed) once the variable that owns it goes out of scope. This is automatically done by a call of the `drop` function.

## Variables and Data Interacting with Move

What does the following example do?

-> "Bind the value 5 to x; then make a copy of the value in x and bind it to y. We now have two variables, x and y, and both equal 5. The two 5s are on the stack"

```rs
let x = 5;
let y = x;
```

The version with `String` is different however.

```rs
let s1 = String::from("hello");
let s2 = s1;
```

A string has three parts on the stack: A pointer to the memory that holds the contents of the string, a length (bytes), and a capacity (bytes).

In the example code above only the pointer, length and capacity are copied. The actual values on the heap do not get copied.

-> When we assign `let s2 = s1` in the code above the variable `s1` is no longer considered valid in rust! We say taht `s1` was _moved_ into `s2`.

## Variables and Data Interacting with Clone

With the `clone()` function one can create a deep copy of a complex data type. This can however be an expensive function call.

```rs
let s1 = String::from("hello");
let s2 = s1.clone();
// s1 was not moved and is still valid
// s2 is a deep copy
println!("s1 = {}, s2 = {}", s1, s2);
```

## Stack-Only Data: Copy

-> Types with known size at compile time always make a deep copy and therefore are not moved like we can see in the example with integers. They implement a trait that is called `Copy`.

## Ownership of Functions

Passing a variable to a function will move or copy, just as assignment does.

```rs
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.
```

## Return Values and Scope

Returning a value can also transfer ownership.

```rs
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.
```
