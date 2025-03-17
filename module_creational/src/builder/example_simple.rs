#[derive(Debug, PartialEq)]
pub struct Product {
    name: String,
    price: u32,
    amount: u32,
}

impl Product {}

#[derive(Default)]
pub struct ProductBuilder {
    name: String,
    price: u32,
    amount: u32,
}

impl ProductBuilder {
    pub fn new() -> ProductBuilder {
        ProductBuilder {
            name: "".to_string(),
            price: 0,
            amount: 0,
        }
    }

    pub fn with_name(mut self, name: String) -> ProductBuilder {
        self.name = name;
        self
    }

    pub fn with_price(mut self, price: u32) -> ProductBuilder {
        self.price = price;
        self
    }

    pub fn with_amount(mut self, amount: u32) -> ProductBuilder {
        self.amount = amount;
        self
    }

    pub fn build(self) -> Product {
        Product {
            name: self.name,
            price: self.price,
            amount: self.amount,
        }
    }
}

#[test]
fn builder_test() {
    let product_common = Product {
        name: "Monitor".to_string(),
        price: 1500000,
        amount: 5,
    };

    let product_builder: Product = ProductBuilder::new()
        .with_name(String::from("Monitor"))
        .with_price(1500000)
        .with_amount(5)
        .build();

    assert_eq!(product_common, product_builder);
}