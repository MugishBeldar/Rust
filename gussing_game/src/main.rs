use::rand::Rng;
fn main() {
    let winning_number = rand::thread_rng().gen_range(1..=100);
    println!("winning number is {}", winning_number);
    loop {
        println!("Please Guess The Number");
        let mut number = String::new();
        let _b1 = std::io::stdin().read_line(&mut number).unwrap();
        let number: i32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if number == winning_number {
            print!("You are win!");
            break;

        } else if number >= winning_number {
            println!("Too large number!");
        } else {
            println!("Too small!");
        }
    }
}
