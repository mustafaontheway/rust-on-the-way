use std::collections::HashSet;

fn main() {

    let members = ["Ayhan Bilir", "Ayhan Bilir", "Ayhan Bilir", "Bengü Burada", "Kağan Güçlü", "Aygül Kızıl"];

    let mut unique_members = HashSet::from(members);

    println!("{:?}", unique_members.contains("Mustafa Büyükdereli"));

    println!("{:?}", unique_members.len());

    println!("{:?}", unique_members.get("Bengü Burada"));

    unique_members.remove("Kağan Güçlü");

    println!("{:?}", unique_members);
}

// false
// 4
// Some("Bengü Burada")
// {"Ayhan Bilir", "Bengü Burada", "Aygül Kızıl"}
