use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    loop {
        play_game();

        println!("Do you want to play again? (yes/no)");

        let mut play_again = String::new();

        io::stdin()
            .read_line(&mut play_again)
            .expect("Failed to read line");

        let mut _play_again = play_again.trim().to_lowercase();


        match _play_again.as_str() {
            "yes" | "y" => println!("Have Fun!!"),
            "no" | "n" => {
                println!("Good Bye~");
                break;
            },
            _ => {
                println!("Invalid Input!!");
                println!("Do you want to play again? (yes/no)");

                play_again.clear();

                io::stdin()
                    .read_line(&mut play_again)
                    .expect("Failed to read line");
            },
        }
    }
}


fn play_game() {
    println!("Number Guessing Game!!");

    println!("Select Difficulty!!");

    println!("1. Easy  2. Medium  3. Hard  4. Impossible ");

    let mut level: u32;

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        level = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter it in the correct format!!");
                continue;
            }
        };

        match level {
            1 => break,
            2 => break,
            3 => break,
            4 => break,
            _ => {
                println!("Please enter a valid number between 1 and 4");
                continue;
            },
        };
    }

    let mut level = match level {
        1 => 100,
        2 => 20,
        3 => 7,
        4 => 1,
        _ => {
            panic!("Invalid Level!!");
        },
    };

    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    while level > 0 {

        println!("Pleae input your guess.");
        println!("If you want to leave this game, type quit or exit");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = guess.to_lowercase();

        if guess.trim() == "quit" || guess.trim() == "exit" {
            println!("Quitting the game... ");
            break;
        }
        let guess: Result<u32, _> = guess.trim().parse();

        let guess = match guess {
            Ok(num) => num,
            Err(_) => {
                println!("Please input number or type 'quit' or 'exit' to leave the game!");
                continue;
            },
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Congratulation!! You win!");
                break;
            }
        }

        level -= 1;
        if level == 0 {
                println!("Fail");
        }
    }
}
