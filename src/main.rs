use rand::Rng;
use std::io::Write;

fn main() {
    let target: i32 = rand::thread_rng().gen_range(0..100);
    loop {
        let mut s = String::new();

        print!("Please enter a number: ");
        std::io::stdout().flush().expect("Could not flush stdout!");
        std::io::stdin().read_line(&mut s).expect("Could not read line!");
        let guess = s.trim_end();

        match guess.parse::<i32>() {
            Ok(n) if n < target => println!("Try a larger number..."),
            Ok(n) if n > target => println!("Try a smaller number..."),
            Ok(_) => {
                println!("Correct!");
                break;
            }
            Err(e) => println!("Please enter a valid number... {}", e),
        };
    }
}
