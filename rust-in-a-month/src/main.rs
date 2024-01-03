// // fn main() {
// //     println!("{:?}", "a".as_bytes());
// // }

// // fn print_months() {
// //     // This function takes no input!
// //     println!("Number of months in the year: {NUMBER_OF_MONTHS}");
// // }

// fn match_number(input: i32) {
//     match input {
//         number @ 4 => println!("{number} is unlucky in China (sounds close to 死)!"),
//         number @ 13 => println!("{number} is lucky in Italy! In bocca al lupo!"),
//         number @ 14..=19 => println!("Some other number that ends with -teen: {number}"),
//         _ => println!("Some other number, I guess"),
//     }
// }

// use core::num;

// fn main() {
//     let str1 = "Hello!";
//     println!(
//         "str1 is {} bytes and also {} characters.",
//         str1.len(),
//         str1.chars().count()
//     );
//     let str2 = "안녕!";
//     println!(
//         "str2 is {} bytes but only {} characters.",
//         str2.len(),
//         str2.chars().count()
//     );

//     println!("The smallest i8: {} The biggest i8: {}", i8::MIN, i8::MAX);
//     println!("The smallest u8: {} The biggest u8: {}", u8::MIN, u8::MAX);
//     println!(
//         "The smallest i16: {} The biggest i16: {}",
//         i16::MIN,
//         i16::MAX
//     );
//     println!(
//         "The smallest u16: {} and the biggest u16: {}",
//         u16::MIN,
//         u16::MAX
//     );
//     println!(
//         "The smallest i32: {} The biggest i32: {}",
//         i32::MIN,
//         i32::MAX
//     );
//     println!(
//         "The smallest u32: {} The biggest u32: {}",
//         u32::MIN,
//         u32::MAX
//     );
//     println!(
//         "The smallest i64: {} The biggest i64: {}",
//         i64::MIN,
//         i64::MAX
//     );
//     println!(
//         "The smallest u64: {} The biggest u64: {}",
//         u64::MIN,
//         u64::MAX
//     );
//     println!(
//         "The smallest i128: {} The biggest i128: {}",
//         i128::MIN,
//         i128::MAX
//     );
//     println!(
//         "The smallest u128: {} The biggest u128: {}",
//         u128::MIN,
//         u128::MAX
//     );

//     let var = 8;
//     let var_reference = &var;
//     println!("{}", var_reference);

//     let my_string: String = "Try to make this a String".into();
//     println!("{}", my_string);

//     const NUMBER_OF_MONTHS: u32 = 12;
//     // print_months();

//     // let mut number = 10;
//     // let number_ref = &number;
//     // let number_change = &mut number;
//     // *number_change += 10;
//     // println!("{}", number_ref);
//     let number = 9;
//     let number_ref = &number;
//     println!("{:p}", number_ref);

//     let number = 555;
//     println!(
//         "Binary: {:b}, hexadecimal: {:x}, octal: {:o}",
//         number, number, number
//     );
//     let father_name = "Vlad";
//     let son_name = "Adrian Fahrenheit";
//     let family_name = "Țepeș";
//     println!(
//         "This is {1} {2}, son of {0} {2}.",
//         father_name, son_name, family_name
//     );
//     println!(
//         "{city1} is in {country} and {city2} is also in {country}, but {city3} is not in {country}.",
//         city1 = "Seoul",
//         city2 = "Busan",
//         city3 = "Tokyo",
//         country = "Korea"
//     );
//     let mut num_vec: Vec<char> = Vec::new();
//     println!("{}", num_vec.capacity()); // 0 elements: prints 0
//     num_vec.push('a'); // add a character
//     println!("{}", num_vec.capacity()); // 1 element: prints 4.
//                                         // Vecs with 1 item always start with capacity 4
//     num_vec.push('a'); // add one more
//     num_vec.push('a'); // add one more
//     num_vec.push('a'); // add one more
//     println!("{}", num_vec.capacity()); // 4 elements: still prints 4.
//     num_vec.push('a'); // add one more
//     println!("{}", num_vec.capacity()); // prints 8.
//                                         // We have 5 elements, but it doubled 4 to 8 to make space

//     let random_tuple = ("Here is a name", 8, vec!['a'], 'b', [8, 9, 10], 7.7);
//     println!("Inside the tuple is: First item: {:?} Second item: {:?} Third item: {:?} Fourth item: {:?} Fifth item: {:?} Sixth item: {:?}", random_tuple.0, random_tuple.1, random_tuple.2, random_tuple.3, random_tuple.4, random_tuple.5,);
//     match_number(50);
//     match_number(13);
//     match_number(16);
//     match_number(4);

//     let mut counter = 0;
//     let mut counter2 = 0;
//     println!("Now entering the first loop.");
//     'first_loop: loop {
//         counter += 1;
//         println!("The counter is now: {}", counter);
//         if counter > 5 {
//             println!("Now entering the second loop.");
//             'second_loop: loop {
//                 // now we are inside 'second_loop
//                 println!("The second counter is now: {}", counter2);
//                 counter2 += 1;
//                 if counter2 == 3 {
//                     break 'first_loop;
//                 }
//             }
//         }
//     }

// }

// #[derive(Debug)]
// struct ColorRgb(u8, u8, u8);
// #[derive(Debug)]
// struct SizeAndColor{
//     size:i32,
//     color:ColorRgb,
// }
// fn main() {
//     let my_color = ColorRgb(10,10,10);
//     let size_and_color = SizeAndColor{
//         size: 10,
//         color: my_color,
//     };
// }

