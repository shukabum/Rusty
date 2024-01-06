use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    println!("Guess the number game");
    let secretnum = rand::thread_rng().gen_range(1..101);
    println!("the secret number:{}",secretnum);
    loop{
        println!("enter the number");
        let mut guess = String :: new();
        io::stdin().read_line(& mut guess)
        .expect("failed to read");
        let guess: u32= match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>continue
        };
        println!("your guess: {}",guess);

        match guess.cmp(&secretnum){
            Ordering::Less=> println!("{}","Too small".red()),
            Ordering::Equal=> {
                println!("{}","You Win".green());
                break;
            },
            Ordering::Greater=> println!("{}","Too Large".red()),
        }

    }
}
