fn main() {
  
    let a_people = vec!["Ayben", "Ayhan", "Aygün", "Aybüke", "Aybilge", "Ayaz"];

    let enum_a_people = a_people.into_iter().enumerate();

    for (i, p) in enum_a_people {

        let person_index = i as u8 + 1;

        println!("Person: {person_index}: {p}")
    }

    //println!("{a_people:?}") // borrow of moved value: `a_people`
}

// Person: 1: Ayben
// Person: 2: Ayhan
// Person: 3: Aygün
// Person: 4: Aybüke
// Person: 5: Aybilge
// Person: 6: Ayaz
