fn main() {

    let my_num: u8 = 17;

    {
        println!("My number (print 2): {my_num}");

        let mut  my_num: i64 = -5_000_000;

        println!("My number (print 3): {my_num}");

        my_num = 600;

        println!("My number (print 4): {my_num}");
    }

    println!("My number (print 1): {my_num}");
}

// My number (print 2): 17
// My number (print 3): -5000000
// My number (print 4): 600
// My number (print 1): 17
