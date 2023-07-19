use std::io;
use rand::Rng;
use std::collections::HashMap;

fn input () -> usize {

    loop{
    let mut x = String::new();

    io::stdin().read_line(  &mut x).expect("failed to read line");

    let a: usize = match x.trim().parse() {
        Ok(num) => num,
        Err(_)=> continue,
    };
    return a
    }   
}

fn main() {
    let mut v : Vec<i32> = Vec::new();
    println!("count of num in the spis");
    let n:usize = input();
    let mut i = n;
    while i > 0{
    let value = rand::thread_rng().gen_range(1..=100);
    v.push(value);
    i -= 1;
    }
    for  i in &v{
        println!("{i}")
    }

    let values = calculate(&mut v);

    println!("sorted");
    for  i in &v{
        println!("{i}")
    }

    println!("average value = {} mediana = {} mode of list = {}" , values.0, values.1, values.2)
}


fn calculate(v: &mut Vec<i32>) -> ( f32, i32, i32){
    let mut aver = 0;
    for i in &mut *v{
        aver += *i;
    }
    let aver = aver as f32/ v.len() as f32;
    for i in 0..v.len(){
        let mut min: i32= 100000;
        let mut min_index: usize = 0;
        for j in 0 + i..v.len(){
            if v[j] < min{
                min = v[j];
                min_index = j;
            }
        }
        let buf = v[i];
        v[i] = min;
        v[min_index] = buf;
    }

    let mut map = HashMap::new();
    
    for value in 0..v.len(){
        let count = map.entry(v[value]).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
    
    let mut max = (0,0);

    for (key, value) in &map {
        if max.1 < *value{
            max.1 = *value;
            max.0 = *key;
        }
    }

    return (aver, v[(v.len())/2], max.0);
}