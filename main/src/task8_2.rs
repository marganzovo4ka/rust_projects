use std::io;
pub fn task8_2() {
    let mut x = String::new();

    println!("enter the sentence");

    io::stdin().read_line(  &mut x).expect("failed to read line");

    let arr_of_let = ["e","y","u","i","o","a","E","Y","U","I","O","A"];
    

    let mut pig_latin = String::new();  

    for word in x.split_whitespace(){
        let s = &word[0..1];
        if pig_latin.is_empty(){
            if arr_of_let.contains(&s){
                pig_latin = format!("{pig_latin}{word}-hay");
            }
            else{
                pig_latin = format!("{pig_latin}{}-{s}ay", &word[1..word.len()]);
            }
        }
        else{
            if arr_of_let.contains(&s){
                pig_latin = format!("{pig_latin} {word}-hay");
            }
            else{
                pig_latin = format!("{pig_latin} {}-{s}ay", &word[1..word.len()]);
            }
        }
    }  
    println!("{pig_latin}");
}
