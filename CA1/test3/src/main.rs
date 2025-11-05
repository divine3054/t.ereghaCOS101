use std::io;
fn main() {
    println!("
        code    item     price\n 
        T       Tea       800\n 
        C       coffee    1,200\n 
        s       sandwich  2,000 \n 
        j       juice     1,500");

    let mut total_cost:f64=0.0;
    loop{
        let mut input1=String::new();

        println!("Enter your code");
        io::stdin().read_line(&mut input1).expect("failed to read line");

        let mut input2=String::new();

        println!("Enter quantity");
        io::stdin().read_line(&mut input2).expect("failed to read line");
        let quantity:f64= input2.trim().parse().expect("an error occured");

        let temp_price:f64 =if input1== "T"{
            800.0
        }else if input1=="C"{
            1200.0
        }else if input1=="S"{
            2000.0
        }else{
            1500.0
        };

        let temp_totalcost:f64=temp_price*quantity;

        let mut input3=String::new();

        println!("enter yes if you want more and 0 if you dont");
        io::stdin().read_line(&mut input3).expect("failed to read line");
        let choice:f32 = input3.trim().parse().expect("error");
        
        total_cost+=temp_totalcost;

        if choice == 0.0{
            break;
        }
    }
    if total_cost>= 5000.0{
        total_cost*=0.95;
    }
    println!("your final amount is {}",total_cost);

    

}
