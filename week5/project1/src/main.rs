use std::io;
fn main() {
    println!("Menu | Price");
    println!("----------------------------");
    println!("P = Poundo Yam/Edinkaiko Soup - N3,200");
    println!("F = Fried Rice & Chicken      - N3,000");
    println!("A = Amala & Ewedu Soup        - N2,500");
    println!("E = Eba & Egusi Soup          - N2,000");
    println!("W = White Rice & Stew         - N2,500");

    let mut total_cost:f64=0.0;
    loop{
        let mut input1=String::new();

        println!("Enter your code");
        io::stdin().read_line(&mut input1).expect("failed to read line");

        let input1= input1.trim();

        let mut input2=String::new();

        println!("Enter quantity");
        io::stdin().read_line(&mut input2).expect("failed to read line");
        let quantity:f64= input2.trim().parse().expect("an error occured");

        let temp_price:f64 =if input1== "P"{
            3200.00
        }else if input1=="F"{
            3000.0
        }else if input1=="A"{
            2500.0
        }else if input1=="E"{
            2000.0
        }else{
            2500.0
        };

        let temp_totalcost:f64=temp_price*quantity;

        let mut input3=String::new();

        println!("enter yes if you want to order and no if you dont");
        io::stdin().read_line(&mut input3).expect("failed to read line");
        input3 = input3.trim().to_string();
        
        total_cost+=temp_totalcost;

        if input3 == "no"{
            break;
        }
    }
    if total_cost>= 10000.0{
        total_cost*=0.95;
    }
    println!("your final amount is {}",total_cost);
    println!("thank you for your patience");
}

