struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user : User = build_user(
        String::from("mail@example.com"), 
        String::from("Cotti"));

    user.email = String::from("new_mail@example.com");
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
