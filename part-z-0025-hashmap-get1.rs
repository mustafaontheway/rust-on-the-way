use std::collections::HashMap;

fn main() {

    let member1_name = String::from("Ayhan Bilir");

    let member2_name = String::from("Bengü Bilir");

    let mut family_members_jobs: HashMap<&str, &str,> = HashMap::new();

    family_members_jobs.insert(&member1_name, "Teacher");

    family_members_jobs.insert(&member2_name, "Nurse");

    println!("{:?}", family_members_jobs.get("Bengü Bilir")); 

    println!("{:?}", family_members_jobs.get("Bengü Bilir").unwrap()); 

    println!("{:?}", family_members_jobs.get("Kağan Bilir").copied().unwrap_or("Unknown"));
}

// Some("Nurse")
// "Nurse"
// "Unknown"
