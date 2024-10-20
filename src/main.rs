use rand::thread_rng;
use rand::Rng;
use std::io::stdin;

fn print_main() {
    println!("Welcome to the Number Guessing Game!\nWe have selected a random number between 1 and 100. See if you can guess it in 10 attempts or fewer. We'll let you know if your guess is too high or too low.\n");
}

fn main() {
    print_main();

    let random_number: i32 = thread_rng().gen_range(1..=100);

    let mut guessed_number_vec: Vec<i32> = Vec::new();

    let mut guess: i32 = 0;
    while guess < 10 {
        println!("You have {} guesses left.", 10 - guess);
        println!("Guessed numbers: {:?}", guessed_number_vec);

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");

        let guessed_number = input.trim().parse().unwrap_or(0);

        guessed_number_vec.push(guessed_number);

        // Clear screen
        print!("\x1B[2J\x1B[1;1H");
        print_main();

        if guessed_number == random_number {
            println!("Congratulations! You guessed the correct number.");
            break;
        } else if guessed_number < random_number {
            println!("Your guess is too low.");
        } else {
            println!("Your guess is too high.");
        }

        guess += 1;
    }

    if guess == 10 {
        println!("You've used all 10 attempts.");
    }

    println!("Thank you for playing!");
}
