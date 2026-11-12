fn main() {

    let mut text1 = String::from("Hi there!");

    let mut text2 = String::from("Lorem ipsum...");

    let longest1 = return_longest_text(&mut text1, &mut text2);

    println!("Longest text 1: {longest1}");

    text1.push_str(" Are you ok today?");

    text2 = "A barking dog never bites!".to_string();

    let longest2 = return_longest_text(&mut text1, &mut text2);

    println!("Longest text 1: {longest2}");

}

fn return_longest_text<'a>(txt1: &'a mut String , txt2: &'a mut String ) -> &'a mut String {

    if txt1.len() >= txt2.len() {

        txt1

    } else {

        txt2
    }
}

// Longest text 1: Lorem ipsum...
// Longest text 1: Hi there! Are you ok today?
