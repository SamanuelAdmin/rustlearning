use std::cell::Cell;


#[derive(Debug)]
struct Order {
    name: String,
    year: u32,
    made_by_phone: bool,
    made_by_mobile: bool,
    made_by_email: bool,
    item_number: u32,
    count: Cell<u32>,
}

fn create_order_template(new_name: String) -> Order {
    Order {
        name: new_name,
        year: 2019,
        made_by_phone: false,
        made_by_mobile: false,
        made_by_email: true,
        item_number: 123,
        count: 0.into(),
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn your_order() {
        let order_template = create_order_template(String::from("Bob"));

        // TODO: Create your own order using the update syntax and template above!
//        let your_order = Order {
//            name: String::from("Hacker in Rust"),
//            year: 2019,
//            made_by_phone: false,
//            made_by_mobile: false,
//            made_by_email: true,
//            item_number: 123,
//            count: 1
//        };
        let your_order = create_order_template(String::from("Hacker in Rust"));
        your_order.count.set(1);

        assert_eq!(your_order.name, "Hacker in Rust");
        assert_eq!(your_order.year, order_template.year);
        assert_eq!(your_order.made_by_phone, order_template.made_by_phone);
        assert_eq!(your_order.made_by_mobile, order_template.made_by_mobile);
        assert_eq!(your_order.made_by_email, order_template.made_by_email);
        assert_eq!(your_order.item_number, order_template.item_number);
        assert_eq!(your_order.count, 1.into());
    }
}
