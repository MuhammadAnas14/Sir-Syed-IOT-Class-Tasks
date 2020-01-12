pub struct Entities{
    pub number1:f64,
    pub number2:f64,
    pub operation:String,
}


pub fn add(calculate:&Entities)->f64{
    calculate.number2 + calculate.number2
}

pub fn sub(calculate:&Entities)->f64{
    calculate.number1 - calculate.number2
}
pub fn mult(calculate:&Entities)->f64{
    calculate.number1 * calculate.number2
}
pub fn div(calculate:&Entities)->f64{
    calculate.number1 / calculate.number2
}

fn main() {
    let event1 = Entities{number1:34.0,number2:32.0,operation:String::from("Addition")};

    if event1.operation == "Addition"{
        let result = add(&event1);
        println!("The result is {}",result);
    }


    else if event1.operation == "Subtraction"{
        let result = sub(&event1);
        println!("The result is {}",result);
    }

    else if event1.operation == "Multiplication"{
        let result = mult(&event1);
        println!("The result is {}",result);
    }
    
    else if event1.operation == "Division"{
        let result = div(&event1);
        println!("The result is {}",result);
    }
    else{
        println!("Invalid input");
    }
}


pub mod calculator{

    pub mod basic_function{

        pub fn add(x:i32,y:i32) -> i32{
            x+y
        }

        pub fn subtract(x:i32,y:i32) -> i32{
            x-y
        }

        pub fn multiply(x:i32,y:i32) -> i32{
            x*y
        }

        pub fn divide(x:f64,y:f64) -> f64{
            x / y
        }
    }

    pub mod power_function{

        pub fn square(x:i32) ->i32{
            x * x
        }
        pub fn cube(x:i32)->i32{
            x * x * x
        }
        pub fn power(x:i32,n:i32)->i32{
            let mut result:i32 = 4;
            for i in (0..n){
                let result = result * x;
            }
            result
        }
    } 
}

// use crate::calculator::basic_function;

// fn main(){
//     let y = basic_function::add(4,3);
//     println!("{}",y)

