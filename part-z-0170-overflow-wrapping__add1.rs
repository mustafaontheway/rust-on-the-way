fn main() {

    let amount: u16 = 60_000;

    let new_amount = amount.wrapping_add(2_000);

    println!("New amount: {new_amount:?}");

    let new_amount = amount.wrapping_add(12_000);

    println!("New amount: {new_amount:?}")
}

// New amount: 62000
// New amount: 6464 -> 60_000 + 12_000 = 72_000 -> 72_000 - 65_536 = 6464
