pub struct ProductItem{
    pub name: String,
    pub price: u32
}

pub struct Products{
    pub products: Vec<ProductItem>
}

impl Products {
    pub fn add_product(&mut self, item: ProductItem){
        self.products.push(item);
    }

    pub fn show_inventory(&self) {
        if self.products.is_empty() {
            println!("You don't have any products");
            return;
        }

        for (i, item) in self.products.iter().enumerate() {
            println!("{}: {} ({}$)", i + 1, item.name.trim(), item.price);
        }
    }

    pub fn clear_inventory(&mut self) {
        self.products.clear();
        println!("All products have been cleared");
    }
}