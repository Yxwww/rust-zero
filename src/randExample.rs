use std::io;
use rand::Rng;

fn guessRandom() {
    println!("Guess the number !");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("the secret number is : {}", secret_number);

    println!("please input your guess:");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println("You guessed {}", guess);
}
