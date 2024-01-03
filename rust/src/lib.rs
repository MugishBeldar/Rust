// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_wait_list() {}
//     }
// }

// fn eat_at_restaurant() {
//     // relative path
//     crate::front_of_house::hosting::add_to_wait_list();
//     // absolute path
//     front_of_house::hosting::add_to_wait_list();
// }

// // using super keyword
// fn sever_order() {}

// mod back_of_house {
//     fn fix_pending_order() {
//         cook_order();
//         super::sever_order();
//     }
//     fn cook_order() {}
// }

// // with struct and struct association functions
// mod back_of_house_2 {
//     pub struct Breakfast {
//         pub toast: String,
//         pub seasonal_fruit: String,
//     }
//     impl Breakfast {
//       pub fn summer(toast: &str)-> Breakfast {
//         Breakfast {
//           toast: String::from(toast),
//           seasonal_fruit:  String::from("abc"),
//         }
//       }
//     }
// }
// pub fn eat_restaurants() {
//   let mut meal = back_of_house_2::Breakfast::summer("abcdefg");
//   meal.toast = String::from("xyz");
// }

//------------------------------------------------------------------------------
// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_wait_list() {}
//     }
// }

// pub use front_of_house::hosting;

// pub fn eat_at_restaurants() {
//   hosting::add_to_wait_list();
//   hosting::add_to_wait_list();
//   hosting::add_to_wait_list();
// }

//--------------------------------------------------------saparate above code in different files
mod front_of_house;
    
pub use front_of_house::hosting;

pub fn eat_at_restaurants() {
  hosting::add_to_wait_list();
  hosting::add_to_wait_list();
  hosting::add_to_wait_list();
} 