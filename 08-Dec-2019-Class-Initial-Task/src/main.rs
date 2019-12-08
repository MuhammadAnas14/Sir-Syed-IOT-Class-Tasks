use std::collections::HashMap;
use std::io;

fn main() {

    let mut bed = HashMap::new();
    
    bed.insert(String::from("Moltiform"), 50000);
    bed.insert(String::from("Dulex"), 70000);
    bed.insert(String::from("bedmakers"), 4000);


    let mut sofa = HashMap::new();
    
    sofa.insert(String::from("Moltiform"), 50000);
    sofa.insert(String::from("Khan Brothera"), 70000);
    sofa.insert(String::from("Yosuf's Sofas"), 4000);


    let mut DresssingTable = HashMap::new();
    
    DresssingTable.insert(String::from("Furniture Makers"), 500000);
    DresssingTable.insert(String::from("Gul Sofas house"), 70000);
    DresssingTable.insert(String::from("Amazon Sofas"), 4000);

    loop{

        println!("WHAT DO YOU WANT TO BUY(1:Bed/2:sofas/3:dressing Table");
        let mut select  = String::new();
        io::stdin().read_line(&mut select).expect("invalid input");

        if select.trim() == "1"{

            println!("Which Company:");
            let mut select_company  = String::new();
            io::stdin().read_line(&mut select_company).expect("invalid input");

            for (key, value) in &bed {
                if key == select_company.trim(){

                    Total_price(key.to_string(),value)

                }
            }
        
        }


        else if select.trim() == "2"{

            println!("Which Company");
            let mut select_company  = String::new();
            io::stdin().read_line(&mut select_company).expect("invalid input");

            for (key, value) in &sofa {
                if key == select_company.trim(){
                
                    Total_price(key.to_string(),value)

                }
            }
        
        }
        else if select.trim() == "3"{

            println!("Which Company");
            let mut select_company  = String::new();
            io::stdin().read_line(&mut select_company).expect("invalid input");

            for (key, value) in &DresssingTable {
                if key == select_company.trim(){
                    Total_price(key.to_string(),value)

                }
            }
        
        }
        else{
            println!("Invalid Input")
        }

        println!("Do you want to cntinue?");
        let mut con  = String::new();
        io::stdin().read_line(&mut con).expect("invalid input");
            
        if con.trim() == "no"{
            continue
        }
        else{
            break
        }

}

}

fn Customer_details(){

    println!("WHAT is your name?");
    let mut name  = String::new();
    io::stdin().read_line(&mut name).expect("invalid input");
    
    println!("enter the phone no");
    let mut phone_no = String::new();
    io::stdin().read_line(&mut phone_no).expect("invalid input");
    let phone_no = phone_no.trim().parse::<u32>().unwrap();

    println!("Your Recipt:");
    println!("Customer Name: {}",name);
    println!("Phone No: {}",phone_no);

}
fn Total_price(item:String,price:&i32){
    
    Customer_details();
    println!("Item Brought{}",item);
    println!("price {}",price)
}