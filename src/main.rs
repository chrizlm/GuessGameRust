use std::io;
use std::cmp::Ordering;
use rand::Rng;



fn main(){
    println!("Guessing game rust!");

    let expected_guess = rand::thread_rng().gen_range(1,101);

    //println!("Expected guess {}", expected_guess);

    loop{

    println!("Enter your guess(number):");

    let mut guess = String::new();


    io::stdin().read_line(&mut guess).expect("Cannot read input");

    let guess: u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
    };


    println!("Your guess is {}", guess);

    match guess.cmp(&expected_guess){
        Ordering::Less=> println!("Value guessed is less"),
        Ordering::Greater=> println!("Value guessed is greater"),
        Ordering::Equal=>{ 
            println!("Value guessed is correct");
            break;

    }
    }

}
}
































/* use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main(){
    println!("This is a guessing game!");

    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("The secret number is: {}", secret_number);
    println!("Enter a number you guessed:");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Cannot read input value");

    let guess: u32 = guess.trim().parse().expect("Please type number!");

    println!("You guessed {}, right?", guess);

    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("You win"),
    }
}
 */









































































/* use std::io;

fn main() {
    println!("Guess number!");
    println!("Please enter number: ");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("Your guess: {}", guess);
}
 */