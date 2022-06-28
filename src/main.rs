use std::io;


fn number_guesser() {

    // a number guessing game
    println!("Guess the number!");
    println!("Input your guess now.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Could not get answer");

    println!("You guessed: {}", guess)

    // TODO create random number to compare to
}

fn primatives(){
    // lets look at some variables
    // Variables are declared with let
    let _new_variable: i32;

    // Variables are immutaqble by default but can be changed to mutable with mut
    let mut _new_mutable_var = 22;
    _new_mutable_var = 50;

    // bool variables
    let _new_bool = true;
    // with annotation
    let _new_bool_two: bool = false;

}

fn main() {
    primatives();
}