// enum ThingsInTheSky {
//     Sun,
//     Star,
// }
// fn create_sun_star(time: i32) -> ThingsInTheSky {
//     match time {
//         6..=18 => ThingsInTheSky::Sun,
//         _ => ThingsInTheSky::Star,
//     }
// }
// fn check_sky_status(sky_status: &ThingsInTheSky) {
//     match sky_status {
//         ThingsInTheSky::Sun => println!("i can see sun"),
//         ThingsInTheSky::Star => print!("i can see star"),
//     }
// }
// fn main() {
//     let time = 19;
//     let sky_status = create_sun_star(time);
//     check_sky_status(&sky_status);
// }

//-----------------------------------------has map
// use std::collections::HashMap;
// fn main() {
//     let a = String::from("a");
//     let b = String::from("b");
//     let mut c = HashMap::new();
//     // c.insert(&a, "aaa");
//     // c.insert(&b, "bbb");
//     c.insert(a, 10);
//     c.insert(b, 20);

//     println!("{:#?}", c);
//     let key_name = String::from("a");
//     let avalue = c.get(&key_name);
//     println!("{:?}", avalue);

//     for (key, value) in &c {
//         println!("key = {}, value = {}", key, value);
//     }

//     c.insert(String::from("a"), 00);
//     c.insert(String::from("a"), 40);
//     println!("{:#?}",c);

//     c.entry(String::from("a")).or_insert(55);
//     c.entry(String::from("a")).or_insert(90);
// }
//------------------------------------------------error handling
// use std::fs::File;
// use std::io:: ErrorKind;

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// use core::num;
// fn main() {
//     let f = File::open("hello1.txt").expect("failed to open file");
//     println!("{:#?}",f);
//     // let f = match f {
//     //     Ok(file) => file,
//     //     Err(error) => match error.kind() {
//     //         ErrorKind::NotFound => match File::create("hello1.txt") {
//     //             Ok(fc) => fc,
//     //             Err(err) => panic!("Error while creating file: {:?}", err),
//     //         },
//     //         _ => panic!("Error opening file: {:?}", error),
//     //     },
//     // };
//     // println!("{:#?}", f);
// }
//-------------------------------------- error propogation
// use std::{fs::File, io, io::Read, char::REPLACEMENT_CHARACTER};
// fn read_user_name_from_file() -> Result<String, io::Error> {
//     let mut file = match File::open("hello.txt") {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut contents = String::new();
//     match file.read_to_string(&mut contents) {
//         Ok(_) => Ok(contents),
//         Err(e) => Err(e),
//     }
// }
// simplify above function
// fn read_user_name_from_file()->Result<String, io::Error>{
//     let mut name = String::new();
//     let mut file = File::open("/home/ad.rapidops.com/mugish.beldar/Mugish Git Repository/gitrepo/Rust/rust-in-a-month/src/files/hello.txt")?;
//     file.read_to_string(&mut name)?;  // used to add file contents in to name
//     Ok(name)
// }
// fn main() {
//     match read_user_name_from_file() {
//         Ok(username) => println!("User name: {}", username),
//         Err(e) => eprintln!("Error reading file: {}", e),
//     }
// }

// //-------------------------------generic type
// struct Gentype<T, U> {
//     x: T,
//     y: U,
// }
// enum Option<T> {
//     Some(T),
//     None,
// }
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }
// fn main() {
//     let number = vec![1, 2, 3, 4, 5];
//     let larg_num = largest_number(number);
//     println!("{}", larg_num);
//     let character = vec!['a', 'b', 'c', 'd', 'e'];
//     println!("{}", largest_number(character));

//     let p1 = Gentype { x: 10, y: 11 };
//     let p2 = Gentype { x: 2.0, y: 3.0 };
// }

// // fn largest_number(number: Vec<i32>) -> i32 {
// //     let mut largest_num = number[0];
// //     for num in number {
// //         if num > largest_num {
// //             largest_num = num;
// //         }
// //     }
// //     largest_num
// // }
// // generic typed function
// fn largest_number<T: PartialOrd + Copy>(number: Vec<T>) -> T {
//     let mut largest_num = number[0];
//     for num in number {
//         if num > largest_num {
//             largest_num = num;
//         }
//     }
//     largest_num
// }

//----------------------------------Traits
// use std::fmt::format;
// pub struct NewsArticle {
//     author: String,
//     headline: String,
//     content: String,
// }
// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{} by {}", self.headline, self.author)
//     }
// }
// pub struct Tweet {
//     username: String,
//     content: String,
//     retweet: bool,
//     reply: bool,
// }
// impl Summary for Tweet {
//     fn summarize(&self) -> String {
//         format!("{} by {}", self.retweet, self.username)
//     }
// }
// pub trait Summary {
//     fn summarize(&self) -> String;
// }

// fn main() {
//     let first_tweet = Tweet {
//         username: String::from("abc"),
//         content: String::from("xyz"),
//         reply: false,
//         retweet: false,
//     };
//     let first_newsarticle = NewsArticle {
//         author: String::from("abc"),
//         headline: String::from("headline"),
//         content:String::from("content"),
//     };
//     println!("{}", first_tweet.summarize());
//     println!("{}", first_newsarticle.summarize());
// }

fn longest_str<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
fn main() {
    let str1 = String::from("abc");
    let str2 = String::from("xyzd");

    let result = longest_str(str1.as_str(), str2.as_str());
    println!("{}", result);
}
