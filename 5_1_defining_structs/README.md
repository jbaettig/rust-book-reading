# Defining and Instantiating Structs

The following example shows how a struct is defined. The names and types inside the struct are called _fields_.

```rs
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

An instance of that struct can be created as follows:

```rs
 let user1 = User {
     active: true,
     username: String::from("someusername123"),
     email: String::from("someone@example.com"),
     sign_in_count: 1,
 };
```

If one wants to modify a field of a struct this can be done like this (Only if user1 is mut):

```rs
user1.email = String::from("anotheremail@example.com");
```

## Creating Instances from Other Instances with Struct Update Syntax

By using _struct update syntax_ it is possible to create a new instance of a struct that includes most of the values from another instance.

```rs
// create user2 with same properties like user 1
// but a different e-mail address
let user2 = User {
    email: String::from("another@example.com"),
    ..user1
};
```

-> Important to note in the code above is that the `username` field of user1 has been moved and can no longer be used. The fields `active` and `sign_in_count` are still valid because their types implement the `Copy` trait.

## Using Tuple Structs Without Named Fields to Create Different Types

Rust also supports structs that look similar to tuples, called _tuple structs_. Tuple structs have the added meaning the struct name provides but don’t have names associated with their fields.

```rs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
```
