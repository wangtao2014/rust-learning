use std::cmp::Ordering;
use std::io;
use rand::Rng;

// constant
const MAX_POINTS: u32 = 100_000;

fn main() {
    println!("猜数字游戏！！！{}", MAX_POINTS);
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("随机生成的数字是： {}", secret_number);

    loop {
        println!("猜测一个数字");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取行");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你猜的数字是：{}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Match!");
                break;
            }
        };
    }
}
