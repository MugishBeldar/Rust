fn function() {
    println!("Hello, world!");
}

fn main() {
    println!("Hello, world!");
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 10;
    println!("The value of x is {}", x);

    // data types
    // scalar data types
    // integer
    // float
    // boolean
    // character
    let int = 5;
    let float = 5.011;
    let boolean = true;
    let character = 'A';
    println!(" {} {} {} {} ", int, float, boolean, character);

    // compound data types
    // tup
    // array
    let tup = (123, "hello");
    let (num, s) = tup;
    println!("{} {}", num, s);

    // function
    function();

    // loop
    let arr = [1, 2, 3, 4, 5, 6, 7, 8];

    for element in arr.iter() {
        println!("{}", element);
    }

    for number in 1..4 {
        println!("{}", number);
    }

    let mut w = 0;
    while (w <= 10) {
        println!("{}", w);
        w = w + 1;
    }
}
