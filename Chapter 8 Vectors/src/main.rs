fn main() {

    // 1)
    let laptops = vec!["Dell","Hp","lenovo","Apple","Asus"];
    
    // 2)
    let length = laptops.len();


    println!("The laptop companies are {:?}",laptops);

    println!("we have {} companies",length);

    // 3)
    let prices = vec![45000.0,87000.0,60000.0,120000.0,58000.0];
    
    // 4)

    for i in &prices{


        let tax:f64 = i * 0.5;
        let final_price = i + tax;

        println!("The final price is {}",final_price);

    }
    
    // 5)
    let last_item = &laptops[4];
    // println!("The third element is {}", last_item);

    match laptops.get(6) {
    Some(last_item) => println!("The last element is {}", last_item),
    None => println!("There is no element.")
}

}
