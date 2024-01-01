// #[allow(unused_variables)]
// use std::{
//     fmt::format,
//     ops::{Range, RangeInclusive},
//     vec,
// };

//---------------------- Const
// fn main() {
//     let num = 1;
//     const N: i32 = 7;
//     // N+=1;
//     println!("{}", num);
//     println!("{}", N);
// }

//----------------------Literals
// fn main() {
//     let one = 1;
//     let two: i32 = 2;
//     let three = 3i8;
//     let four = 4_i8;
//     println!("{}", one);
//     println!("{}", two);
//     println!("{}", three);
//     println!("{}", four);
// }

//--------------------- Type Casting Two way implicit(automaticaly not supported by rust) explicit(manually);
// fn main() {
//     let num = 1;
//     println!("{} - byte", std::mem::size_of_val(&num));
//     let num2 = 2i16;
//     println!("{}  - byte", std::mem::size_of_val(&num2));
//     let convertedtype = num as i16;
//     println!("{} - converted type", std::mem::size_of_val(&convertedtype));
// }

//-------------------- Operator in rust - arethmentic, compound assigment, comparision , logical, bitwise operator
// fn main() {
//     // 1+2
//     // arithmetic operator
//     println!("{}", 1 + 2);
//     println!("{}", 1 - 2);
//     println!("{}", 1 * 2);
//     println!("{}", 1.0 / 2.0);
//     println!("{}", 11 % 2);
//     // comparison operator
//     println!("{}", 12 < 18);
//     println!("{}", 12 > 18);
//     println!("{}", 12 == 18);
//     println!("{}", 12 != 18);
//     /*
//     ==
//     =  let name = 12;
//     */
//     // logical operator
//     println!("{}", 12 == 12 || 13 == 1 || 2 == 2);
//     println!("{}", !1 == 1);
//     let mut a = 12;
//     println!("{}", a);
//     a += 10;
//     println!("{}", a);
// }

// --------------------Scope and shadowing in rust
// fn main() {
//     let age = 20;
//     println!("{}", age); // not found error
//     let age = 22;
//     println!("{}", age);
//     {
//         // shadowing allows you to re-declaire a variable in the same scope.
//         let address = "demo address";
//         let age = 23; // shadowing allows you to re-declaire a variable
//         println!("{}", age);
//         println!("{}", address);
//     }
//     // println!("{}", address);  // not found error
// }

//-----------------------String in rust
// fn main() {
//     // String litrals
//     let str1: &str = "string1";
//     println!("{}", str1);
//     // String Object
//     // 1. using new keyword
//     let mut str2 = String::new(); // return empty string
//     println!("{}", str2);
//     // push string in this empty string
//     str2.push_str("str2 push");
//     // push char in str2
//     str2.push('c');
//     println!("{}", str2);
//     // 2 using from keyword
//     let str3 = String::from("string3");
//     println!("{}", str3);
// }

//--------------------------stirng slicing
// fn main() {
//     let str1 = "string1".to_string();
//     let str2 = str1[1..3].to_string();
//     println!("{}", str2);
//     let str3 = &str1[0..7]; // in & .to_string() not required
//     println!("{}", str3);
// }

//--------------------------string concatenation
// fn main() {
//     // 1. using format keyword --> used for both string litrals and string objecti
//     let str1 = String::from("string1");
//     let str2 = String::from("string2");
//     let con = format!("{} {}", str1, str2);
//     let con1 = format!("{} {}", str1, str1);
//     let con2 = format!("{} {}", str2, str2);
//     println!("{}", con);
//     println!("{}", con1);
//     println!("{}", con2);
//     // 2. using + operator --> used only for string objects
//     let con3 = str1 + " " + &str2; // why we use & in str2
//     println!("{}", con3);
// }

//--------------------------string methods
// fn main() {
//     //     1. new() ---- Creates a new empty String.
//     // 2. from() ---- Creates a new String having default value.
//     // 3. to_string() ---- Converts string literal to string object.
//     // 4. replace() ---- Replaces all matches of a pattern with another string.
//     // 5. push() ---- Appends the given char to the end of this String.
//     // 6. push_str() --- Appends a given string onto the end of this String.
//     // 7. len() ---- Returns the length of the String.
//     // len
//     let str1 = "string1".to_string();
//     println!("{}", str1.len());
//     // replace
//     let change = str1.replace("st", "ST");
//     println!("{}", change);
// }

//---------------------------escape sequences
// fn main() {
//     println!("Hello\nworld!");
//     println!("Hello\\world!");
//     println!("Hello\'world!");
//     println!("Hello\"world!");
//     println!("Hellooo \rworld!");
// }

//---------------------------decision making if...else, if..elseif...else
// fn main() {
//     let num = 0;
//     let num1 = 2;
//     let num3 = 1;
//     // if...else
//     if num == num3 {
//         println!("true");
//     } else {
//         println!("false");
//     }
//     // if...elseif...else
//     if num == num3 {
//         println!("Inside If...");
//     } else if num1 >= num {
//         println!("Inside ElseIf...");
//     } else {
//         println!("Inside Else...");
//     }
// }

