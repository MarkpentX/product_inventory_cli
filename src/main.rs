use std::io;

struct ProductItem{
    name: String,
    price: u32
}

struct Products{
    products: Vec<ProductItem>
}

impl Products{
    fn add_product(&mut self, item: ProductItem){
        self.products.push(item);
    }

    fn show_inventory(&self) {
        if self.products.is_empty() {
            println!("You don't have any products");
            return;
        }

        for (i, item) in self.products.iter().enumerate() {
            println!("{}: {} ({}$)", i + 1, item.name.trim(), item.price);
        }
    }

    fn clear_inventory(&mut self) {
        self.products.clear();
        println!("All products have been cleared");
    }
}

fn main() {
    let mut products_list = Products {
        products: Vec::new(),
    };
    loop {
        println!("Please enter what you want \n1: Add a new product\n2: Show all products\n3: Delete all products\n4: Stop app");
        let user_answer = get_user_answer_in_number();

        match user_answer {
            1 => {
                println!("Enter product name:");
                let mut product_name = String::new();

                io::stdin()
                    .read_line(&mut product_name)
                    .expect("Failed to read line");

                let price = get_user_answer_in_number();
                let product = ProductItem {
                    name: product_name,
                    price,
                };

                products_list.add_product(product)
            }
            2 => {
                products_list.show_inventory();
            }
            3 => {
                products_list.clear_inventory()
            }
            4 => break,
            other => {
                println!("Unknown answer type: {}", other);
            }
        }
    }
}

fn conversion_text_to_number(conversion_text: &str) -> u32 {
    conversion_text.trim().parse().expect("Please type a number!")
}

fn get_user_answer_in_number() -> u32 {
    let mut user_answer = String::new();

    io::stdin()
        .read_line(&mut user_answer)
        .expect("Failed to read line");

    conversion_text_to_number(&user_answer)
}