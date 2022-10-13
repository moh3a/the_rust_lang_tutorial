#[allow(dead_code)]
pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        pub fn seat_at_table() {}
    }

    pub mod serving {
        pub fn take_order() {}
        pub fn server_order() {}
        pub fn take_payment() {}
    }
}

#[allow(dead_code)]
pub mod back_of_house {
    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast { toast: String::from(toast), seasonal_fruit: String::from("peaches") }
        }
    }

    pub enum Appetizer {
        Soup, Salad
    }

    pub fn fix_incorrect_order () {
        cook_order();
        // super::serve_order();
    }
    fn cook_order() {}
}
