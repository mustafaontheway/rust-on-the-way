fn main() {

}

// pub fn parse_and_add(a: &str, b: &str) -> u32 {
    
//     let a_num = a.parse::<u32>();
//     let b_num = b.parse::<u32>();
    
//     if a_num.is_ok() && b_num.is_ok() {
//         a_num.unwrap() + b_num.unwrap()
//     } else {
//         0
//     }
// }

pub fn parse_and_add2(a: &str, b: &str) -> Result<u32, String> {
  
    let a_num = a.parse::<u32>();
    let b_num = b.parse::<u32>();

    if a_num.is_ok() && b_num.is_ok() {
        Ok(a_num.unwrap() + b_num.unwrap())
    } else {
        Err("Some error!".to_string())
    }
}
