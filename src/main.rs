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

fn matching() {
    let test_number = 15;

    match test_number {
        1 => println!("One!"),
        2 | 4 | 6 | 8 | 10 => println!("Even number"),
        13..=19 => println!("A teen"),
        _ => println!("a regular number"),
    }
}

fn block_testing() {
    // Variables bindings have a scope

    // This variable has a scope of the function
    let first_var = 5;
    println!("The first variable is {}", first_var);

    // This is a block
    {
        // This variable has a scope of this block
        // Variables can have the same name as others in the function, this is called shadowing
        let first_var = 20;
        println!("The first variable is {}", first_var);
    }
    
    // The first variable is still the same
    println!("The first variable is {}", first_var);

    // Mutable variables can be changed in embedded blocks
    let mut second_var = 7;
    println!("The second variable is {}", second_var);
    {
        second_var = 20;
        println!("The second variable is {}", second_var);
    }
    println!("The second variable is {}", second_var);

    // Frozen data
    // this variable is mutable now
    let mut mut_data_one = 7;
    mut_data_one = 10;
    {
        // once it is shadowed the variable becomes frozen and will remain frozen untill the end of the block (when the variable goes out of scope)
        let mut_data_one = mut_data_one;
    }
    mut_data_one = 15;

}

fn main() {
    block_testing();
}
