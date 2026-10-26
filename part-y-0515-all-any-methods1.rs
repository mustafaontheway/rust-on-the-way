fn main() {
 
    let citizen_ages = vec![17u8, 85, 24, 10, 8, 96, 75, 21];

    let are_all_voters = citizen_ages.iter().all(|age| *age >= 18 && *age <= 90);

    println!("Are all voters? {:?}", are_all_voters);

    let any_voter = citizen_ages.iter().any(|age| (18..=90).contains(age));

    println!("Any voter? {:?}", any_voter);
}

// Are all voters? false
// Any voter? true
