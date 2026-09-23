fn main() {

    let (_names1, _names2) = return_values("Mustafa", "Aygül");

    let (mut age, _is_married) = return_values(37, false);

    age += 2;

    println!("Her real age is {age}.")
}

fn return_values<T, U>(v1: T, v2: U) -> (T, U) {

    (v1, v2)
}

// Her real age is 39.
