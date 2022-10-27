struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email, // valid because email above is the correct name
        username,
        active: true,
        sign_in_count: 1,
    }
}


fn main() {
    // Build a user account
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("some_user-name123"));

    let user2 = User{
        email: String::from("another@example.com"),
            ..user1 // Fills the rest of the values those specified in user1
    };

    let _active = &user2.active;
    let _username = &user2.username;
    let _email = &user2.email;  // user1.email is now unavailable
    let _count = &user2.sign_in_count;

    // TODO: Continue...
    // https://doc.rust-lang.org/book/ch05-01-defining-structs.html#using-tuple-structs-without-named-fields-to-create-different-types

}
