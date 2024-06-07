fn main() {
    println!("Hello, world!");
    another_function(4);

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y {y}");

    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let new_number = if condition { 5 } else { 6 };

    println!("result {new_number}");

    test();

    test_for_two();
}

fn another_function(another: i32) {
    println!("Another function {another}");

    let x = five();
    println!("The value of five is: {x}");
}

fn five() -> i32 {
    5
}

fn test() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn test_while() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn test_for() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn test_for_two() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
