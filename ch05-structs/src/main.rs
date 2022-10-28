struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    // Using the Field Init shorthand
    User {
        email, // valid because email above is the correct name
        username,
        active: true,
        sign_in_count: 1,
    }
}

// Tuple structs that contain no fields
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Struct with no field
struct AlwaysEqual; // unit-like struct

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

// Defining a method
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Define a method with the same name as an instance
    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, &rect: Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }

}


fn main() {
    // Defining and initiating structs
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("some_user-name123"));

    // Creating instances from other instances with struct update syntax
    let user2 = User{
        email: String::from("another@example.com"),
            ..user1 // Fills the rest of the values those specified in user1
    };

    let _active = &user2.active;
    let _username = &user2.username;
    let _email = &user2.email;  // user1.email is now unavailable
    let _count = &user2.sign_in_count;

    // Using tuple structs without named fields to create different types
    let _white = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    // Unit-like structs without any fields
    let _subject = AlwaysEqual;

    // An example program using structs
    let scale = 2;
    let rect = Rectangle {
        height: dbg!(30 * scale),
        width: 50
    };

    dbg!(&rect);

    println!("rect is {:?}", rect);
    println!("rect is {:#?}", rect);

    // Method syntax
    println!("The area of the rectangle is {}", rect.area());

    // Associated functions
    let _sq = Rectangle::square(3);

}
