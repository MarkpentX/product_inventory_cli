mod products;
mod input;
use products::*;
use input::*;

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
                let product_name = get_user_answer_in_string();

                println!("Enter product price:");
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

