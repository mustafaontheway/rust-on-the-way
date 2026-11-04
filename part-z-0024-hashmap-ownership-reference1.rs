use std::collections::HashMap;

fn main() {

    let member1_name = String::from("Ayhan Bilir");

    let member1_age = 27u8;

    let member2_name = String::from("Bengü Bilir");

    let member2_age = 25u8;

    // let mut family_members: HashMap<&String, u8> = HashMap::new();

    // family_members.insert(&member1_name, member1_age);

    // family_members.insert(&member2_name, member2_age);

    // println!("{member2_name}"); // Bengü Bilir

    let mut family_members: HashMap<String, u8> = HashMap::new();

    family_members.insert(member1_name, member1_age);

    //println!("{member1_name}"); // Error! value borrowed here after move

    family_members.insert(member2_name.clone(), member2_age);

    println!("{member2_name}"); // Bengü Bilir
}

