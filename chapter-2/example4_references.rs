fn main() {
    let country = String::from("Turkey"); //owned
    let ref_one = &country; // a reference to the str
    let ref_two = &country; // a reference to the str

    println!("{}, {}, {}", ref_one, ref_two, country);

    let country1 = return_str();
    println!("{}", country1);
}

fn return_str() ->  String {
    let country = String::from("Turkey");
    country
}

/*
Turkey, Turkey, Turkey
Turkey
 */
