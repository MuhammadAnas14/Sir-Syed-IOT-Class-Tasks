struct Entities{
    Number1:f64,
    Number2:f64,
    Operation:String,
}


fn add(calculate:&Entities)->f64{
    calculate.Number2 + calculate.Number2
}
fn sub(calculate:&Entities)->f64{
    calculate.Number1 - calculate.Number2
}
fn Mult(calculate:&Entities)->f64{
    calculate.Number1 * calculate.Number2
}
fn div(calculate:&Entities)->f64{
    calculate.Number1 / calculate.Number2
}

fn main() {
    let Event1 = Entities{Number1:34.0,Number2:32.0,Operation:String::from("Addition")};

    if Event1.Operation == "Addition"{
        let result = add(&Event1);
        println!("The result is {}",result);
    }


    else if Event1.Operation == "Subtraction"{
        let result = sub(&Event1);
        println!("The result is {}",result);
    }

    else if Event1.Operation == "Multiplication"{
        let result = Mult(&Event1);
        println!("The result is {}",result);
    }
    
    else if Event1.Operation == "Division"{
        let result = div(&Event1);
        println!("The result is {}",result);
    }
    else{
        println!("Invalid input");
    }
}
