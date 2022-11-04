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

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // Pattern that binds to value
}

// The match control flow construct
fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
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

    let qt = Coin::Quarter(UsState::Alaska);
    value_in_cents(&qt);

    // Matching with Option<T>
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1)
        }
    }

    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);

    // Catch-all patterns and the _ placeholder
    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
        // _ => re_roll()
    } // dice_roll

    // Concise control flow with if let
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let mut count = 0;
    if let Coin::Quarter(state) = qt {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    } // fi

}
