mod my_struct;

fn main() {
    let mut user : my_struct::User = build_user(
        String::from("mail@example.com"), 
        String::from("Cotti"));

    user.set_email("new_mail@example.com");
}

fn build_user(email: String, username: String) -> my_struct::User {
    my_struct::User::new(&email, &username)
}
