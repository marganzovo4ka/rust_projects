use std::io;
fn main() {
    println!("choose what to convert \n 1-Celsius: default \n 2-fahrenheit");

let mut buf = String::new();

io::stdin()
    .read_line(&mut buf)
    .expect("Failed to read line");

let buf: i32 = buf.trim().parse().expect("Please type a number!");
if buf == 2{
    println!("input temperature in fahrenheit");
    let mut f = String::new();

    io::stdin()
    .read_line(&mut f)
    .expect("Failed to read line");

    let f: f32 = f.trim().parse().expect("Please type a number!");

    let c: f32 = (f-32.0)/1.8;
    println!("temperature in celsium = {c}");
}
else{
    println!("input temperature in Celsius");
    let mut c = String::new();

    io::stdin()
    .read_line(&mut c)
    .expect("Failed to read line");

    let c: f32 = c.trim().parse().expect("Please type a number!");

    let f: f32 = (c+32.0)*1.8;
    println!("temperature in forenheit = {f}");
}
}
