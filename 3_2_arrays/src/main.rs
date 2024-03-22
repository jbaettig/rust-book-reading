fn main() {
    let a = [1, 2, 3, 4, 5];
    println!("some array a: {:?}", a);
    println!("first element of a: {}", a[0]);
    println!("second element of a: {}", a[1]);

    let b = [3; 5];
    println!("some array b: {:?}", b);

    println!("accessing out of bound -> panic!");
    let _c = a[5];
}