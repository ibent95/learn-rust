use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    print_hello_world();


	loop {
		let s_n: i32 = generate_secret_number();

		let g_n: i32 = match guess_number_in_string().trim().parse() {
	        Ok(num) => num,
	        Err(_) => continue,
	    };

		let result_of_compare: i32 = compare_guess_to_secret_number(s_n, g_n);

		if result_of_compare == 1 {
    		println!("You win!");
    		break;
		} else if result_of_compare == 2 {
    		println!("Too big!")
		} else {
    		println!("Too small!");
		}
	}
}

fn print_hello_world() {
    println!("Hello, world! By @ibent95...");
}

fn guess_number_in_string() -> String {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

	let _guess_number: i32 = guess
		.trim()
		.parse()
        .expect("Wanted a number");

    println!("You guessed: {guess}");
    
    return guess;
}

fn generate_secret_number() -> i32 {    
	let secret_number: i32 = rand::thread_rng().gen_range(1..=100);
	let secret_string: String = secret_number.to_string();

    println!("The secret number is: {secret_string}");
    
    return secret_number;
}

fn compare_guess_to_secret_number(secret_number: i32, guess_number: i32) -> i32 {
	let result: i32 = match guess_number.cmp(&secret_number) {
        Ordering::Less => 0,
        Ordering::Greater => 2,
        Ordering::Equal => 1,
    };

    return result;
}