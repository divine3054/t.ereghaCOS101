use std::io::Write;
fn main() {
       let content = "\
            Lager:
            - 33 Export
            - Desperados
            - Goldberg
            - Gulder
            - Heineken
            - Star

            Stout:
            - Legend
            - Turbo King
            - Williams

            Non-Alcoholic:
            - Maltina
            - Amstel Malta
            - Malta Gold
            - Fayrouz
        ";

    let mut file = std::fs::File::create("data.txt").expect("create failed");
    file.write_all(content.as_bytes()).expect("Write failed");
   
    println!("\nData written to file.")
}
