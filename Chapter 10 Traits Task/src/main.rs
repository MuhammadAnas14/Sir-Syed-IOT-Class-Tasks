//////////////////////////TASK#1//////////////////////////

#[derive(Debug)]
struct RigthAngleTrianle<T> {
    base :T,
    perpendicular:T
}

impl RigthAngleTrianle<i32> {
    fn hypotenuse(&self) -> i32 {
        (self.base.pow(2) + self.perpendicular.pow(2))
    }
}

fn main() {

    let triangle1 = RigthAngleTrianle{
        base: 5,
        perpendicular:4
    };
    println!("The hypotenuse is {}",triangle1.hypotenuse());
}

//////////////////////////TASK#2////////////////////
#[derive(Debug)]
pub struct Books {
    pub author: String,
    pub publisher: String,
    
}

impl Books {
    fn NewBook(auth: String,write:String) -> Books {
        Books { author: auth, publisher: write }
    }
}

pub trait BookInformation {
    fn Info(&self) -> String;
    
}

impl BookInformation for Books{
    fn Info(&self) -> String {
        format!("Written by {} and publish by{}", self.author, self.publisher)
    }
}

pub fn Publishdate(ondate:impl BookInformation){
    println!("{} on date",ondate.Info())
    
}


fn main(){
    let book1 = Books::NewBook(String::from("John Wick"), String::from("Watson"));
    println!("{}",book1.Info());
    Publishdate(book1);

}


