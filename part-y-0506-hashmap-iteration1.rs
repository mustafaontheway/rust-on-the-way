use std::collections::HashMap;

fn main() {

    let mut salary_by_id: HashMap<&str, u16> = HashMap::new();

    salary_by_id.insert("au004296", 4000);

    salary_by_id.insert("mb004298", 4300);

    salary_by_id.insert("mk004307", 3700);

    for (_, sal) in &mut salary_by_id {

        *sal += 300;
    }

    println!("Updated salary info: {:?}",salary_by_id)
}

// Updated salary info: {"mb004298": 4600, "au004296": 4300, "mk004307": 4000}
