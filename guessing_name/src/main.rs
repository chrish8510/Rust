use std::io;
use std::cmp::Ordering; // cmp means comparing guess and secret_number
use rand::Rng; // :: means associating with or part of

fn main() {
    println!("Guess the number!");
	
	let secret_number = rand::thread_rng().gen_range(1, 101); // rand::thread_rng() is method of Rng module
	
	loop {
	    println!("Please input your guess.");
	
	    let mut guess = String::new();
	
	    io::stdin().read_line(&mut guess) // read_line that puts input from user into string and also returns a value. 
	    // & is a reference that you can use it multiple times throughout your code
	    // io::stdin() -> io::Result
	        .expect("Failed to read line"); // io::Result's Enums' variants are Ok or Err(error handling)
			// enumeration is a type that can have a fixed set of values
		
	    let guess: u32 = match guess.trim().parse() { // : means annotate or simply put "tell"
	        Ok(num) => num,
			Err(_) => continue,
		};
		
	    println!("You guessed: {}", guess);
	
	    match guess.cmp(&secret_number) {
		    Ordering::Less => println!("Too small!"),
		    Ordering::Greater => println!("Too big!"),
		    Ordering::Equal => {
				println!("You win!");
				break;
			}
		}
	}
}
