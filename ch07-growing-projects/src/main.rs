use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    println!("Hello, world! From the main binary.");

    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);

}
