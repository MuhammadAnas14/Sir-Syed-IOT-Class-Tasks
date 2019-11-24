use cal::calculator::basic_function;
use cal::calculator::power_function;



fn main(){
    let y = basic_function::add(4,3);
    println!("{}",y);
    let z = power_function::square(3);
    println!("{}",z);
}