fn main() {
    let name1 = "Alice Smith";
    println!("My name is {}", name1);

    // find and replace
    let name2 = name1.replace("Alice", "Bob");
    println!("You can also call me {}", name2);

    let faculty = "Faculty of Arts and Humanities";
    // find and replace
    let school = faculty.replace("Faculty", "School");
    println!("I am a student of the {}", school);
}