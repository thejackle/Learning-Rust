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

fn printing_things() {
    // Integer math
    println!("5 + 7 = {}", 5 + 7);
    println!("22 - 70 = {}", 22 - 70);

    // Types can be used to define the output
    println!("5 + 7 = {}", 5i32 + 7);
    println!("22 - 70 = {}", 22 - 70i32);

    // boolean logic can also be performed
    println!("true and true = {}", true && true);

    // underscores can be used for improved readability
    println!("One million dollars, {}$", 1_000_000);

}

fn main() {
    printing_things();
}
