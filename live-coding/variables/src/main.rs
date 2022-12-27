
include!(concat!(env!("OUT_DIR"), "/hello.rs"));

fn main() {

    let mut count = 0;
    
    loop {
        count += 1;
        if count == 5 {
            break;
        }
    }
    println!("The value of count is: {}", count);

    println!("message: {}", message());

}

