fn main() {

    let mut member1 = set_member("Aybüke Güler".to_string(), 1287, 912, 0.04, true);

    println!("{:?}", member1);

    set_discount_rate(&mut member1, 0.07);

    println!("Member 1 new discount rate: {}", member1.discount_rate);

    println!("{:?}", member1);
}

#[derive(Debug)]
struct Member {

    name: String,
    member_id: u16,
    montly_payment_usd: u16,
    discount_rate: f32,
    is_continuing: bool
}

fn set_member(name: String, member_id: u16, montly_payment_usd: u16, discount_rate: f32, is_continuing: bool) -> Member {

    Member { name, member_id, montly_payment_usd, discount_rate, is_continuing }
}

fn set_discount_rate(m: &mut Member, new_rate: f32) {

    m.discount_rate = new_rate;
}

// Member { name: "Aybüke Güler", member_id: 1287, montly_payment_usd: 912, discount_rate: 0.04, is_continuing: true }
// Member 1 new discount rate: 0.07
// Member { name: "Aybüke Güler", member_id: 1287, montly_payment_usd: 912, discount_rate: 0.07, is_continuing: true }
