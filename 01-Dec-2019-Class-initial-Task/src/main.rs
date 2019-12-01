//////////////TASK#1///////////////

// extern crate rand;
// use rand::{thread_rng, Rng};

// fn main() {
//     let mut rng = thread_rng();

//     let mut vec= Vec::new();

//     for i in 0..100{

//         vec.push(rng.gen_range(0.0,900.0));
//     }
//     // println!("{:?}",vec);
//     println!("Total : {}",vec.len());
//     let mut x =0;
//     let mut y =0;
//     let mut z =0;
//     let mut j =0;

//     while j<100{
//         if vec[j] <=300.0{
//             x= x+1;

//         }
//         else if vec[j]<=600.0{
//             y= y+1;

//         }
//         else{
//             z= z+1;
//         }
//         j=j+1;
//     }
    
//     println!("Between 0 to 300: {}",x);
//     println!("Between 300 to 600: {}",y);
//     println!("Between 600 to 900: {}",z);
    
// }

///////////TASK#2//////////////

// extern crate rand;
// use rand::{thread_rng, Rng};

// fn main() {
//     let mut rng = thread_rng();

//     // let num2: u32 = rng.gen_range(0,100);

//     let mut vec= Vec::new();
//     loop{
//         let num2: u32 = rng.gen_range(0,100);

//         if num2>90{
//             break
//         }
//         else{
//             vec.push(num2)
//         }
//     };
//     println!("{:?}",vec);
//     vec.sort();
//     println!("{:?}",vec)

// }


////////TASK#3///////////

use rand::Rng;

fn main(){
    
    loop {
        let one = rand::thread_rng().gen_range(1, 7);

        let two = rand::thread_rng().gen_range(1, 7);

        println!("Roll one {}",one);
        println!("Roll two {}",two);

        let result = one + two;

        println!("result is {}",result);
        break

    }
}

 
