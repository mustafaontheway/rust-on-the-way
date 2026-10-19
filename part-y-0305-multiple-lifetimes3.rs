const MY_PROVERB: &str = "Out of sight, out of mind";

fn main() {

    let greet = "Hello people!".to_string();

    {
        let new_greet = "Hi there!";
        
        return_words(new_greet, &greet, "Be careful!");
    }

    return_words(MY_PROVERB, &greet, "Be careful!"); 
}

// different lifetimes!

fn return_words<'a, 'b>(w1: &'a str, w2: &'b str, warning: &'static str) -> (&'a str, &'b str) {

    println!("{warning}");

    (w1, w2)   
}
