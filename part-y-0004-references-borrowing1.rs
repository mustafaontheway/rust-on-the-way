fn main() {

    // Primitive Stack types implement Copy trait, so direct assignment clones value instead of moving ownership
    // References (&) work on Stack values too, but aren't mandatory to preserve ownership of Copy types
  
    let age = 36u8;

    let her_age = &age;

    let his_age = age;

    println!("Age diff: {}", age - her_age); // Age diff: 0

    println!("Age diff: {}", age - his_age); // Age diff: 0
}
