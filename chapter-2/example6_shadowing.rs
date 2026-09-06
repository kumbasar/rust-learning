 fn main() {
     let country = String::from("Turkey");
     let country_ref = &country;

     let country = 8;

     println!("{} or {}", country, country_ref);
 }

 /*
8 or Turkey
  */
