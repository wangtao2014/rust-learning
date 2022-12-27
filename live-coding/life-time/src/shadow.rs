pub fn shadow() -> i32 {
    let x = 1;
    let x = x + 2;
    let x = x * 3;
    x
}

// 整数类型
// 浮点类型
// 布尔类型
// 字符类型
pub fn parse_num() -> u32 {
    let num = "23".parse().expect("不是一个数字");
    num
}

// tuple
pub fn parse_tuple() -> (i32, f64, u8) {
    let tup: (i32, f64, u8) = (23, 8.9, 8);
    let (x, y, z) = tup;
    println!("x: {}, y: {}, z: {}", x, y, z);
    tup
}

// if condition
pub fn parse_condition() {
    let condition = true;
    let number = if condition { 1 } else { 0 };
    println!("the value of the number: {}", number);
}

// loop while for
pub fn parse_loop() {
    let mut condition = 0;

    let result = loop {
        condition += 1;

        if condition == 10 {
            break condition * 2;
        }
    };
    println!("the value of the number: {}", result);
}

pub fn parse_while() {
    let mut condition = 3;

    while condition != 0 {
        println!("number: {}", condition);
        condition = condition - 1;
    }
    // another method
    for number in (1..4).rev() {
        println!("{}!", number);
    }

    println!("lift off");
}

pub fn parse_for() {
    let array = [1, 2, 3, 4, 5, 6, 7, 8];

    for i in array.iter() {
        println!("number: {}", i);
    }
}

pub fn string_test() {
    let mut hello = String::from("hello");
    hello.push_str(", world");
    println!("{}", hello);
}

pub fn string_alloc() {
    let s1 = String::from("hello");
    let s2 = s1;
    print!("{}", s2); // borrow of moved value: `s1` value borrowed here after move
}

pub fn move_test() {
    let s1 = String::from("hello");

    take_ownership(s1); // move
    // println!("{}", s1);

    let x = 100;
    makes_copy(x); // copy
    println!("{}", x);
}

pub fn take_ownership(some_thing: String) {
    println!("{}", some_thing);
}

pub fn makes_copy(some_num: i32) {
    println!("{}", some_num);
}

// browwed
pub fn parse_for_browwed() {
    let mut hello = String::from("hello");
    let length = calculate_length(&mut hello);

    println!("the length of {} is {}", hello, length);
}
// 和变量一样，引用也是不可变的，如果需要改变，加 mut，限制：只能有一个可变引用（在同一个作用域）
pub fn calculate_length(s: &mut String) -> usize {
    s.push_str(", world");
    s.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shadow() {
        assert_eq!(shadow(), 9);
        assert_eq!(parse_num(), 23);
        assert_eq!(parse_tuple(), (23, 8.9, 8));
        parse_condition();
        parse_loop();
        parse_while();
        parse_for();
        parse_for_browwed();
        string_test();
    }
}