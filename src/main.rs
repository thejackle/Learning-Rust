use std::io;


fn main() {

    println!("Guess the number!");
    println!("Input your guess now.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess);

    println!("You guessed: {}", guess)
}
