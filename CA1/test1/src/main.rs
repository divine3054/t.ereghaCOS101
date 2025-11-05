use std::io;
fn main() {
    let mut input1=String::new();

    println!("Enter your customer name");
    io::stdin().read_line(&mut input1).expect("failed to read line");
    

    let mut input2 =String::new();

    println!("Enter the amount of unit you've consumed");
    io::stdin().read_line(&mut input2).expect("failed to read line");
    let units_consumed:f32= input2.trim().parse().expect("there was an error in reading the text");

    let rate:f32=if units_consumed>=0.0 && units_consumed<=100.0{
        20.0
    }else if units_consumed >100.0 && units_consumed<=300.0{
        35.0
    }else{
        50.0
    };

    let mut total_bill:f32=rate*units_consumed;

    if units_consumed>500.0{
        total_bill+=5000.0
    }
    println!("Dear {},\n
        units_consumed :{}\n
        rate per unit :{}
        Total bill :{}", input1,units_consumed,rate,total_bill)
}
