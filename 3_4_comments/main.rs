fn main() {
    print_x(5);
    print_labeled_measurement(5, 'h');
    print_plus_one(5);
}

fn print_x(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn print_plus_one(x: i32) {
    let y = add_one(x);
    println!("plus one: {x} + 1 = {y}");
}

fn add_one(x: i32) -> i32 {
    x + 1
}
