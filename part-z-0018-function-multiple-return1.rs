fn main() {

    let (sum, subs) = sum_or_subs(-5000, 119);

    println!("{sum}");

    println!("{subs}")
}

fn sum_or_subs(x: i128, y: i128) -> (i128, i128) {

    (x + y, x - y)
}

// -4881
// -5119
