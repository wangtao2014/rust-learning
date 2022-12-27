pub mod longest;
pub mod area;
pub mod closuer;
pub mod shadow;
pub mod browwed;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let string1 = String::from("long string is longer");
        let string2;
        let result;
        {
            string2 = String::from("xyz");
            result = longest::longest(string1.as_str(), string2.as_str());
        }
        
        println!("The longest string is {}", result);
    }
}
