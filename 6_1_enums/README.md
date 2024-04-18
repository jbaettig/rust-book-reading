# Defining an Enum

Enums are used to enumerate stuff like in the following example.

```rs
// Possible IP Address types
enum IpAddrKind {
    V4,
    V6,
}

// instance a kind of ip address
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;

// using enums in structs
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};
```

Enums in rust can also have data associated to it directly.

```rs
enum IpAddr {
    V4(String),
    V6(String),
}

// call the enum v4 function to create an instance
let home = IpAddr::V4(String::from("127.0.0.1"));

// call the enum v6 function to create an instance
let loopback = IpAddr::V6(String::from("::1"));

// this also works with other types!
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

// or multiple values!
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

It is also possible for enums to have functions in it. They are very similar to structs.

```rs
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

A very common enum is `Option`. It can be used to represent values thate are currently absent.

```rs
let some_number = Some(5);
let some_char = Some('e');

let absent_number: Option<i32> = None;

// this code will not work since Option<i8> != i8
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;

```
