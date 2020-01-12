
fn main(){
	
    ///////////Task1//////
    let add = |x: u32,y:u32| -> u32 {
         x + y };

    let result = add(5,3);
    println!("{}",result);

    ///////Task2///////
    let ispriime = |n: u32|-> bool {
        for a in 2..n {
        if n % a == 0 {
            return false; 
        }
    }
    true 
};
    let result2 = ispriime(34);
    println!("{}",result2)

}
