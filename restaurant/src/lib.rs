mod back_of_house;
mod front_of_house;

pub use crate::front_of_house::hosting;
fn deliver_order() {}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Mango");

    //mind chnaged the toast to orange
    meal.toast = String::from("Orange");
    println!("I'd like {} toast please", meal.toast);

    hosting::add_to_waitlist();
}
