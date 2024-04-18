# Concise Control Flow with if let

`if let` can be used to minimize boilerplate code. One can think of if let as syntax sugar for a match that runs code when the value matches one pattern and then ignores all other values. `if let` is non-exhaustive.

```rs
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("The maximum is configured to be {}", max),
    _ => (),
}

// =>

let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {}", max);
} // an else would be possible here
```
