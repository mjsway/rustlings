// variables5.rs
//
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a
// hint.


// This is an example of shadowing (https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing)

fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    let number = 3; // don't rename this variable
    println!("Number plus two is : {} which means number is {}", number + 2, number);
    {
        let number = number * number;
        println!("Number in the inner block scope is : {}", number);
    }
    println!("Number plus two is : {}, which means number in the outer scope is once again : {}", number + 2, number)
}
