fn main() {
    let tup = (500, 6.4, 1);

    let (_x, y, _z) = tup;

    println!("The value of y is: {y}");
    println!("The value of index 0 is: {}", tup.0);
}