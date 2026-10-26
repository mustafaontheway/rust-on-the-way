fn main() {

    let quarterly_car_sales_amounts = vec![
        vec![950u16, 800, 845],
        vec![750, 812, 1000],
        vec![965, 700, 745],
        vec![1010, 745, 902],
    ];

    let monthly_car_sales_amounts: Vec<u16> = quarterly_car_sales_amounts.iter().flatten().copied().collect();

    println!("{monthly_car_sales_amounts:?}");

    // let's use for loops

    let mut monthly_car_sales_amounts: Vec<u16> = Vec::new();

    for quarter in &quarterly_car_sales_amounts {

        for month in quarter {

            monthly_car_sales_amounts.push(*month);
        }
    }

    println!("{monthly_car_sales_amounts:?}");
}

// [950, 800, 845, 750, 812, 1000, 965, 700, 745, 1010, 745, 902]
// [950, 800, 845, 750, 812, 1000, 965, 700, 745, 1010, 745, 902]
