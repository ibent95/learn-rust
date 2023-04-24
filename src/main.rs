use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    print_hello_world();
    print_variable_data_type();

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

fn print_variable_data_type() {
	let text: &str = "Text";
	let character: char = 'Z';
	let heart_eyed_cat: char = '😻';
	let int: i32 = 123;
	let float: f32 = 3.0;
	let boolean: bool = true;
	
	let tuple: (i32, f64, u8) = (500, 6.4, 1);
	let (t_x, t_y, t_z) = tuple;
	let five_hundred = tuple.0;
    let six_point_four = tuple.1;
    let one = tuple.2;

    let months: [&str; 12] = [
    	"January", "February", "March", "April", "May", "June", "July",
    	"August", "September", "October", "November", "December"
	];
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
	let first: i32 = numbers[0];
    let second: i32 = numbers[1];

    println!("Text = {}", text);
    println!("Character = {}", character);
    println!("Heart eyed cat = {}", heart_eyed_cat);
    println!("Integer Signed 32 = {}", int);
    println!("Float Signed 32 = {}", float);
    println!("Boolean = {}", boolean);

    // println!("Tuple = {}", tuple);
    println!("t_x = {}", t_x);
    println!("t_y = {}", t_y);
    println!("t_z = {}", t_z);
    println!("Five hundred = {}", five_hundred);
    println!("Six point four = {}", six_point_four);
    println!("One = {}", one);

    // println!("Months = {}", months);
    // println!("Numbers = {}", numbers);
    println!("First = {}", first);
    println!("Second = {}", second);
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