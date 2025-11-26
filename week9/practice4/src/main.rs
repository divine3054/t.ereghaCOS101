use std::fs::OpenOptions;
use std::io::Write;
fn main() {
    let mut file = OpenOptions::new().append(true).open("../files/data.txt").expect("cannot open file");
    let dept = "Department of Computer Sciences";

    
    file.write_all("\nHello class".as_bytes()).expect("Write Failed");
    file.write_all("\n This is the appendage to the document".as_bytes()).expect("Write failed");
    
    println!("file append success")
}