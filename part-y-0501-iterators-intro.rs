fn main() {
  
    let ages = vec![4u8, 8, 15, 16, 23, 42];

    for age in &ages { 
      
        print!("{age} ");
    }

    println!(" ");

    // we can think age in ages.iter == age in &ages

   for age in ages.iter() {
     
        print!("{age} ");
    }

    println!(" ");

    // we can think age in ages.into_iter == age in ages
    // if we're sure we won't use ages later, let's eat it.

    for age in ages.into_iter() {

      print!("{age} ");
    }

    //println!("{:?}", ages); // Error! Remember we eated! -> clear the memory!
}
