// Defining an Enum
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

// fn route(ip_kind: IpAddrKind) {}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}


fn main() {
    // Enum values
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    // The option enum and its advantage over null values
    // https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#the-option-enum-and-its-advantages-over-null-values

}
