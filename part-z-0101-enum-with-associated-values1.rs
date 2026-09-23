fn main() {

    let head_fintech = Heads::FinTech("Musta Büyükdereli".to_string());

    println!("FinTech Head Person is {:?}", head_fintech);

    if let Heads::FinTech(name) = head_fintech {

        println!("FinTech Head Person is {}", name);
    }
}

#[derive(Debug)]
enum Heads {

    FinTech(String),
    Finance(String),
    Sales(String),
    Operations(String),
    HR(String),
    Audit(String)
}

// FinTech Head Person is FinTech("Musta Büyükdereli")
// FinTech Head Person is Musta Büyükdereli
