use std::{fs::File, io::{Write, Read, Error}};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Gender {
    Unspecified = 0,
    Male = 1,
    Female = 2,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct User {
    pub name: String,
    age: u8,
    pub(crate) gender: Gender,
}

impl User {
    pub fn new(name: String, age: u8, gender: Gender) -> Self { 
        Self { name, age, gender } 
    }

    pub fn load(filename: &str) -> Result<Self, Error> {
        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let user = serde_json::from_str(&contents)?;
        Ok(user)
    }

    pub fn persist(&self, filename: &str) -> Result<usize, Error> {
        let mut file = File::create(filename)?;
        let data = serde_json::to_string(self)?;
        file.write_all(data.as_bytes())?;
        Ok(data.len())
    }
}

impl Default for User {
    fn default() -> Self {
        User::new("unknown name".into(), 0, Gender::Unspecified)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user() {
        let user = User::new("Wangtao".into(), 36, Gender::Male);
        let path  = "user1";
        user.persist(path).unwrap();
        
        let user1 = User::load(path).unwrap();
        assert_eq!(user, user1);
    }
}