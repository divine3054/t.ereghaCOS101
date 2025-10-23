use std::io;

fn main() {
    let mut input1 = String::new();

    println!("enter age");
    io::stdin().read_line(&mut input1).expect("wrong entry");
    let age:f32=input1.trim().parse().expect("error when reading");

    let mut input2 =String::new();

    println!("are u experienced enter 1 else 2 for inexperienced");
    io::stdin().read_line(&mut input2).expect("wrong entry");
    let experienced:i32=input2.trim().parse().expect("error when reading");

    if experienced==1 && age >=40.0{
        println!("your incentive is N1,560,000");
    }else if experienced==1 && age >=30.0 && age<40.0{
        println!("your incentive is N1,480,000");
    }else if experienced==1 && age <28.0{
        println!("your incentive is N1,300,000");
    }else{
        println!("your incentive is N100,000");
    }
}
