use std::io::Read;
use std::io;


fn show_database_structure() {
    println!("password required To access admin");
    println!("Enter password:");
    

    let mut password = String::new();
    io::stdin().read_line(&mut password).unwrap();
    let password = password.trim();
    if password == "cos101"{
        let mut file=std::fs::File::open("globacom.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        println!("{}", contents);
    }
    else{
        println!("invalid password,Entry denied!!")
    }
    
}


fn show_employee_table() {
    let mut file=std::fs::File::open("employees.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}


fn show_dataplan_table() {
    println!("password required To acces dataplan");
    println!("Enter password:");
    let mut password = String::new();
    io::stdin().read_line(&mut password).unwrap();
    let password = password.trim();
    if password == "cos101"{
        let mut file=std::fs::File::open("dataplan.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        println!("{}", contents);
    }
    else{
        println!("invalid password,Entry denied!!")
    }
}

fn show_customer_table() {
    let mut file=std::fs::File::open("globacom.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}
fn show_department_table() {
    let mut file=std::fs::File::open("department.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}
fn show_project_table() {
    let mut file=std::fs::File::open("project.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

fn main() {
    loop{
        println!("Enter user role:");
    println!("admin | employee | customer | vendor | project_manager");

    let mut role = String::new();
    io::stdin().read_line(&mut role).unwrap();
    let role = role.trim();

    match role {
        "admin" => show_database_structure(),
        "employee" => show_employee_table(),
        "project_manager" => show_project_table(),
        "customer" => show_customer_table(),
        "vendor" => show_dataplan_table(),
        _ => println!("Invalid role entered"),
    }
    println!("DO YOU WISH TO CHANGE YOUR ROLE");
    println!("Y/N");
    let mut restart = String::new();
    io::stdin().read_line(&mut restart).unwrap();
    let restart = restart.trim();
    if restart == "Y"{
        continue;
    }
    else{
        break;
    }
    }
    
}
