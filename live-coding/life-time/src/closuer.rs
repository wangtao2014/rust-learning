use std::{thread, time::Duration};

pub fn muuuuu(intensity: u32) -> u32 {
    println!("muuuuu.....");
    thread::sleep(Duration::from_secs(2));
    intensity
}

pub fn workout(intensity: u32, random_number: u32) {
    // method 1
    // let action = muuuuu;

    // method 2
    /*
     * |param1, param2,...| {
            语句1;
            语句2;
            返回表达式
        }
     */
    let action2 = || {
        println!("muuuuu.....");
        thread::sleep(Duration::from_secs(2));
        intensity
    };
    
    if intensity < 25 {
        println!(
            "今天活力满满，先做 {} 个俯卧撑!",
            action2()
        );
        println!(
            "旁边有妹子在看，俯卧撑太low，再来 {} 组卧推!",
            action2()
        );
    } else if random_number == 3 {
        println!("昨天练过度了，今天还是休息下吧！");
    } else {
        println!(
            "昨天练过度了，今天干干有氧，跑步 {} 分钟!",
            action2()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_muuuuu() {
        let intensity = 10;
        let random_number = 7;

        workout(intensity, random_number);
    }

}