use std::io;
use std::collections::HashMap;

fn input () -> i32 {

    loop{
    let mut x = String::new();

    io::stdin().read_line(  &mut x).expect("failed to read line");

    let a: i32 = match x.trim().parse() {
        Ok(num) => num,
        Err(_)=> {
            println!("enter rhe number"); 
            continue;
        }
    };
    return a
    }   
}

fn main() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();
    loop{
        println!("\n1 - Add worker\n2 - check workers\n3 - Exit");
        let choose = input();
        match choose{
            1 => add(&mut departments),
            2 => view(&mut departments),
            3 => break,
            _ => {println!("this number is no here"); continue;}
        };
    }
}

fn add(departments: &mut HashMap<String, Vec<String>>){
    println!("enter the name of worker");

    let mut person = String::new();
    io::stdin().read_line(  &mut person).expect("failed to read line");
    let person: String = (person.trim()).to_string();

    println!("enter the name of department");

    let mut dep = String::new();
    io::stdin().read_line(  &mut dep).expect("failed to read line");
    let dep = (dep.trim()).to_string();

    let mut _vect:Vec<String> = Vec::new();

    let count = departments.entry(dep).or_insert(_vect);
    count.push(person);

}

fn view(departments: &HashMap<String, Vec<String>>){
    let mut vec: Vec<String> = departments.clone().into_keys().collect();
    vec.sort_unstable();
    for dep in vec
    {    
        println!("{}: {:?}", dep, departments[&dep]);
    }
}