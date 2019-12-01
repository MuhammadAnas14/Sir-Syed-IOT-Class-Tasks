use std::io;
fn main() {

enum Items{
    Fruits,
    Vegetable, 
    Household_goods,
    
}

fn Price(items: Items) -> u8 {
    match items {
        Items::Fruits => 15,
        Items::Vegetable => 12,
        Items::Household_goods => 35,
    }
}
#[derive(Debug)]
struct Customer_detail {
    Name: String,
    Phone_no:u32,
    Address:String
}

fn details(customer: &Customer_detail){
    println!("the name of customer is {}",customer.Name);
    println!("the phone_no of customer is {}",customer.Phone_no);
    println!("the Address of customer is {}",customer.Address); 
}


loop {
    

    println!("WHAT DO YOU WANT TO BUY(fruits/vegetable/household");
    let mut select  = String::new();
    io::stdin().read_line(&mut select).expect("invalid input");
    
    println!("WHAT is your name?");
    let mut name  = String::new();
    io::stdin().read_line(&mut name).expect("invalid input");
    
    println!("enter the phone no");
    let mut phone_no = String::new();
    io::stdin().read_line(&mut phone_no).expect("invalid input");
    let phone_no = phone_no.trim().parse::<u32>().unwrap();
    
    println!("enter the Address");
    let mut address  = String::new();
    io::stdin().read_line(&mut address).expect("invalid input");
    
    let customer1 = Customer_detail{
        Name:name,
        Phone_no:phone_no,
        Address:address
    };

    details(&customer1);


    if select.trim() == "fruits"{
        let purchase = Items::Fruits;
        let total_price = Price(purchase);
        println!("The total price of your purchase items are {}",total_price);
        break
    }

    else if select.trim() == "vegetable"{
        let purchase = Items::Vegetable;
        let total_price = Price(purchase);
        println!("The total price of your purchase items are {}",total_price);
        break
    }

    else if select.trim() == "household"{
        let purchase = Items::Household_goods;
        let total_price = Price(purchase);
        println!("The total price of your purchase items are {}",total_price);
        
        break
    }

    else {
        println!("invalid selection")
    }
    
}

}

