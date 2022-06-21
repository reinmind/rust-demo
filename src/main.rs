use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_num = rand::thread_rng().gen_range(1..101);
    loop {
        println!("input:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        println!("you guessed: {}",guess);
        let guess : u32 = match guess.trim().parse(){
            Ok(num) => num,Err(_) => continue
        };
        match guess.cmp(&secret_num){
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You win secret_num: {}",secret_num);
                break;
            },
        };
    }
}
