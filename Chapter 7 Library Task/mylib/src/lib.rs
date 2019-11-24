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

