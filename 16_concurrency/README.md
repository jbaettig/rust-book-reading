# Using Threads to Run Code Simultaneously

Execution order in threads cannot be garanteed which can lead to the following problems:

- **Race conditions**, where threads are accessing data or resources in an inconsistent order
- **Deadlocks**, where two threads are waiting for each other, preventing both threads from continuing
- Bugs that happen only in certain situations and are hard to reproduce and fix reliably

## Creating a New Thread with spawn

With `thread::spawn` a new thread can be created. It then executes the code provided as a closure.

```rs
use std::thread;
use std::time::Duration;

fn main() {
    // if the main thread ends the program will stop!
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
```

## Waiting for All Threads to Finish Using join Handles

The `thread::spawn` function returns a value of the type `JoinHandle`. It is possible to call `wait` on it which will wait untill the thread has ended.

```rs
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
```

## Using move Closures with Threads

With the `move` keyword one can pass variables to a thread which it then will own.

```rs
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
```

## Using Message Passing to Transfer Data Between Threads

With the help of _channels_ rust enables message passing between threads. A channel has two halves: a transmitter and a receiver. A new channel can be created with `mpsc::channel`. `mpsc` stands for _multiple producers, single consumer_. A channel in rust therefore can have multiple sending parts but only one recieving part.

```rs
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```

## Channels and Ownership Transference

The ownership rules play a vital role in message sending because they help you write safe, concurrent code. The following code **will not work**!

```rs
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        // this takes ownership
        tx.send(val).unwrap();
        // this is not allowed due to ownership!
        println!("val is {}", val);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```

## Sending Multiple Values and Seeing the Receiver Waiting

See function `channel_with_multiple_values` in file `src/main.rs`

## Creating Multiple Producers by Cloning the Transmitter

See function `channel_with_multiple_transmitters` in file `src/main.rs`

# Shared-State Concurrency

In a way, channels in any programming language are similar to single ownership, because once you transfer a value down a channel, you should no longer use that value. Shared memory concurrency is like multiple ownership: multiple threads can access the same memory location at the same time.

## Using Mutexes to Allow Access to Data from One Thread at a Time

the term _Mutex_ is an abbreviation for _mutual exclusion_. A mutex allows only one thread to access particular data at a given time. Management of mutexes can be incredibly tricky to get right, which is why so many people are enthusiastic about channels. However, **thanks to Rust’s type system and ownership rules, you can’t get locking and unlocking wrong**!

## The API of Mutex<T>

A new muxtex can be created with a call to `Mutex::new(5)`. The following code shows a single thread mutex:

```rs
use std::sync::Mutex;

fn main() {
    // create a mutex
    let m = Mutex::new(5);

    {
        // aquire a lock on the muxtex
        // if it would not be available the thread would panic
        let mut num = m.lock().unwrap();
        // change the mutex value
        *num = 6;
    } // muxtex will be unlocked here (out of scope)

    println!("m = {:?}", m);
}
```

A ` Mutex<T>` is a smart pointer. More accurately, the call to lock _returns_ a smart pointer called `MutexGuard`, wrapped in a `LockResult` that we handled with the call to `unwrap`.

## Multiple Mutex<T> Ownership with Multiple Threads using Arc<T>

`Arc<T>` is a type like `Rec<T>` that is safe to use in concurrent situations.

See function `mutex_with_multiple_threads` in file `src/main.rs`

# Extensible Concurrency with the Sync and Send Traits

## Allowing Transference of Ownership Between Threads with Send

The `Send` marker trait indicates that ownership of values of the type implementing `Send` can be transferred between threads. Almost every Rust type is `Send`, but there are some exceptions, including `Rc<T>`.

## Allowing Access from Multiple Threads with Sync

The `Sync` marker trait indicates that it is safe for the type implementing `Sync` to be referenced from multiple threads.
