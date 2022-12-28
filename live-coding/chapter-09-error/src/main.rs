use std::fs::File;
use std::io::{Error, ErrorKind, Read};

fn load_file() {
    let file = File::open("hello.txt");

    let _file = match file {
        Ok(file) => file,
        Err(err) => panic!("load file error: {}", err),
    };
}

fn load_file_unwrap() {
    // match 表达式的快捷表示方法
    // expert 和 unwrap 类似，但可指定错误信息
    let _file = File::open("hello.txt").unwrap();
    let _file = File::open("hello.txt").expect("Open file failed");
}

// 使用了很多match，很有用，但是很原始
// 使用闭包实现，Result<T, E> 有很多方法接受闭包为参数，使用 match 实现

fn create_file() {
    let file = File::open("hello.txt");
    
    let _file = match file {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(err) => panic!("Error creating file, {:?}", err),
            },
            oe => panic!("Error opening the file, {:?}", oe),
        },
    };
}

fn create_file_clouser() {
    let f = File::open("hello.txt").unwrap_or_else(|error|{
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Error creating file, {:?}", error);
            })
        } else {
            panic!("Error opening the file, {:?}", error);
        }
    });
}

fn read_username_from_file() -> Result<String, Error> {
    let file = File::open("hello.txt");

    let mut f = match file {
        Ok(file) => file,
        Err(err) => return Err(err),
    };

    let mut buf = String::new();
    
    match f.read_to_string(&mut buf) {
        Ok(_) => Ok(buf),
        Err(err) => Err(err),
    }
}
// ? 运算符 错误传播的快捷方式 如果 Result 是 Ok，Ok 中的值就是表达式的结果，继续执行
// 如果 Result 是 Err，Err 就是整个函数的返回结果，函数返回，类似使用 return

fn read_username_from_file_another() -> Result<String, Error> {
    let mut file = File::open("hello.txt")?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    Ok(buf)
}

// 链式调用
fn read_username_from_file_another2() -> Result<String, Error> {
    let mut buf = String::new();
    File::open("hello.txt")?.read_to_string(&mut buf)?;
    Ok(buf)
}

// ? 运算符只能用于返回 Result 的函数
// the `?` operator can only be used in a function that returns `Result` or `Option`
fn open_file_error() -> Result<(), Box<dyn std::error::Error>>{
    let _file = File::open("hello.txt")?;
    Ok(())
}

fn main() {
    // load_file(); // panic program exit
    // create_file();

    // create_file_clouser();
    // load_file_unwrap();
    let _result = read_username_from_file();
    let _result1 = read_username_from_file_another();
    open_file_error();
}