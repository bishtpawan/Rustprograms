use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
	let random_num = rand::thread_rng().gen_range(1,101);
	println!("Random number is {}",random_num);
	
	loop{
		let mut guess =String::new();
		
		println!("Guess any number between 1 to 100");

		io::stdin().read_line(&mut guess).expect("Unable to read");

		println!("You guess {}",guess);
	
		let guess:u32 = match guess.trim().parse(){
					Ok(num) => num,
					Err(_) => continue,
					};

match guess.cmp(&random_num){
		Ordering::Less => println!("Too small"),
		Ordering::Greater => println!("Too big"),
		Ordering::Equal => {
					println!("You win");
					break;
					}
		}
	}
}
