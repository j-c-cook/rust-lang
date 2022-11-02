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

    let _home = IpAddrKind::V4(127, 0, 0, 1);
    let _loopback = IpAddrKind::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    // The option enum and its advantage over null values
    let _some_number = Some(5);
    let _some_char = Some('e');

    let _absent_number: Option<i32> = None;

    let _x: i8 = 5;
    let _y: Option<i8> = Some(5);
    // let sum = _x + _y;  // Breaks

    // Option API documentation: https://doc.rust-lang.org/std/option/enum.Option.html

    // The match control flow construct
    // TODO: continue here -> https://doc.rust-lang.org/book/ch06-02-match.html#the-match-control-flow-construct
}
