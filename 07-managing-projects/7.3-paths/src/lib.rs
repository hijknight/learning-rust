use crate::back_of_house::Breakfast;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // crate::front_of_house::hosting::add_to_waitlist();
    //
    // front_of_house::hosting::add_to_waitlist();

    // order food
    let mut meal: Breakfast = back_of_house::Breakfast::summer("Wheat");
    // change mind on bread
    meal.toast = String::from("Rye");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;


}

fn deliver_order() {

}

mod back_of_house {

    // enums have public fields when declared as pub
    pub enum Appetizer {
        Soup,
        Salad,
    }




    // default private fields when declared pub
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }


    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order()
    }

    fn cook_order() {}
}