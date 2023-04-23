use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    print_hello_world();

    let g_n: i32 = guest_number();
    
    let s_n: i32 = generate_secret_number();

    compare_guest_to_secret_number(s_n, g_n);
}

fn print_hello_world() {
    println!("Hello, world!");
}

fn guest_number() -> i32 {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

	let guess_number: i32 = guess
        .trim()
        .parse()
        .expect("Wanted a number");

    println!("You guessed: {guess}");
    
    return guess_number;
}

fn generate_secret_number() -> i32 {    
	let secret_number: i32 = rand::thread_rng().gen_range(1..=100);
	let secret_string: String = secret_number.to_string();

    println!("The secret number is: {secret_string}");
    
    return secret_number;
}

fn compare_guest_to_secret_number(secret_number: i32, guess_number: i32) {
    match guess_number.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}