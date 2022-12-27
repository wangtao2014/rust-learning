pub struct Person {
    name: String,
    age: i32,
    height: f32,
    email: String,
}

impl Person {
    pub fn new(name: String, age: i32, height: f32, email: String) -> Self {
        Self {
            name,
            age,
            height,
            email,
        }
    }

    pub fn print_person(&self) {
        println!(
            "name: {} \n age: {} \n height: {} \n email: {} ",
            self.name, self.age, self.height, self.email
        );
    }
}

pub fn hi_name(name: &str) {
    println!("hi {}", name);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let person = Person::new("wangtao".into(), 36, 172.0, "576217702@qq.com".into());
        person.print_person();
        assert_eq!(person.name, "wangtao");
        assert_eq!(person.age, 36);
        assert_eq!(person.height, 172.0);
        assert_eq!(person.email, "576217702@qq.com");
    }
}
