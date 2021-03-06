use std::io;
use std::cmp::Ordering;
use rand::Rng;
use std::fmt;


struct User {
    name: String,
    email: String,
    age: String,
}

impl fmt::Display for User{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "User details: name {}, email {}, age{}", self.name, self.email, self.age)
    }
}

fn player_detail_response(name: String , email: String, age: String) {
   let mut user1 = User{
        name,
        email,
        age,
    };

    println!("{}", user1);
}

fn player_details_acquisition(){

    //enter name and email and age
    println!("Enter your name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("cannot read input");
    
    println!("Enter your email:");
    let mut email = String::new();
    io::stdin().read_line(&mut email).expect("cannot read input");

    println!("Enter your age:");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Cannot read input");

    //println!("the following are your details name:{}, email:{}, age{}", name, email, age);
    player_detail_response(name, email, age);

}


fn main(){

    let rec = Rectangle{
        width: 30,
        height: 20,
    };

    are_calc(&rec);
    println!("{:#?}", rec);
    println!("Guessing game rust!");

    player_details_acquisition();


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



#[derive(Debug)]
struct Rectangle{
    height: u32,
    width: u32,
}


fn are_calc(rec1: &Rectangle)-> u32{
    rec1.height * rec1.width
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