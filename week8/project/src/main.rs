use std::io;

struct Person {
    name: String,
    experience_years: u32,
}

fn get_user_input() -> Vec<Person> {
    let mut people: Vec<Person> = Vec::new();
    

    loop {
       
        println!("Enter your name");
        
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read line");
        let name = name.trim().to_string();

        if name=="" {
            break;
        }

        println!("Enter experience years for {}:", name);
        let mut str = String::new();
        io::stdin().read_line(&mut str).expect("Failed to read line");

        let experience_years: u32 =str.trim().parse().expect("error reading line");

        people.push(Person { name, experience_years });
    }
    people
}

fn main() {
    let people_data = get_user_input();

    let highest_experience_person = people_data.iter()
        .max_by_key(|person| person.experience_years);

    match highest_experience_person {
        Some(person) => {
            println!("The person with the highest experience is: {} with {} years.",
                     person.name, person.experience_years);
        },
        None => {
            println!("The list of candidates is empty.");
        }
    }
}
