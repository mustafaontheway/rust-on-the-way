use std::collections::HashMap;

fn main() {

    let member1_name = String::from("Ayhan Bilir");

    let member2_name = String::from("Bengü Bilir");

    let mut family_members_jobs: HashMap<&str, &str,> = HashMap::new();

    family_members_jobs.insert(&member1_name, "Teacher");

    family_members_jobs.insert(&member2_name, "Nurse");

    family_members_jobs.entry("Bumin").or_insert("Idle");

    family_members_jobs.entry("Aygün").or_insert("Idle");

    println!("{:?}", family_members_jobs);

    //family_members_jobs["Aygün"] = "Doctor";

    family_members_jobs.insert("Aygün", "Doctor");

    println!("{:?}", family_members_jobs);
}

// {"Bengü Bilir": "Nurse", "Aygün": "Idle", "Bumin": "Idle", "Ayhan Bilir": "Teacher"}
// {"Ayhan Bilir": "Teacher", "Bengü Bilir": "Nurse", "Bumin": "Idle", "Aygün": "Doctor"}
