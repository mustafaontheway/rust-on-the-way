fn main() {

    let her_name = "Aybüke Bilir";

    let _emp_aybuke = Employee { full_name: her_name, id: "ab004296", salary_usd: 3200};

    let _emp_ayhan = Employee { full_name: "Ayhan İyibilir", id: "aib004287", salary_usd: 3400};
}

struct Employee<'a> {

    full_name: &'a str,
    id: &'static str,
    salary_usd: u16 
}


