pub struct Order {
    product_name: String,
    quantity: u32,
    unit_price: u32,
}

impl Order {
    pub fn new(product_name: String, quantity: u32, unit_price: u32) -> Order {
        validate_product_name(&product_name);
        validate_quantity(&quantity);
        validate_unit_price(&unit_price);
        Order {
            product_name,
            quantity,
            unit_price,
        }
    }
    pub fn total(&self) -> u32 {
        self.unit_price * self.quantity
    }
    pub fn set_product_name(&mut self, product_name: String) {
        validate_product_name(&product_name);
        self.product_name = product_name
    }
    pub fn product_name(&self) -> &String {
        &self.product_name
    }
    pub fn set_quantity(&mut self, quantity: u32) {
        validate_quantity(&quantity);
        self.quantity = quantity
    }
    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }
    pub fn set_unit_price(&mut self, unit_price: u32) {
        validate_unit_price(&unit_price);
        self.unit_price = unit_price
    }
    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }
}

fn validate_product_name(product_name: &String) {
    if product_name.is_empty() {
        panic!("Product name cannot be empty!")
    }
    if product_name.len() > 300 {
        panic!("Product name cannot be greater than 300 bytes!")
    }
}

fn validate_quantity(quantity: &u32) {
    if *quantity <= 0 {
        panic!("Quantity cannot be negative!")
    }
}

fn validate_unit_price(unit_price: &u32) {
    if *unit_price <= 0 {
        panic!("Unit price cannot be negative!")
    }
}

#[test]
fn test_order() {
    let mut order = Order::new("Rusty Book".to_string(), 3, 2999);

    assert_eq!(order.product_name(), "Rusty Book");
    assert_eq!(order.quantity(), &3);
    assert_eq!(order.unit_price(), &2999);
    assert_eq!(order.total(), 8997);

    order.set_product_name("Rust Book".to_string());
    order.set_quantity(2);
    order.set_unit_price(3999);

    assert_eq!(order.product_name(), "Rust Book");
    assert_eq!(order.quantity(), &2);
    assert_eq!(order.unit_price(), &3999);
    assert_eq!(order.total(), 7998);
}

#[test]
#[should_panic]
fn test_empty_product_name() {
    Order::new("".to_string(), 3, 2999);
}

#[test]
#[should_panic]
fn test_long_product_name() {
    Order::new("a".repeat(301), 3, 2999);
}

#[test]
#[should_panic]
fn test_zero_quantity() {
    Order::new("Rust Book".to_string(), 0, 2999);
}

#[test]
#[should_panic]
fn test_zero_unit_price() {
    Order::new("Rust Book".to_string(), 3, 0);
}
