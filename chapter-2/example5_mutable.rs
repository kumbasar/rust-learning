fn main() {
    let mut my_number = 8;
    let num_ref = &mut my_number;
    *num_ref += 10;
    println!("num ref: {}", num_ref);

    let second_number = 800;
    let triple_ref = &&&second_number;
    println!("Are they equal? {}", second_number == ***triple_ref);


    let mut number = 10;
    let number_change = &mut number;
    *number_change += 10; // mutable reference
    let number_ref = &number; // immutable reference
    println!("{}", number_ref);


}

/*
num ref: 18
Are they equal? true
20
 */
