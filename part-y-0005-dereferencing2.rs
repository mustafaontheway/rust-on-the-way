fn main() {

}


pub fn get(v: &[u32], i: usize, default_val: u32) -> u32 {
    
    v.get(i).copied().unwrap_or(default_val)
}

pub fn get2(v: &[u32], i: usize, default_val: u32) -> u32 {
    
    if i < v.len() {
        *v.get(i).unwrap()
    } else {
        default_val
    }
}
