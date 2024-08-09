use rand::Rng; 
use std::io; // Import the Rng trait for random number generation

fn main() {
    loop {
        let fruits: [&str; 5] = ["apple", "banana", "mango", "orange", "guava"];
        fruit_printer(fruits);
        
        let random_num = select_random_fruit(fruits.len());
        println!("computer slected text {}",random_num);
        let user_number = match take_user_input() {
            Some(num) if num > 0 && num <= fruits.len() as i8 => num - 1,
            _ => {
                println!("Sorry, something went wrong or the number is out of range. Try again.");
                continue;
            }
        };
        
        check_result(user_number, random_num, fruits);
    }
}

fn check_result(user_number: i8, computer_num: i8, fruits: [&str; 5]) {
    if user_number == computer_num {
        println!("🎉🎉 *** CONGRATULATIONS *** 🎉🎉");
        println!("🥳 You have WON the game! 🎉");
        println!("🍎 You guessed:  {}", fruits[user_number as usize]);
        println!("🤖 Computer guessed: {}", fruits[computer_num as usize]);

        println!();
        println!("╭────────────────────────────────╮");
        println!("│                                │");
        println!("│     🎊 You did it! 🎊           │");
        println!("│                                │");
        println!("│    You correctly guessed the    │");
        println!("│        fruit this time! 🍍     │");
        println!("│                                │");
        println!("╰────────────────────────────────╯");

        println!();
        println!("✨ Here's a virtual trophy for you! 🏆");
        println!("───────────────");
        println!("|   🏆  🏅   |");
        println!("|   🥇  🎖️  |");
        println!("───────────────");
    } else {
        println!("🎉🌟 *** GAME OVER *** 🌟🎉");
        println!("😔 Oh no, you didn't win this time!");
        println!("🍏 You guessed:  {}", fruits[user_number as usize]);
        println!("🤖 Computer guessed: {}", fruits[computer_num as usize]);

        println!();
        println!("╭────────────────────────────────╮");
        println!("│                                │");
        println!("│          💪 Better Luck        │");
        println!("│         Next Time! ✨           │");
        println!("│                                │");
        println!("╰────────────────────────────────╯");

        println!();
        println!("Here's a consolation prize for you! 🎁");
        println!("───────────");
        println!("|   🍎   |");
        println!("|   🍌   |");
        println!("|   🍇   |");
        println!("───────────");
    }
}

fn take_user_input() -> Option<i8> {
    let mut user_input = String::new();
    println!("Please enter your guessed number (1 to 5): ");
    io::stdin().read_line(&mut user_input).expect("Sorry, can't read your data");
    
    match user_input.trim().parse::<i8>() {
        Ok(num) => Some(num),
        Err(e) => {
            println!("Invalid input: {}", e); 
            None
        }
    }
}

fn fruit_printer(fruits: [&str; 5]) {
    for (index, element) in fruits.iter().enumerate() {
        println!("{}.  {}", index + 1, element);
    }
}

fn select_random_fruit(limit: usize) -> i8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..limit) as i8
}
