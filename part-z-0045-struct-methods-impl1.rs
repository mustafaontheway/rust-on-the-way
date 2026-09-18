fn main() {

    let member1 = set_member("Aybüke Güler".to_string(), 1287, 912, 0.04, true);

    member1.print_member_info();
}

#[derive(Debug)]
struct Member {

    name: String,
    member_id: u16,
    montly_payment_usd: u16,
    discount_rate: f32,
    is_continuing: bool
}

impl Member {

    fn print_member_info(self) {

        println!("Member name: {}", self.name);
        println!("Member ID: {}", self.member_id);
        println!("Member monthly payment $: {}", self.montly_payment_usd);
        println!("Member discount rate: {}", self.discount_rate);
        println!("Member is continuing?: {}", self.is_continuing);
    }
}

fn set_member(name: String, member_id: u16, montly_payment_usd: u16, discount_rate: f32, is_continuing: bool) -> Member {

    Member { name, member_id, montly_payment_usd, discount_rate, is_continuing }
}

fn set_discount_rate(m: &mut Member, new_rate: f32) {

    m.discount_rate = new_rate;
}

// Member name: Aybüke Güler
// Member ID: 1287
// Member monthly payment $: 912
// Member discount rate: 0.04
// Member is continuing?: true
