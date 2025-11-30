
use std::io::Write;
use std::fs::OpenOptions;

struct Student {
    name: String,
    matric: String,
    department: String,
    level: u32,
}

fn main(){
    // 1. Create a vector of students
    let students = vec![
        Student {
            name: "Oluchi Mordi".to_string(),
            matric: "ACC10211101".to_string(),
            department: "Accounting".to_string(),
            level: 300,
        },
        Student {
            name: "Adams Aliyu".to_string(),
            matric: "ECO10100111".to_string(),
            department: "Economics".to_string(),
            level: 200,
        },
        Student {
            name: "Shania Bolade".to_string(),
            matric: "CSC10328828".to_string(),
            department: "Computer".to_string(),
            level: 200,
        },
        Student {
            name: "Adekunle Gold".to_string(),
            matric: "EEE10202002".to_string(),
            department: "Electrical".to_string(),
            level: 100,
        },
        Student {
            name: "Blanca Edemon".to_string(),
            matric: "MEE10202001".to_string(),
            department: "Mechanical".to_string(),
            level: 100,
        },
    ];

    let mut file = std::fs::File::create("data.txt").expect("create failed");
    file.write_all("PAU SMIS - Student Records\n".as_bytes()).expect("Write failed");
    let _ =writeln!(
        file,
        "{:<20} {:<15} {:<15} {:<5}",
        "Student Name", "Matric Number", "Department", "Level"
    ).expect("write failed");
    let mut new_file = OpenOptions::new().append(true).open("data.txt").expect("cannot open file");
    for s in students{
        let _= writeln!(
            new_file,
            "{:<20} {:<15} {:<15} {:<5}",
            s.name, s.matric, s.department, s.level
        );
    }
    
    
}