fn main() {

    let mut sales_by_employees: [u16; 12] = [0; 12];

    println!("Sales by employees (USD amounts / weekly: {sales_by_employees:?}");

    println!("Size 1: {:?}", std::mem::size_of_val(&sales_by_employees));

    sales_by_employees[3] = 15_000;

    println!("Sales by employees (USD amounts / weekly: {sales_by_employees:?}");

    println!("Size 2: {:?}", std::mem::size_of_val(&sales_by_employees));
}

// Sales by employees (USD amounts / weekly: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
// Size 1: 24
// Sales by employees (USD amounts / weekly: [0, 0, 0, 11000, 0, 0, 0, 0, 0, 0, 0, 0]
// Size 2: 24
