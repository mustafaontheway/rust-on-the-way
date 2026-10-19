fn main() {

    let word1 = "Hi".to_string();
    let word2 = "Hello".to_string();

    return_words(&word1, &word2, "Be careful!"); 
}

// rust foundation, i think this is "over-engineering"! 

fn return_words<'a, 'b>(w1: &'a str, w2: &'b str, warning: &'static str) -> (&'a str, &'b str) {

    println!("{warning}");

    (w1, w2)   
}
