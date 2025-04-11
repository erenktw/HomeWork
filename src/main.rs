use std::io;

#[derive(Debug)]
struct Customer {
    name: String,
    surname: String,
    balance: f32,
}

#[derive(Debug)]
struct Product {
    name: String,
    price: f32,
    stock: u32,
}

struct Market {
    customers: Vec<Customer>,
    products: Vec<Product>,
}

impl Market {
    fn new() -> Self {
        Market {
            customers: Vec::new(),
            products: Vec::new(),
        }
    }

    fn add_customer(&mut self, name: &str, surname: &str, balance: f32) {
        self.customers.push(Customer {
            name: name.to_string(),
            surname: surname.to_string(),
            balance,
        });
    }

    fn add_product(&mut self, name: &str, price: f32, stock: u32) {
        self.products.push(Product {
            name: name.to_string(),
            price,
            stock,
        });
    }

    fn list_products(&self) {
        println!("--- Product List ---");
        for (i, product) in self.products.iter().enumerate() {
            println!(
                "{} - {} | Price: {:.2} | Stock: {}",
                i, product.name, product.price, product.stock
            );
        }
    }

    fn list_customers(&self) {
        println!("--- Customer List ---");
        for (i, customer) in self.customers.iter().enumerate() {
            println!(
                "{} - {} {} | Balance: {:.2}",
                i, customer.name, customer.surname, customer.balance
            );
        }
    }

    fn purchase(&mut self, customer_index: usize, product_index: usize, quantity: u32) {
        if customer_index >= self.customers.len() || product_index >= self.products.len() {
            println!("Invalid customer or product selection.");
            return;
        }

        let product = &mut self.products[product_index];
        let customer = &mut self.customers[customer_index];

        let total_price = product.price * quantity as f32;

        if product.stock < quantity {
            println!("Not enough stock available.");
        } else if customer.balance < total_price {
            println!("Insufficient balance.");
        } else {
            product.stock -= quantity;
            customer.balance -= total_price;
            println!(
                "{} purchased {} x '{}'.",
                customer.name, quantity, product.name
            );
        }
    }
}

fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let mut market = Market::new();

    // Sample data
    market.add_customer("John", "Smith", 150.0);
    market.add_customer("Emily", "Johnson", 300.0);

    market.add_product("Apple", 5.0, 50);
    market.add_product("Banana", 7.5, 30);
    market.add_product("Chocolate", 10.0, 20);

    loop {
        println!("\n==== Market System ====");
        println!("1. Show Customers");
        println!("2. Show Products");
        println!("3. Purchase Product");
        println!("4. Exit");

        let choice = read_input("Your choice:");

        match choice.as_str() {
            "1" => market.list_customers(),
            "2" => market.list_products(),
            "3" => {
                market.list_customers();
                let customer_index: usize = read_input("Enter customer number:")
                    .parse()
                    .unwrap_or(0);

                market.list_products();
                let product_index: usize = read_input("Enter product number:")
                    .parse()
                    .unwrap_or(0);

                let quantity: u32 = read_input("How many would you like to buy?")
                    .parse()
                    .unwrap_or(1);

                market.purchase(customer_index, product_index, quantity);
            }
            "4" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid option, please try again."),
        }
    }
}
