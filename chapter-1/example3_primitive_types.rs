/*
fn main() {
    let my_number = 100;
    println!("{}", my_number as char);
}
*/

fn main() {
    let my_number = 100;

    println!("{}", my_number);
    println!("{}", my_number as u8 as char);
    
    let my_number1: u8 = 100;
    println!("{}", my_number1 as char);
    
    let my_number2 = 256;
    println!("{}", my_number2 as u8);
    
    let my_number3 = 600;
    println!("{}", my_number3 as u8);
    
    println!("Size of a char: {}", std::mem::size_of::<char>());
    
    println!("Size of a: {}", "a".len());
    println!("Size of ğ: {}", "ğ".len());
    
    let str1 = "Hello!";
    println!("str1 is {} bytes", str1.len());
    
    println!("{:?}", "a".as_bytes());
    println!("{:?}", "ğ".as_bytes());
    
    println!("str1 is {} bytes and also {} characters.", str1.len(), str1.chars().count());
    
}

/* Output
100
d
d
0
88
Size of a char: 4
Size of a: 1
Size of ğ: 2
str1 is 6 bytes
[97]
[196, 159]
str1 is 6 bytes and also 6 characters.
*/
