use rand::Rng;
use std::io;

// simple number baseball game.
fn main() {
    let mut answer = [0u32; 3];
    print!("Generating number...");

    for i in 0..3 {
        loop {
            let x: u32 = rand::thread_rng().gen_range(0..10);
            if i == 0 && x == 0 {
                continue;
            }
            if !answer.contains(&x) {
                answer[i] = x;
                break;
            }
        }
    }

    println!("done.");
    loop {
        println!("Please input 3-digit number");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                if num < 100 || num > 999 {
                    println!("Invalid number");
                    continue;
                }
                num
            }
            Err(_) => {
                println!("Invalid input");
                continue;
            }
        };

        // split 3 digits
        let guess = [guess / 100, guess / 10 % 10, guess % 10];
        let mut strikes = 0;
        let mut balls = 0;

        for i in 0..3 {
            if guess[i] == answer[i] {
                strikes += 1;
            } else if guess.contains(&answer[i]) {
                balls += 1;
            }
        }

        println!("{strikes} strikes and {balls} balls!");

        if strikes == 3 {
            println!("You win!");
            break;
        }
    }
}
