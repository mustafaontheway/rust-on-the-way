fn main() {

   let age: u8 = 17;

   println!("Pointer: {:p}", &age);

   // Numeric value

   let numeric_pointer_value = &age as *const u8 as usize; // *const -> it tells us "immutable raw pointer"

   println!("Numeric pointer: {}", numeric_pointer_value);

   // Macro

   let raw_pointer = std::ptr::addr_of!(age);

   println!("Pointer: {:?}", raw_pointer);
}

// Pointer: 0x64006ff667
// Numeric pointer: 429504067175
// Pointer: 0x64006ff667
