fn main() {
    let mut s3 = String::from("foo");
    let s4 = "bar";
    s3.push_str(s4);
    s3.push('!');
    println!("s3 is {s3}");

    let s5 = String::from("Hello, ");
    let s6 = String::from("world!");
    let s7 = s5 + &s6; // note s1 has been moved here and can no longer be used
    println!("s7 is {s7}");
}
