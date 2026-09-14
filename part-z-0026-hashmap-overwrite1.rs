use std::collections::HashMap;

fn main() {

    let member1_name = String::from("Ayhan Bilir");

    let member2_name = String::from("Bengü Bilir");

    let mut family_members_jobs: HashMap<&str, &str,> = HashMap::new();

    family_members_jobs.insert(&member1_name, "Teacher");

    family_members_jobs.insert(&member2_name, "Nurse");

    println!("{:?}", family_members_jobs); 

    family_members_jobs.insert(&member2_name, "Student");

    println!("{:?}", family_members_jobs); 
}

// {"Bengü Bilir": "Nurse", "Ayhan Bilir": "Teacher"}
// {"Bengü Bilir": "Student", "Ayhan Bilir": "Teacher"}
