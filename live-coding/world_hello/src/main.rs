mod random_self;
use random_self::random_test;

mod priority_self;
use priority_self as other_priority_self;

mod types;
mod value_tour;

use hello_utils::{ num_utils, str_utils };

#[allow(dead_code)]
fn function() {
    println!("called `function()`");
}

fn main() {

    // function();
    // random_test();
    // other_priority_self::function();
    // other_priority_self::nested::function();
    // other_priority_self::private_nested::function();
    // value_tour::value_tour();

    str_utils::hi_name("wangtao");
    let sum = num_utils::num_add(10, 20);
    println!("sum: {:?}", sum);
}
