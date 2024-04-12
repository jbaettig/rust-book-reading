fn main() {
    reference();

    let mut s = String::from("hello");
    change_mutable_reference(&mut s);
    println!("{}", s);
}

fn reference() {
    let s1 = String::from("hello");
    // pass s1 as a reference
    let len = calculate_length(&s1);
    // s1 was not moved and is still valid
    println!("The length of '{}' is {}.", s1, len);
}

// s is a reference to a String
fn calculate_length(s: &String) -> usize {
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

// mutable reference
fn change_mutable_reference(some_string: &mut String) {
    some_string.push_str(", world");
}
