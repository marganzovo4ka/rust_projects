use std::io;
use colored::*;


pub fn input () -> i32 {
    loop{
    let mut x = String::new();

    io::stdin().read_line(  &mut x).expect("failed to read line");

    let a: i32 = match x.trim().parse() {
        Ok(num) => num,
        Err(_)=> continue,
    };
    return a
    }   
}

pub fn simple_numbers() {
    let mut vec: Vec<i32> = Vec::new();
    println!("введите число, до которого выписывать простые числа");
    let n = input();
    if n < 2{
        println!("тут нет простых чисел");
    }
    else {
        vec.push(2);
        for i in 3..n+1{
            let mut flag = true;
            for j in &vec{
                if j*j > i{
                    break;
                }
                if i%j == 0{
                    flag = false;
                    break;
                }
            }
            if flag == false{
                continue;    
            }
            else{
                vec.push(i);
            }
        }
    }
 
    if n >= 7{
        print!("2, {}, {}, {}, ", "3".blue(), "5".blue(), "7".blue())
    }
        let mut j = 4;
        while j <= vec.len()- 1{
            if j+1 == vec.len(){
                print!("{}, ", vec[j]);
                break;
            }
            if vec[j+1]-vec[j] == 2 && j+1 != vec.len(){
                print!("{}, {}, ", vec[j].to_string().blue(), vec[j+1].to_string().blue());
                j += 2;
            }
            else{
                print!("{}, ", vec[j]);
                j+=1;
            }
        }
}
