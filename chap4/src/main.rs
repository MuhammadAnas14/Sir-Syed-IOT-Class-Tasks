// TASK#1

fn main() {
    let mut s = String::from("PAKISTAN");

    add_string(& mut s)

}
fn add_string(new: & mut String){
    new.push_str(" ZINDABAD");
    println!("{}",new);
}

//TASK#2

use std::io;
fn main() {

    println!("Enter any string");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("invalid input");
    
    let length = len_string(&input);

    println!("The lenght of string is {}",length);
    // println!("{}",input)

}
fn len_string(s:&String)->usize{
    s.trim().len()
}


