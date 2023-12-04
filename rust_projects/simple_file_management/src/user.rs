use std::fmt;
use std::ops;

/// Each user in the dabatase has:
/// * name
/// * surname
/// * age
pub struct User {
    pub name: String,
    pub surname: String,
    pub age: String,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}, {}", self.name, self.surname, self.age)
    }
}

impl ops::Add<i32> for User {
    type Output = Self;

    fn add(self, age_increase: i32) -> Self {
        Self {
            name: self.name,
            surname: self.surname,
            age: (self.age.parse::<i32>().unwrap() + age_increase).to_string(),
        }
    }
}