//---------------------------match statement same as switch statement
// fn main() {
//     let num = 3;
//     match num {
//         // match a single value
//         1 => println!("1"),
//         // match a several values
//         2 | 3 | 4 | 5 => println!("2,3,4,5"),
//         // match an inclusive range
//         6..=10 => println!("Inclusive range 6..=10"),
//         // handle rest of cases
//         _ => println!("Special case.."),
//     }
// }

//--------------------------Loop in rust
// fn main() {
//     // for loop
//     for i in 1..=10 {
//         println!("i = {}", i);
//     }
//     for i in (1..=10).rev() {
//         println!("i = {}", i);
//     }
//     // string is not itrator in rust
//     // for i in "loop in string" {
//     //     println!("{}", i);
//     // }
//     // while loop
//     let mut num = 1;
//     while num <= 10 {
//         println!("num = {}", num);
//         num += 1;
//     }
//     // loop in rust
//     loop {
//         println!("Used for infinite operation");
//     }
// }

//----------------------------continue and break statement
// fn main() {
//     let mut num = 1;
//     while num <= 10 {
//         if num == 5 {
//             // num += 1;
//             // continue;
//             break;
//         }
//         println!("num = {}", num);
//         num += 1;
//     }
//     for num in 0..10 {
//         if num == 5 {
//             // continue;
//             break;
//         }
//         println!("num = {}", num);
//     }
// }

//----------------------------get input from user
// fn main() {
//     // for string
//     let mut line = String::new();
//     println!("Enter Here");
//     let _b1 = std::io::stdin().read_line(&mut line).unwrap();
//     println!("{}", line);
//     // for int
//     let _x: i32 = line.trim().parse().expect("Input not an integer");
//     println!("{}", _x);
// }

//----------------------------tuple in rust
// fn main() {
//     let data: (i32, f32, bool, char, &str) = (10, 10.10, false, 't', "tuple");
//     println!("{:?}", data);
//     println!("{}", data.0);
//     println!("{}", data.1);
//     println!("{}", data.2);
//     // not possible
//     // let data:(i32||f32||bool||char||&str) = (10.0, 10, 10.10, false, 't', "tuple");
// }

//---------------------------array
// fn main() {
//     let arr = [100, 200, 300];
//     // let arr = ["array", 100,200,300, ]; // not possible in rust
//     println!("{:?}", arr);
//     let arr1: [i32; 4] = [0, 1, 2, 3];
//     println!("{:?}", arr1);
//     println!("{}", arr1[2]);
//     let arr2: [i32; 4] = [10; 4];
//     println!("{:?}", arr2);
// }

//---------------------------looping on array
// fn main() {
//     let arr1 = [1, 2, 3, 4, 5, 6, 7];
//     for i in arr1.iter() {
//         println!("{}", i);
//     }
// }

//---------------------------multidimention array
// fn main() {
//     let arr2 = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
//     for i in arr2.iter() {
//         for j in i.iter() {
//             print!("{}", j);
//         }
//         println!("\n\n");
//     }
// }

//---------------------------functions
// fn returning_sum()->i32{
//     let num1 = 10;
//     let num2 = 10;
//     num1 + num2
// }
// fn parameter_sum(x:i32, y:i32) -> i32 {
//     x+y
// }
// fn sum() {
//     let num1 = 10;
//     let num2 = 10;
//     println!("{}", num1 + num2);
// }
// fn main() {
//     // demo_function();
//     sum();
//     println!("{}",returning_sum());
//     let sum = parameter_sum(20,20);
//     println!("{}", sum);
// }

//---------------------------vectore
// fn main() {
//     // first method
//     let mut v = vec![1, 2, 3, 4, 5, 6, 7];
//     v.push(100);
//     println!("{:?}", v);
//     v.pop();
//     // v.push("value"); // not possible
//     println!("{:?}", v);
//     for val in v {
//         println!("{}", val)
//     }
//     // second method
//     let mut v: Vec<i32> = Vec::new(); // return empty array
//     println!("{:?}", v);
//     v.push(10);
//     v.push(20);
//     println!("{}", v[0]);
// }

//---------------------------Ownership
// fn owenership(v: Vec<i32>) {
//     println!("{:?}", v);
// }
// fn return_ownership(v: Vec<i32>) -> Vec<i32> {
//     v
// }
// fn main() {
//     // theree way
//     // 1. assigning value to another variable (but here not work)
//     let num = 2;
//     println!("{}", num);
//     let num2 = num;
//     println!("{}", num2);
//     println!("{}", num);
//     // 2. passing in fuction parameter
//     let v = vec![1, 2, 3, 4, 5, 6, 7, 8];
//     owenership(v);
//     // println!("{:?}",v); // give moved error here owner ship transfer to the function.
//     // 3. by returning value to the function
//     let v1 = vec![1, 2, 3, 4, 5];
//     let v2 = return_ownership(v1);
//     // println!("{:?}",v1);  // give moved error here owner ship transfer to v2
//     println!("{:?}", v2);
// }

