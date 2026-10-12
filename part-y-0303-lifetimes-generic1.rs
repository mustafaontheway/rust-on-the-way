fn main() {
  
    let txt_1 = String::from("ABC"); 

    let txt_2 = "ABCDE"; 

    let txt_3: &'static str = "ABCDEsas";

    let txt_4: &'static str = "ABCDEsasasasd";

    // Lifetimes are generic constraints that tie an output reference's validity directly to its input references.

    let _longest_result1 = longest(&txt_1, txt_2); // &txt_1 -> this ref might be lost, not static

    let _longest_result2 = longest_2(txt_2, txt_3);

    let _longest_result3 = longest_2(txt_3, txt_4);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {

    if x.len() > y.len() { x } else { y }
}

fn longest_2(x: &'static str, y: &'static str) -> &'static str {

    if x.len() > y.len() { x } else { y }
}

