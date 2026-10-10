fn main() {

    let city_mayor = "Aykan Köroğlu".to_string();

    let mayor = city_mayor.clone();

    let cm = &city_mayor;

    let our_mayor = city_mayor;

    //println!("{city_mayor}"); // error[E0382]: borrow of moved value: `city_mayor

    {
        println!("{mayor}"); // Aykan Köroğlu

        drop(our_mayor);
    }

    //println!("{our_mayor}"); // error[E0382]: borrow of moved value: `our_mayor                                                                                                                                     
}

