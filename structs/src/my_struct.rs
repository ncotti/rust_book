pub struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl User {
    pub fn get_email(&self) -> &String {
        &self.email
    }

    pub fn set_email(&mut self, data: &str) {
        self.email = String::from(data); 
    }

    pub fn new(email: &str, username: &str) -> User {
        User {
            email: String::from(email),
            username: String::from(username),
            active: true,
            sign_in_count: 1,
        }
    }
}