use rand::Rng;
use std::cmp::Ordering;
use std::io;
use colored::*;

fn main() {
    println!("Adivinhe o número!");

    let secret_number = rand::thread_rng().gen_range(1, 10);

    loop {
        println!("Por favor, digite seu palpite:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Por favor digite um número!");
                continue;
            }
        };

        println!("Você chutou: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Abaixo!".red()),
            Ordering::Greater => println!("{}", "Acima!".red()),
            Ordering::Equal => {
                println!("{}", "Acertou!".green());
                break;
            }
        }
    }
}
