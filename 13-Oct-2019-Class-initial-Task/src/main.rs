struct report_card{
    gpa:f64,
    grade:String,
    total_marks:f64    
}

impl report_card {
    fn progress(&self,next_year:&report_card)->bool{
        self.gpa > next_year.gpa
    }
}

fn main() {

    let year1 = report_card{
        gpa:3.7,
        grade:String::from("A+"),
        total_marks:445.0
    };

    let year2 = report_card{
        gpa:3.3,
        grade:String::from("B"),
        total_marks:378.0
    };
    let year3 = report_card{
        gpa:2.9,
        grade:String::from("C"),
        total_marks:345.0
    };
    let year4 = report_card{
        gpa:3.6,
        grade:String::from("A"),
        total_marks:420.0
    };
 
    
    let all_gpa = [year1.gpa,year2.gpa,year3.gpa,year4.gpa];

    let gain_marks = [year1.total_marks,year2.total_marks,year3.total_marks,year4.total_marks];

    

    let Best_year = highest_gpa(&all_gpa);
    println!("The highest gpa is {}",Best_year);
    
    let g = Overall_marks(&gain_marks);
    println!("The Overall marks are: {}",g);

    println!("The year1 gpa is greater than year2 {}",year1.progress(&year2));
    println!("The year3 gpa is greater than year4 {}",year3.progress(&year4));
}
fn Overall_marks(max:&[f64;4])->f64{
    let mut result:f64 = 0.0;
    for x in 0..4{
        result = result + max[x];
    }
    result
}

fn highest_gpa(max_gpa:&[f64;4])->f64{
    let mut initial = max_gpa[0];
    
    for i in 0..4{
        if max_gpa[i] > initial{
            initial = max_gpa[i]
        }
    }
    initial
}
