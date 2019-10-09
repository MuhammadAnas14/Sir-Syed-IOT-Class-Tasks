// struct Entities{
//     Number1:f64,
//     Number2:f64,
//     Operation:String,
// }


// fn add(calculate:&Entities)->f64{
//     calculate.Number2 + calculate.Number2
// }
// fn sub(calculate:&Entities)->f64{
//     calculate.Number1 - calculate.Number2
// }
// fn Mult(calculate:&Entities)->f64{
//     calculate.Number1 * calculate.Number2
// }
// fn div(calculate:&Entities)->f64{
//     calculate.Number1 / calculate.Number2
// }

// fn main() {
//     let Event1 = Entities{Number1:34.0,Number2:32.0,Operation:String::from("Addition")};

//     if Event1.Operation == "Addition"{
//         let result = add(&Event1);
//         println!("The result is {}",result);
//     }


//     else if Event1.Operation == "Subtraction"{
//         let result = sub(&Event1);
//         println!("The result is {}",result);
//     }

//     else if Event1.Operation == "Multiplication"{
//         let result = Mult(&Event1);
//         println!("The result is {}",result);
//     }
    
//     else if Event1.Operation == "Division"{
//         let result = div(&Event1);
//         println!("The result is {}",result);
//     }
//     else{
//         println!("Invalid input");
//     }
// }


// Task#1
#[derive(Debug)]
struct Marks {
    math : f64,
    science : f64,
    english : f64,
    pakistan_studies : f64,
    urdu : f64

}

impl Marks {

    fn total_marks(&self)->f64{
        self.math + self.science + self.english + self.pakistan_studies + self.urdu
    }

    fn percentage(&self)->f64{
        let total_marks = self.math + self.science + self.english + self.pakistan_studies + self.urdu;
        let max_marks:f64 = 500.0;
        (total_marks /  max_marks) * 100.0
    }
}

fn main(){

    let student1 = Marks{
        math:87.5,
        science:90.0,
        english:79.0,
        pakistan_studies:74.0,
        urdu:45.0};

        
    println!("The total marks is {}",student1.total_marks());

    println!("The Percentage is {}%",student1.percentage());

}






















