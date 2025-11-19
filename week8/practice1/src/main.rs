fn main() {
    //using Vec::new()
    let v:Vec<i64> = Vec::new();

    // printing the size of vector
    println!("\nThe length of Vec::new is: {}",v.len());

    // Using macro
    let v = vec!["Divine","Dominion","Delight","kareem","patience"];

    // printing the size of vector
    println!("\n The length of vec macro is: {}",v.len());
}
