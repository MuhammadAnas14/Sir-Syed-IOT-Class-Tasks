// TASK#1
struct Employee {
    employee_no: u32,
    name: String,
    email: String,
    gender: String,
    phone_no: u64,
    active:bool
}

fn main() {

    let employee1 = Employee{
        employee_no: 1067,
        name: String::from("anas"),
        email: String::from("anas@gmail.com"),
        gender: String::from("male"),
        phone_no: 03226532810,
        active:true
    };

    let employee2 = Employee{
        employee_no: 1743,
        name: String::from("ali"),
        email: String::from("ali23@gmail.com"),
        phone_no: 03225432844,
        ..employee1

    };

    println!("The name of employee1 is :{} and phone_no of employee1 is {}",employee1.name,employee1.phone_no);
    println!("the email of employee2 is :{} and gender of employee1 is {}",employee2.employee_no,employee2.gender)
    
}

//TASK2
#[derive(Debug)]
struct Student {
    Name: String,
    Father_name: String,
    Class: u64,
    Grade: String
}

fn main(){

    let student1 = add_student(String::from("mustafa"),String::from("Ahmed"),12,String::from("A"));

    println!("The name of student is :{}",student1.Name);
    println!("The class of the student :{}",student1.Class);
    println!("{:#?}",student1)
}

fn add_student(name:String,father_name:String,class:u64,grade:String)->Student{
    Student {
        Name:name,
        Father_name:father_name,
        Class:class,
        Grade:grade
    }
}

//TASK#3

struct Triangle{
    Base : f64,
    Height: f64,
    Type_of_triangle: String,
}

fn main(){
    
    let triangle1 = Triangle{Base:8.0,Height:8.0,Type_of_triangle:String::from("right angle triangle")};

    let area = area_of_triangle(&triangle1);

    println!("Type of triangle is: {}",triangle1.Type_of_triangle);
    println!("area is : {}",area);

}

fn area_of_triangle(area:&Triangle)->f64{
    0.5 * area.Height * area.Base 
}
