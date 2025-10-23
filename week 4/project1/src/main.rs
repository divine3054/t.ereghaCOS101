use std::io;
fn main() {
    let mut input1 = String::new();

    println!("enter a");
    io::stdin().read_line(&mut input1).expect("wrong entry");
    let a:f32=input1.trim().parse().expect("error when reading");

    let mut input2 =String::new();

    println!("enter b");
    io::stdin().read_line(&mut input2).expect("wrong entry");
    let b:f32=input1.trim().parse().expect("error when reading");

    let mut input3 = String::new();

    println!("enter c");
    io::stdin().read_line(&mut input3).expect("wrong entry");
    let c:f32=input1.trim().parse().expect("error when reading");


    let discriminant=b*b-(4.0*a*c);

    if discriminant>0.0{
        let root1=(-b+discriminant.sqrt())/(2.0/a);
        let root2=(-b-discriminant.sqrt())/(2.0/a);
        println!("roots are {} and {}",root1,root2);

    }else if discriminant==0.0{
        let root = -b/(2.0 * a);
        println!("root is {}",root);
    }
    else{
        println!("no real roots they are complex");
        let real_part=-b/(2.0*a);
        let i=(-discriminant).sqrt()/(2.0*a);
        println!("root1 = {} + {}i", real_part,i);
        println!("root2 = {} - {}i",real_part,i);

    }
}
