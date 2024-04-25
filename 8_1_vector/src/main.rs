fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // use when you want to panic
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    // use when you want to handle out of bound
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // iterating over a vector (mutable)
    let mut v2 = vec![100, 32, 57];
    for i in &mut v2 {
        *i += 50;
        println!("i {}", i);
    }

}
