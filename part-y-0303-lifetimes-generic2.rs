fn main() {

    let names_static: [&'static str; 4] = ["Mustafa", "Aygün", "Aykan", "Ayben"];

    let cities = ["İzmir", "Ankara", "İstanbul", "Bursa"];

    let names_string = ["Bengü".to_string(), "Biricik".to_string(), "Birkan".to_string(), "Bilge".to_string()];

    let _r1 = get_first(&names_static);

    let _r2 = get_first(&cities);

    let _r3 = get_first2(&names_string);
}

fn get_first(list: &[&'static str]) -> &'static str {

   &list[0]
}

fn get_first2<'a>(list: &'a[String]) -> &'a str {

   &list[0]
}
