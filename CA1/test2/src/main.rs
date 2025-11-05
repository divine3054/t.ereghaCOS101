use std::io;

fn main() {
    loop{
        let mut input1=String::new();

        println!("Enter your principal");
        io::stdin().read_line(&mut input1).expect("failed to read line");
        let p:f32= input1.trim().parse().expect("there was an error in reading the text");

        let mut input2=String::new();

        println!("Enter the rate");
        io::stdin().read_line(&mut input2).expect("failed to read line");
        let r:f32= input2.trim().parse().expect("there was an error in reading the text");

        let mut input3 =String::new();

        println!("Enter the amount of time");
        io::stdin().read_line(&mut input3).expect("failed to read line");
        let t:f32= input3.trim().parse().expect("there was an error in reading the text");


        let a:f32=p*(1.0+(r/100.0)).powf(t);
        let _local_interest=a-p;


        println!("do you wish to calculate for another user \n if yes type 1 else type 0");
        let mut input4=String::new();

        
        io::stdin().read_line(&mut input4).expect("failed to read line");
        let choice:i32=input4.trim().parse().expect("error");
        
         

        if choice == 0{
            break;
        }
    }
}
