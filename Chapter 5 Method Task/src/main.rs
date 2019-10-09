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






















