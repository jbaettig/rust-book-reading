# Validating References with Lifetimes

Lifetimes ensure that references are valid as long as we need them to be.

## Preventing Dangling References with Lifetimes

Lifetimes are mainly used to prevent _dangling references_. They are references that point to the wrong data. The following code contains a dangling reference since `x` is out of scope when `r` is printed.

```rs
fn main() {
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}
```

## The Borrow Checker

The Rust borrow checker ensures safe memory usage by enforcing rules on how references are borrowed and used. It compares the lifetimes of references as displayed in the code below. Since the lifetime of `r` `'a` is bigger than the lifetime `'b` of `r` the compiler will not compile this code.

```rs
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+
```

## Lifetime Annotation Syntax

Lifetime annotations don’t change how long any of the references live. Rather, they describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes. Lifetime annotations have a slightly unusual syntax: the names of lifetime parameters must start with an apostrophe (') and are usually all lowercase and very short, like generic types.

```rs
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

## Lifetime Annotations in Function Signatures

Within the `<>` of the `longest` function the lifetime `'a` is defined. `x` and `y` and the return `&str`are set to have the same lifetime. They are now bonded to each other and the return value will live as long as the inputs.

```rs
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

For example, the following code would not work since the lifetime of `string2` is smaller than the one of `result`. `result` will not be valid in the outer scope.

```rs
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}
```

## Lifetime Annotations in Struct Definitions

A `struct` can also contain reference fields. But they need to be anotated with lifetimes!

```rs
struct ImportantExcerpt<'a> {
    part: &'a str, // This is a string slice reference
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
```

## Lifetime Elision

For some specific patterns rust manages the lifetimes itself. Those patterns are called the _lifetime elision rules_. Lifetimes on function or method parameters are called _input lifetimes_, and lifetimes on return values are called _output lifetimes_. There are three rules for lifetime elision:

1. The first rule is that the compiler assigns a lifetime parameter to each parameter that’s a reference.
2. The second rule is that, if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
3. The third rule is that, if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters.

## Lifetime Annotations in Method Definitions

Mostly not cause of the elision rules.

## The Static Lifetime

Static lifetimes live as long as the program does. All string literals automatically have the `'static` lifetime. The `'static` lifetime should not be used to fix lifetime erros...

# Generic Type Parameters, Trait Bounds, and Lifetimes Together

Since lifetimes are a type of generic, the declarations of the lifetime parameter 'a and the generic type parameter T go in the same list inside the angle brackets after the function name.

```rs
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
