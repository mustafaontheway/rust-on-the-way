fn main() {

    let my_num: u8 = 98;

    let even_or_odd = match my_num {

        val if val % 2 == 0 => format!("{val} is even."),
        val if val % 2 != 0 => format!("{val} is odd."),
        _ => format!("Not even or odd",)
    };

    println!("{even_or_odd}")
}

// 98 is even.
