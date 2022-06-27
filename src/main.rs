use std::io;


fn Number_Guesser() {

    println!("Guess the number!");
    println!("Input your guess now.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Could not get answer");

    println!("You guessed: {}", guess)
}

fn main() {
    Number_Guesser();
}
