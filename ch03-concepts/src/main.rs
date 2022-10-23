fn main() {
    // Shadowing
    let x = 5;
    {
        let x = x + 2;
        println!("Value of x in the inner scope: {x}");
    }
    println!("Value of x in the outer scope: {x}");

    // Indexing tuples
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    println!("{five_hundred}");

    // Arrays
    const A_SIZE: usize = 5;
    let a: [i32; A_SIZE] = [1, 2, 3, 4, 5];
    for i in 0..A_SIZE {
        let _a = a[i];
        println!("{_a}")
    } // next i

    let val: i32 = another_function(A_SIZE as i32);
    println!("Value: {val}");

    if val == 25 {
        println!("Condition met.");
    }

    let condition: bool = true;
    let num = if condition {val} else {0};
    println!("num: {num} ");

    // Returning values from loops
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 { break counter * 2; }
    };
    println!("The result is {result}");

} // main

fn another_function(x: i32) -> i32 {
    println!("Another function: {x}");
    return x * 5;
}
