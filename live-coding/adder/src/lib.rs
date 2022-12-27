use std::{result::*, io::{self, Read}, fs::{File, self}};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

pub struct Guess {
    pub value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess { 
        if value < 1 || value > 100 {
            panic!("Guess must be between 1 and 100, got {}", value);
        }
        Guess { value }
     }
}

pub fn read_username_from_file() -> Result<String, io::Error> {
    let filepath = "/Users/yuanliling/Documents/rust/live-coding/adder/src/hello.txt";
    // method 1
    // let f = File::open(filePath);
    // let mut f = match f {
    //     // 打开文件成功，将file句柄赋值给f
    //     Ok(file) => file,
    //     // 打开文件失败，将错误返回(向上传播)
    //     Err(e) => return Err(e),
    // };

    // let mut username = String::new();
    // match f.read_to_string(&mut username) {
    //     Ok(_) => Ok(username),
    //     Err(e) => Err(e),
    // }

    // method 2
    // let mut s = String::new();
    // File::open(filepath)?.read_to_string(&mut s)?;
    // Ok(s)

    // method 3
    fs::read_to_string(filepath)
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::ErrorKind};

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        
        let file = File::open("hello.txt");
        
        let f = match file {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(file) => file,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                other_error => panic!("Problem opening the file: {:?}", other_error),
            }
        };
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Sunface");
        let target = "Sunface";
        assert!(
            result.contains(target),
            "你的问候中并没有包含目标姓名 {} ，你的问候是 `{}`",
            target,
            result
        );
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn it_works_result() -> Result<(), String>{
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn read_from_file_test() {
        let username = read_username_from_file().unwrap();
        println!("username: {}", username);
        eprintln!("Error: Could not complete task");
    }
}
