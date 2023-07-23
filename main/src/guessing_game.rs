use std::io;
use rand::Rng;
pub fn guessing_game() {

    let secret_num = rand::thread_rng().gen_range(1..=100);

    loop{
    println!("input the number");
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("failed to read line");
    let num:u32 = match num.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
    };

    if num == secret_num{
        println!("you win!");
        break;
    }
    else if num < secret_num{
        println!("too small");
    }
    else{
        println!("too big");
    }
}
}
