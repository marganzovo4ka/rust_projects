use std::io;
fn main() {
    println!("input num of fibonachi");

let mut n = String::new();

io::stdin()
    .read_line(&mut n)
    .expect("Failed to read line");

let n: i32 = n.trim().parse().expect("Please type a number!");
let mut equal: i32 = 1;
let mut prev: i32 = 1;
if n == 1 || n == 2{
println!("fibonacci number - {equal}")
}
else{
    for _var in 2..n{
        let buf: i32  = equal;
        equal = equal + prev;
        prev = buf;
    }
    println!("fibonacci number - {equal}");
}
}
