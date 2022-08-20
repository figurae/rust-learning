use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if !(1..=100).contains(&value) {
            panic!("number from 1 to 100 pls, not {}", value)
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    let answer: i32 = rand::thread_rng().gen_range(1..101);

    loop {
        println!("gimme number!");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("bad number");

        let guess: Guess = match guess.trim().parse() {
            Ok(num) => Guess::new(num),
            Err(_) => continue,
        };

        println!("ur number: {}", guess.value);

        match guess.value.cmp(&answer) {
            Ordering::Less => println!("number too smol"),
            Ordering::Greater => println!("number too big"),
            Ordering::Equal => {
                println!("number fine");
                break;
            }
        }
    }
}
