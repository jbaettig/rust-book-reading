# Storing Keys with Associated Values in Hash Maps

The type `HashMap<K, V>` stores a mapping of keys of type K to values of type V using a hashing function, which determines how it places these keys and values into memory. Hash maps are useful when you want to look up data not by using an index, as you can with vectors, but by using a key that can be of any type. Just like vectors, hash maps store their data on the heap. A new Map can be created as follows.

```rs
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

For types that implement the Copy trait, like i32, the values are copied into the hash map. For owned values like String, the values will be moved and the hash map will be the owner of those values!

## Overwriting a Value

```rs
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);
```

## Adding a Key and Value Only If a Key Isn’t Present

```rs
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);
```

## Updating a Value Based on the Old Value

```rs
let text = "hello world wonderful world";
let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}
```