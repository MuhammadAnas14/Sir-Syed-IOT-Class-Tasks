fn main() {
    
    let mut japanese = String::from("japanese");

    japanese.push_str("language");

    let arabic = String::from("السلام عليكم");
   
    let mut jap_ara = japanese + &arabic;

    jap_ara.push_str("!");

    println!("{}",jap_ara);

    for c in arabic.chars() {
        println!("{}", c);
    }
    for b in arabic.bytes() {
        println!("{}", b);
    }


}
