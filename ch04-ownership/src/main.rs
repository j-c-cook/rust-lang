fn main() {
    // 4.1 OWNERSHIP
    // mutable string
    let mut s: String = String::from("Hello");
    s.push_str(", world!");
    println!("{s}");

    // Move
    let x = 5;
    let y = x; // copy 5

    let s1 = String::from("Hello");
    let s2 = s1;  // call s1.drop() and give ownership to s2 (i.e. move)

    // Clone
    let s3 = s2.clone();
    println!("{s2}\t{s3}");

    // Copy -- similar to clone

    // Taking ownership vs. making copy depends on whether the object has a copy trait
    takes_ownership(s3);  // s3 is dropped
    makes_copy(y);

    let s4: String = gives_ownership();
    let s5: String = takes_and_gives_back(s4);

    let (_s2, _len) = calc_length(s5);

    // 4.2 REFERENCES
    // TODO: Continue here


}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
} // takes_ownership

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {

    a_string
}

fn calc_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}