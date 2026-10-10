fn main() {

    let my_name = String::from("Mustafa");

    let r1 = return_three_chars(&my_name);

    println!("{r1}");

    let her_name = "Aygün";

    let _r2 = return_three_chars(her_name);
}

fn return_three_chars(name: &str) -> &str {

    &name[..=2]
}
