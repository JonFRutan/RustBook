use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("GUESS THE NUMBER!");
	let secret_number = rand::thread_rng().gen_range(1..=100); //rand library, we use thread_rng for a local random number generation seeded by the OS. gen_range(1..=100) is how we define the range between 1 and 100.
	//note: could you obtain a similar range by just applying modulo 100 against any output
	//println!("SECRET NUMBER: {}", secret_number);

	loop {
    	println!("PLEASE INPUT YOUR GUESS: ");
    	let mut player_guess = String::new();
    	io::stdin()						     //from the io standard library we call stdin to read input from the console
    		.read_line(&mut player_guess)    //read_line, a function of io, will take input from the console and update the referenced mutable
    		.expect("Failed to read line!"); //read_line returns an enum called `Result`. `Result` can be `Ok` or `Error`. .expect catches the error, crashes, and prints the line if this happens.

	  //let player_guess: u32 = player_guess.trim().parse().expect("Please Type A Number!");
		let player_guess: u32 = match player_guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};
		
    	println!("You Guessed: {player_guess}");

		//match expressions end after the first match found, so subsequent arms will not be executed or checked
		match player_guess.cmp(&secret_number) {
			Ordering::Less => println!("Too small."),
			Ordering::Greater => println!("Too large."),
			Ordering::Equal => {
				println!("You win!"); 
				break;
			}
		}
	}
}
