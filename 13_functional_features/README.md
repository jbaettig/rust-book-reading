# Closures: Anonymous Functions that Capture Their Environment

A closure is an anonymous function that can be stored within a variable. They start with two pipe operators which can habe arguments between them. Afterwards the logic follows.

```rs
|| self.most_stocked()
```

Closures don’t usually require you to annotate the types of the parameters or the return value like fn functions do. But it is possible and sometimes even required.

```rs
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

```

Here is an example of a function and possible closures:

```rs
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

A closure body can do any of the following:

- move a captured value out of the closure
- mutate the captured value
- neither move nor mutate the value
- capture nothing from the environment

Depending on their behaviour, they implement different traits:

1. `FnOnce` applies to closures that can be called once. All closures implement at least this trait, because all closures can be called. A closure that moves captured values out of its body will only implement FnOnce and none of the other Fn traits, because it can only be called once.
2. `FnMut` applies to closures that don’t move captured values out of their body, but that might mutate the captured values. These closures can be called more than once.
3. `Fn` applies to closures that don’t move captured values out of their body and that don’t mutate captured values, as well as closures that capture nothing from their environment. These closures can be called more than once without mutating their environment, which is important in cases such as calling a closure multiple times concurrently.

# Processing a Series of Items with Iterators

All iterators implement a trait named `Iterator` that is defined in the standard library. The definition of the trait looks like this:

```rs
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}
```

## Methods that Consume the Iterator

Methods that call `next` are called _consuming adaptors_, because calling them uses up the iterator. Here, `sum` calls `next`:

```rs
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }
```

## Methods that Produce Other Iterators

_Iterator adaptors_ are methods defined on the `Iterator` trait that don’t consume the iterator. Instead, they produce different iterators by changing some aspect of the original iterator. The `map` method takes a closure to call on each item as the items are iterated through. We need to call `collect` to consume the iterator.

```rs
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
```

# Comparing Performance: Loops vs. Iterators

Iterators are a bit faster...
