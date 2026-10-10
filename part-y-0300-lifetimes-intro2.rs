fn main() {

    let year: u16 = 2026;

    let this_year = year; // copy trait

    println!("{year} is {this_year}.");

    let city_mayor = "Aykan Köroğlu".to_string();

    drop(year); // warning: calls to `std::mem::drop` with a value that implements `Copy` does nothing    

    drop(city_mayor); // move occurs because `city_mayor` has type `String`, which does not implement the `Copy` trait

    {
        println!("This year is {year}.");
    }

    //println!("Mayor is {city_mayor}."); // Error

}

