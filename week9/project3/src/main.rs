use std::io::Write;
use std::fs::OpenOptions;
fn main() {
    let names_of_commissioners: [&str; 5] = [
    "Aigbogun Alamba Daudu",
    "Murtala Afeez Bendu",
    "Okorocha Calistus Ogbona",
    "Adewale Jimoh Akanbi",
    "Osazuwa Faith Etieye",
    ];

    let ministries: [&str; 5] = [
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let geopolitical_zones: [&str; 5] = [
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];
 
    let mut file = std::fs::File::create("data.txt").expect("create failed");
    let _= writeln!(
                file,
                "{:<25} {:<20} {:<15}\n\n",
                "names_of_commissioners", "ministries", "geopolitical_zones",
            );
    let mut new_file = OpenOptions::new().append(true).open("data.txt").expect("cannot open file");        
    for s in 0..=4{
        let _= writeln!(
            new_file,
            "{:<25} {:<20} {:<15}",
            names_of_commissioners[s], ministries[s], geopolitical_zones[s],
        );
        }

}
