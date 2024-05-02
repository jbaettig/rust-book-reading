# Traits: Defining Shared Behavior

A _trait_ defines functionality a particular type has and can share with other types. Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose. They are similar to _interfaces_ in other languages.

## Defining a Trait

A trait can be declared using the `trait` keyword. Within the trait it's methods are defined.

```rs
pub trait Summary {
    fn summarize(&self) -> String;
}
```

## Implementing a Trait on a Type

A trait can be implemented for a type by using the `impl` keyword. All methods of the trait must be implemented.

```rs
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

One restriction to note is that we can implement a trait on a type only if at least one of the trait or the type is local to our crate.

## Default Implementations

Methods of a trait can have a default implementation. Default implementations can call other methods in the same trait, even if those other methods don’t have a default implementation. In this way, a trait can provide a lot of useful functionality and only require implementors to specify a small part of it.

```rs
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```

## Traits as Parameters

In the following example `item` can be any type that implements the `Summary` trait!

```rs
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

### Trait Bound Syntax

The `impl Trait` syntax works for straightforward cases but is actually syntax sugar for a longer form known as a _trait bound_.

```rs
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

Here is an example where the bound syntax is better:

```rs
// impl Trait
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
// bound Syntax
pub fn notify<T: Summary>(item1: &T, item2: &T) {
```

### Specifying Multiple Trait Bounds with the + Syntax

With mulitple traits:

```rs
pub fn notify(item: &(impl Summary + Display)) {
// bound Syntax
pub fn notify<T: Summary + Display>(item: &T) {
```

### Clearer Trait Bounds with where Clauses

For better readability Rust provides a `where` clause.

```rs
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
```

## Returning Types that Implement Traits

We can also use the `impl Trait` syntax in the return position. The ability to specify a return type only by the trait it implements is especially useful in the context of closures and iterators.

```rs
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
```

However, you can only use `impl Trait` if you’re returning a single type. The following would not work:

```rs
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}
```

## Using Trait Bounds to Conditionally Implement Methods

By using a trait bound with an impl block that uses generic type parameters, we can implement methods conditionally for types that implement the specified traits.

```rs
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```