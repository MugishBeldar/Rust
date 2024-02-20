fn main() {
    // borrowing
    let x = 5;
    let _y = x;
    println!("The value of x is {}", x);

    let str1 = String::from("hello world");
    // let str2 = str1;  // value borrowed from str1
    let str2 = str1.clone();
    println!("The value of str1 is {}", str1);
    println!("The value of str2 is {}", str2);

    let mut str1 = String::from("hello world");
    str1.push_str("!");
    str1.pop();
    println!("The value of str1 is {}", str1);
}
