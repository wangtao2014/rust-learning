mod back_of_house;
mod vec_test;
mod string_test;

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::hosting::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    meal.seasonal_fruit = String::from("blueberries");
    println!("I'd like to eat {:#?}", meal);
}

pub fn vec_test_another() {
    vec_test::vec_new();
    vec_test::vec_read();
    vec_test::vec_borrow();
    vec_test::vec_iter();
}

pub fn string_test_another() {
    string_test::string_iter();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        eat_at_restaurant();
        vec_test_another();
        string_test_another();
    }
}
