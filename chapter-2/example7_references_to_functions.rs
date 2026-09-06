fn print_country(country_name: &String)  {
    println!("{country_name}");
}

/*
fn main() {
    let country = String::from("Austria");
    let country =print_country(country);  //value moved here
    print_country(country); //already died
}
*/

fn main() {
    let country = String::from("Turkey");
    print_country(&country);
    print_country(&country);

    let mut country1 = String::from("Turkey");

    add_hungary(&mut country1);

    let country2 = String::from("Germany");
    add_hungary2(country2)
}

fn add_hungary(country: &mut String) {
    country.push_str("-Hungary");
    println!("{country}");
}

fn add_hungary2(mut country: String) {
    country.push_str("-Hungary");
    println!("{country}");
}

/*
Turkey
Turkey
Turkey-Hungary
Germany-Hungary

 */
