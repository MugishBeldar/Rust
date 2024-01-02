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

enum ThingsInTheSky {
    Sun,
    Star,
}
fn create_sun_star(time: i32) -> ThingsInTheSky {
    match time {
        6..=18 => ThingsInTheSky::Sun,
        _ => ThingsInTheSky::Star,
    }
}
fn check_sky_status(sky_status: &ThingsInTheSky) {
    match sky_status {
        ThingsInTheSky::Sun => println!("i can see sun"),
        ThingsInTheSky::Star => print!("i can see star"),
    }
}
fn main() {
    let time = 19;
    let sky_status = create_sun_star(time);
    check_sky_status(&sky_status);
}
