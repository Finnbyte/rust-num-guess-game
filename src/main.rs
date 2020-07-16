use rand::prelude::*;
use std::io;
use std::cmp::Ordering;

fn generate_number() -> u32 { //generates the random number
    let mut rng = thread_rng();

    let min = 1; //min value for random number
    let max = 20; //max value for random number

    let rand_num = rng.gen_range(min, max);
    return rand_num;
}

fn main() {
    let random = generate_number();
    let mut input = String::new();
    let attempts = 5; //set attempt amount

    for tries in (0..6)  {
        let mut input = String::new();

        if tries == 5 {
            println!("\nYou lost");
            break;
        }

        io::stdin() 
            .read_line(&mut input)
            .expect("Cannot read input!");
        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => panic!("error parsing input!"),
        };

        println!("Your guess was {}", input);

        match input.cmp(&random) {
            Ordering::Less => println!("Your guess was smaller than the number!"),
            Ordering::Greater => println!("Your guess was bigger than the number!"),
            Ordering::Equal => {
                println!("\nYou won :)");
                break;
            }
        }
    }
}
