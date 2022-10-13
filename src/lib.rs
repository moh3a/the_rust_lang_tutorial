mod e5_module_system;
use crate::e5_module_system::{front_of_house::hosting, back_of_house::Appetizer};

pub fn eat_at_restaurant() {
    // absolute path to module
    crate::e5_module_system::front_of_house::hosting::add_to_waitlist();

    // relative path
    e5_module_system::back_of_house::fix_incorrect_order();
    // e5_module_system::front_of_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    
    let mut meal = e5_module_system::back_of_house::Breakfast::summer("Rye");
    println!("{:?}", meal);

    meal.toast = String::from("Wheat");

    let _soup = Appetizer::Soup;
    let _salad = Appetizer::Salad;
}
