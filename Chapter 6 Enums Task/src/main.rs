// // TASK # 1

enum Vehicle{
    bike(i32,String),
    trucks(i32,String),
    cars(i32,String)
}

fn main(){

    let my_bike = Vehicle::bike(423,String::from("Honda"));

    let my_car = Vehicle::cars(523,String::from("Civic"));

    let my_truck = Vehicle::trucks(289,String::from("Toyota"));

    let a: Option<Vehicle> = Some(Vehicle::cars(234, String::from("audi")))
}

// TASK # 2

// enum Department{
//     civil{Student:i32,Classes:i32},
//     computer{Student:i32,Classes:i32},
//     electronic{Student:i32,Classes:i32}

// }

// fn main(){

//     let dept1 = Department::civil{Student:240,Classes:8};

//     let dept2 = Department::electronic{Student:1,Classes:6};
    
//     let dept3 = Department::computer{Student:150,Classes:7};
    

// }
