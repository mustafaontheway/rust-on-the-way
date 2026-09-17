fn main() {

    let mut member_au = Member {
    
        name: String::from("Ayhan Unutmaz"),
        member_id: 4296,
        montly_payment_usd: 650,
        discount_rate: 0.08,
        is_continuing: true
    };

    member_au.discount_rate = 0.12;

    println!("{:?}", member_au)

}

#[derive(Debug)]
struct Member {

    name: String,
    member_id: u16,
    montly_payment_usd: u16,
    discount_rate: f32,
    is_continuing: bool
}

// Member { name: "Ayhan Unutmaz", member_id: 4296, montly_payment_usd: 650, discount_rate: 0.12, is_continuing: true }
