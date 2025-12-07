struct Laptop {
    brand: String,
    price: u32,
}

impl Laptop {
    fn total_cost(&self, quantity: u32) -> u32 {
        self.price * quantity
    }
}

fn main() {
    let hp = Laptop { brand: "HP".to_string(), price: 650_000 };
    let ibm = Laptop { brand: "IBM".to_string(), price: 755_000 };
    let toshiba = Laptop { brand: "Toshiba".to_string(), price: 550_000 };
    let dell = Laptop { brand: "Dell".to_string(), price: 850_000 };

    let qty = 3;

    let total_hp = hp.total_cost(qty);
    let total_ibm = ibm.total_cost(qty);
    let total_toshiba = toshiba.total_cost(qty);
    let total_dell = dell.total_cost(qty);

    let grand_total = total_hp + total_ibm + total_toshiba + total_dell;

    println!("Total cost for 3 HP laptops: {}", total_hp);
    println!("Total cost for 3 IBM laptops: {}", total_ibm);
    println!("Total cost for 3 Toshiba laptops: {}", total_toshiba);
    println!("Total cost for 3 Dell laptops: {}", total_dell);
    println!("\nGrand Total Cost: {}", grand_total);
}
