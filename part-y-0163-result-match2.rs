fn main() {

}

fn sum_numbers2(x: i128, y: i128) -> Result<i128, String> {

    match x.checked_add(y) {
        Some(r) => Ok(r),
        None => Err("Overflow error: sum exceeded i128 limits".to_string())
    }
}

fn sum_numbers(x: i128, y: i128) -> Result<i128, String> {

    let inner_sum = Ok(x + y);

    if inner_sum.is_ok() {

        inner_sum

    } else {

        Err("Some errors found...".to_string())
    }
}
