// import rand from external crate 
extern crate rand;
// use Rng from the external crate rand
// now range is available in the code base
use rand::Rng;
// import io from the std (standard library)
use std::io;

// function main with no input
fn main() {
    // print macros 
    println!("Guess the number between 1 and 100!");
    println!("Please input your guess!");
    // let creates a variable x
    // mut makes it mutable, by default everything is immutable 
    // String::new() returns a new instance of string
    // `Sting` is string type provided by std
    // `::` accesses the static methods on String
    // `new` creates a new instance of String.  
    let mut guess = String::new();
    // io::stdin accesses the stdin function on the io
    // io::stdin() creates a new instance of the stdin
    // is a type that represents the standard input of your terminal
    // read_line is the method wich takes the input and provide to the argument
    // & will indicate that its a reference
    // multiple parts of code can access the same variable 
    // like variables reference  
    io::stdin().read_line(&mut guess)
        .expect("failed to read line!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);
    println!("You guessed: {}", guess);
}

