use std::collections::HashSet;

fn main() {

    let members = ["Ayhan Bilir", "Ayhan Bilir", "Ayhan Bilir", "Bengü Burada", "Kağan Güçlü", "Aygül Kızıl"];

    let unique_members = HashSet::from(members);

    let students = HashSet::from(["Mustafa Büyükdereli", "Aygül Kızıl"]);

    println!("{:?}", unique_members.union(&students));

    println!("{:?}", unique_members.difference(&students));
}

// ["Aygül Kızıl", "Ayhan Bilir", "Kağan Güçlü", "Bengü Burada", "Mustafa Büyükdereli"]
// ["Ayhan Bilir", "Kağan Güçlü", "Bengü Burada"]
