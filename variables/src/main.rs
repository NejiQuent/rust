//const THREE_HOURS_IN_SECONDS: u32 = 3600 * 3;

// fn main() {
//    let mut x = 5;
//    println!("The value of x is: {}", x);
//    x = 6;
//    println!("The value of x is: {}", x);
// }

fn main() {
    /* Variables and mutability */
    let x = 5;
    let x = x + 1;
    {
    let x = x * 2;
    println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);
    let spaces = "   "; //The advantage to shadow the variable is that we can use a new type of variable.
    let spaces = spaces.len(); //For ex, if we had used let mut, spaces.len will do an error because it attend number type and during declaration of let mut spaces... it's declared as a str.
    println!("number of spaces: {} ", spaces);
}