//---------------------------borrowing
// fn borrowing(v:&Vec<i32>) -> &Vec<i32> {
//     v
// }
// fn main() {
//     let vector_1 = vec![1, 2, 3, 4];
//     borrowing(&vector_1); // give value as borrow
//     println!("{:?}", vector_1); // its called borrowing
// }

//---------------------------struct
// struct Employee {
//     name: String,
//     age: i32,
//     address: String,
//     phone_no: i64,
// }
// fn struct_function(x: Employee) {
//     println!("fn name {}", x.name);
//     println!("fn age {}", x.age);
//     println!("fn address {}", x.address);
//     println!("fn phone no. {}", x.phone_no);
// }
// fn main() {
//     let mut employee_1 = Employee {
//         name: String::from("Employ one"),
//         age: 23,
//         address: String::from("Employ one address"),
//         phone_no: 10236547897,
//     };
//     println!("{}", employee_1.name);
//     println!("{}", employee_1.age);
//     println!("{}", employee_1.address);
//     println!("{}", employee_1.phone_no);
//     employee_1.phone_no = 9510824067;
//     println!("{}", employee_1.phone_no);
//     struct_function(employee_1);
// }

// //---------------------------method
// struct Rectangle {
//     length: i32,
//     width: i32,
// }
// // Method for rectangle structure
// impl Rectangle {
//     fn area(&self) -> i32 {
//         self.length * self.width
//     }
// }
// fn main() {
//     // make area method callable using struct
//     let area_1 = Rectangle {
//         length: 1000,
//         width: 20,
//     };
//     // call area method
//     let ans = area_1.area();
//     println!("{}", ans);
// }

//---------------------------enum
// #[derive(Debug)]
// enum  Gender{
//     Male,
//     Female,
// }
// #[derive(Debug)]
// struct People {
//     name:String,
//     gender:Gender,
// }
// fn main() {
//     let male = Gender::Male;
//     let female = Gender::Female;
//     println!("{:?}", male);
//     println!("{:?}", female);
//     let show = People {
//         name: String::from("People one"),
//         gender: Gender::Male,
//     };
//     println!("{}", show.name);
//     println!("{:?}", show.gender);
//     println!("{:?}", show);
// }

// ---------------------------------enum with data types
// #[derive(Debug)]
// enum Details {
//     Name(String),
//     Age(i32)
// }
// #[derive(Debug)]
// enum Vehicle {
//     Car,
//     Bike,
// }
// fn fn_for_enum(x:Vehicle) {
//     match x {
//         Vehicle::Bike => println!("bike"),
//         Vehicle::Car => println!("car"),
//     }
// }
// fn main() {
//     let set_name = Details::Name(String::from("f_name l_name"));
//     let set_age = Details::Age(20);
//     println!("{:?}",set_name);
//     println!("{:?}",set_age);
//     fn_for_enum(Vehicle::Bike);
//     fn_for_enum(Vehicle::Car);
// }

//----------------------------------closure
// fn main() {
//     // first
//     let x = |i: i32| -> i32 { i * 10 };
//     println!("{}", x(12));
//     // second
//     let y = | i | {i * 20};
//     println!("{}", y(12));
// }

//----------------------------------recursion
// fn fact(x: i32) -> i32 {
//     if x > 1 {
//         x * fact(x - 1)
//     } else {
//         x
//     }
// }
// fn main() {
//     println!("{}", fact(5));
// }

//----------------------------------concurrancy
// use std::{thread, time::Duration};
// fn main() {
//     // thread::spawn(||{
//     //     for i in 0..10 {
//     //         println!("spawn thread run {}", i);
//     //         thread::sleep(Duration::from_millis(1));
//     //     };
//     // });
//     // for i in 0..5 {
//     //     println!("main thread run {}", i);
//     //     thread::sleep(Duration::from_millis(1));
//     // };
//     // join handle
//     let handle = thread::spawn(||{
//         for i in 0..10 {
//             println!("spawn thread run {}", i);
//             thread::sleep(Duration::from_millis(1));
//         };
//     });
//     for i in 0..5 {
//         println!("main thread run {}", i);
//         thread::sleep(Duration::from_millis(1));
//     };
//     handle.join().unwrap();
// }

//----------------------------------module
// 1. single module
mod module_1 {
    pub fn details() {
        println!("from module one");
    }
}
// 2. sub module
mod module_2 {
    pub fn details() {
        println!("from module_2");
    }
}
mod module_3 {
    pub fn details() {
        println!("from module_3");
    }
}
// 3. nested module
mod module_4 {
    pub mod nested_module_1 {
        pub fn details() {
            println!("from nested module 1")
        }
    }
}
fn main() {
    module_1::details();
    module_2::details();
    module_3::details();
    module_4::nested_module_1::details();
}
