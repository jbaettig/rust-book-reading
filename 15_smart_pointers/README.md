# Smart Pointers

Smart pointers, on the other hand, are data structures that act like a pointer but also have additional metadata and capabilities.

## Using Box<T> to Point to Data on the Heap

Boxes allow you to store data on the heap rather than the stack. What remains on the stack is the pointer to the heap data. A `Box<T>` is mostly used in these situations:

- When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
- When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
- When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type

### Using a Box<T> to Store Data on the Heap

Define the variable `b` to have the value of a `Box` that points to the value `5`, which is allocated on the heap:

```rs
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}
```

### Enabling Recursive Types with Boxes

A value of _recursive type_ can have another value of the same type as part of itself. Recursive types pose an issue because at compile time Rust needs to know how much space a type takes up. Because boxes have a known size, we can enable recursive types by inserting a box in the recursive type definition.

#### More Information About the Cons List

The `cons` type is the List Type of the Lisp language. Here is the pseudocode:

```
(1, (2, (3, Nil)))
```

The following code contains an enum definition for a cons list.

```rs
// will not compile due to unknown size
enum List {
    Cons(i32, List),
    Nil,
}
```

#### Using Box<T> to Get a Recursive Type with a Known Size

Because a `Box<T>` is a pointer, Rust always knows how much space a `Box<T>` needs: a pointer’s size doesn’t change based on the amount of data it’s pointing to.

```rs
// this will compile
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
```

# Treating Smart Pointers Like Regular References with the Deref Trait

Implementing the Deref trait allows you to customize the behavior of the dereference operator `*`.

## Following the Pointer to the Value

```rs
fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

## Using Box<T> Like a Reference

```rs
fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

## Defining Our Own Smart Pointer

See code in main.rs

## Implicit Deref Coercions with Functions and Methods

_Deref coercion_ converts a reference to a type that implements the `Deref` trait into a reference to another type. For example, deref coercion can convert `&String` to `&str` because `String` implements the `Deref` trait such that it returns `&str`.

Deref coercion makes the following possible:

```rs
fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    // this will work thanks to deref coercion
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
```

## How Deref Coercion Interacts with Mutability

Similar to how you use the Deref trait to override the _ operator on immutable references, you can use the DerefMut trait to override the _ operator on mutable references.

# Running Code on Cleanup with the Drop Trait

The second trait important to the smart pointer pattern is `Drop`, which lets you customize what happens when a value is about to go out of scope. You can provide an implementation for the `Drop` trait on any type, and that code can be used to release resources like files or network connections. The code in the `Drop` trait will always be run when the variable goes out of scope.

```rs
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}
```

## Dropping a Value Early with std::mem::drop

The `std::mem::drop` function is different from the `drop` method in the Drop trait. We call it by passing as an argument the value we want to force drop.

```rs
fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}
```

# Rc<T>, the Reference Counted Smart Pointer

In the majority of cases, ownership is clear: you know exactly which variable owns a given value. However, there are cases when a single value might have multiple owners. For example, in graph data structures, multiple edges might point to the same node, and that node is conceptually owned by all of the edges that point to it. A node shouldn’t be cleaned up unless it doesn’t have any edges pointing to it and so has no owners.

You have to enable multiple ownership explicitly by using the Rust type `Rc<T>`, which is an abbreviation for reference counting.

## Using Rc<T> to Share Data

```rs
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    // reference count is 1
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // reference count is 2
    let b = Cons(3, Rc::clone(&a));
    // reference count is 3
    let c = Cons(4, Rc::clone(&a));
}
```

# RefCell<T> and the Interior Mutability Pattern

_Interior mutability_ is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data; normally, this action is disallowed by the borrowing rules.

## Enforcing Borrowing Rules at Runtime with RefCell<T>

With `RefCell<T>` the borrowing rules are enforced during runtime.

- Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
- Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at compile time; RefCell<T> allows immutable or mutable borrows checked at runtime.
- Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.

## Interior Mutability: A Mutable Borrow to an Immutable Value

There are situations in which it would be useful for a value to mutate itself in its methods but appear immutable to other code.

### A Use Case for Interior Mutability: Mock Objects

Rust doesn’t have objects in the same sense as other languages have objects, and Rust doesn’t have mock object functionality built into the standard library as some other languages do. However, you can definitely create a struct that will serve the same purposes as a mock object.

See Code in lib.rs

## Keeping Track of Borrows at Runtime with RefCell<T>

When creating immutable and mutable references, we use the `&` and `&mut` syntax, respectively. With `RefCell<T>`, we use the `borrow` and `borrow_mut` methods, which are part of the safe API that belongs to `RefCell<T>`. The `RefCell<T>` keeps track of how many `Ref<T>` and `RefMut<T>` smart pointers are currently active. Just like the compile-time borrowing rules, `RefCell<T>` lets us have many immutable borrows or one mutable borrow at any point in time.

## Having Multiple Owners of Mutable Data by Combining Rc<T> and RefCell<T>

A common way to use `RefCell<T>` is in combination with `Rc<T>`. Recall that `Rc<T>` lets you have multiple owners of some data, but it only gives immutable access to that data. If you have an `Rc<T>` that holds a `RefCell<T>`, you can get a value that can have multiple owners and that you can mutate!

```rs
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
```

# Summary

Interesting chart: https://github.com/usagi/rust-memory-container-cs/blob/master/3840x2160/rust-memory-container-cs-3840x2160-dark-back.png
