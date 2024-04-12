fn main() {
    scope();
    string();
    string_clone();
}

fn scope() {
    // string literal (hardcoded string)
    let s = "hello"; // s is valid here (in scope)
    println!("{}", s);
} // s is not valid anymore here (out of scope)

fn string() {
    let mut s = String::from("hello");
    // push_str() appends a literal to a String
    s.push_str(", world!");
    // This will print `hello, world!`
    println!("{}", s);
}

fn string_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}
