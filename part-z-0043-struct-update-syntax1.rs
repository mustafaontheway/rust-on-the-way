fn main() {

    let member1 = Member {

        name: "Ayhan Bilir".to_string(),
        member_id: 4296,
        montly_payment_usd: 650,
        discount_rate: 0.08,
        is_continuing: true
    };

    let _member2 = Member {
        
        name: "Hakan Bilir".to_string(),
        member_id: 4234,
        ..member1
    };
}

#[derive(Debug)]
struct Member {

    name: String,
    member_id: u16,
    montly_payment_usd: u16,
    discount_rate: f32,
    is_continuing: bool
}

