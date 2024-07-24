use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Debug, Clone)]
struct Product {
    name: String,
    price: f64,
}

#[derive(Debug)]
struct Receipt {
    products: Vec<(Product, u32)>,
    total: f64,
}

impl Receipt {
    fn new() -> Self {
        Receipt {
            products: Vec::new(),
            total: 0.0,
        }
    }

    fn add_product(&mut self, product: Product, quantity: u32) {
        self.total += quantity as f64 * product.price;
        self.products.push((product, quantity));
    }

    fn print_receipt(&self) {
        println!("--- Shopping Receipt ---");
        for (product, quantity) in &self.products {
            println!(
                "{} x{} @ ₹{:.2} each = ₹{:.2}",
                product.name,
                quantity,
                product.price,
                *quantity as f64 * product.price
            );
        }
        println!("------------------------");
        println!("Total: ₹{:.2}", self.total);
    }
}

fn main() {
    let market_products = vec![
        Product { name: "Apple".to_string(), price: 30.0 },
        Product { name: "Milk".to_string(), price: 50.0 },
        Product { name: "Bread".to_string(), price: 40.0 },
        Product { name: "Eggs".to_string(), price: 5.0 },
        Product { name: "Rice".to_string(), price: 60.0 },
        Product { name: "Chicken".to_string(), price: 150.0 },
    ];
    let products = market_products.clone();
    println!("| Product Name | Price |");
    println!("|-------------|-------|");
    for product in products {
        println!("| {:<11} | {:>5.1} |", product.name, product.price);
    }

    let product_map: HashMap<String, Product> = market_products.into_iter()
        .map(|product| (product.name.clone(), product))
        .collect();

    let mut receipt = Receipt::new();

    loop {
        let mut product_name = String::new();
        print!("Enter product name (or 'done' to finish): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut product_name).unwrap();
        let product_name = product_name.trim().to_string();

        if product_name.to_lowercase() == "done" {
            break;
        }

        if let Some(product) = product_map.get(&product_name) {
            let mut quantity_str = String::new();
            print!("Enter quantity for {}: ", product_name);
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut quantity_str).unwrap();
            let quantity: u32 = quantity_str.trim().parse().expect("Please enter a valid number");

            receipt.add_product(product.clone(), quantity);
        } else {
            println!("Product {} not found in the market.", product_name);
        }
    }

    receipt.print_receipt();
}
