fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let user1 = User {
        username: "John".to_string(),
        email: "jhon.doe@contoso.com".to_string(),
        sign_in_count: 1,
        active: true,
    };

    println!("User email is {}", user1.email);
}
