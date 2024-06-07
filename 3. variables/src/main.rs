fn main() {
    let tup: (i32, f64, i32) = (6, 3.2, 23);

    let (_x, y, _z) = tup;

    println!("The value of y is: {y}");
}
