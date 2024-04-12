fn main() {
    let mut user = build_user(String::from("mail@test.ch"), String::from("franz"));
    println!("User active: {}", user.active);
    println!("User name: {}", user.username);
    println!("User mail: {}", user.email);
    println!("User sign in count: {}", user.sign_in_count);

    user.email = String::from("mail@example.com");

    println!("User mail: {}", user.email);

    // create some points
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("RGB of black is: {}{}{}", black.0, black.1, black.2);
    println!("XYZ of origin is: {}{}{}", origin.0, origin.1, origin.2);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// "real" struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
