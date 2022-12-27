pub struct Cache {}

impl Cache {
    pub fn new() -> Cache {
        return Cache {};
    }

    pub fn do_something(some_string: String) {
        println!("hello world {}", some_string);
    }
}