use simple_numbers::input;

mod task8_1;
mod task8_2;
mod task8_3;
mod guessing_game;
mod div_voltage;
mod simple_numbers;


fn main() {
    loop{
        println!("\n tasks: \n1 - guessing game(1-2)\n2 - div_valtage_calculator(3-4)\n3 - task8_1\n4 - task8_2\n5 - task8_3\n6 - simple_numbers(1-8)\n7 - exit");
    
        let n = input();
        match n{
            1=> guessing_game::guessing_game(),
            2=> div_voltage::div_voltage(),
            3=> task8_1::task8_1(),
            4=> task8_2::task8_2(),
            5=> task8_3::task8_3(),
            6=> simple_numbers::simple_numbers(),
            7=> break,
            _ =>println!("this number is no here"),
        }
    }
}
