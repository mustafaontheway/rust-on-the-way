fn main() {

    let _member_au = set_member(String::from("Ayhan Unutmaz"), 4296, 650, 0.08, true);
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
