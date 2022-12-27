pub fn browwed_test() {
    let s1 = gives_ownership();
    let s2 = String::from("haha");
    let s3 = takes_and_gives_back(s2);
    println!("xxxxx{}", s1);
    println!("yyyyy{}", s3);
}

pub fn gives_ownership() -> String {
    let s1 = String::from("hello world");
    s1
}

pub fn takes_and_gives_back(some_string: String) -> String {
    some_string
}

pub fn area(dim: (u32, u32)) -> u32 {
    dim.0 * dim.1
}

#[derive(Debug)]
pub enum UsState {
    Alaska,
    Abm,
}

pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

pub fn option_test() {
    let some_num = Some(5);
    let some_string = Some("hello world");

    let absent_number: Option<i32> = None;

    println!("z{:?}", some_num);
    println!("z{:?}", some_string);
    println!("z{:?}", absent_number);
}

pub fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("the state is {:?}", state);
            25
        },
    }
}

pub fn plus_one(option: Option<i32>) -> Option<i32> {
    match option {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// 代码更少，更加的简洁，放弃了穷举的可能，match 的语法糖
pub fn if_let() {
    let option = Some(3u8);

    match option {
        Some(3) => println!("three"),
        _ => println!("others"),
    }

    if let Some(3) = option {
        println!("three");
    } else {
        println!("others");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_browwed() {
        browwed_test();

        let rect = (30, 50);
        println!("{}", area(rect));

        let num = value_in_cents(Coin::Quarter(UsState::Alaska));
        println!("{}", num);

        option_test();
        if_let();
    }
}