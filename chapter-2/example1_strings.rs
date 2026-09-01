fn main() {
    let name = "volkan";
    let surname = String::from("kumbasar"); //owned string

    let emoji = "😂";

    println!("Hello, {} {}! {emoji}", name, surname);


    let size_of_string = std::mem::size_of::<String>();
    let size_of_i8 = std::mem::size_of::<i8>();
    let size_of_f64 = std::mem::size_of::<f64>();
    let size_of_jaurim = std::mem::size_of_val("자우림");
    let size_of_adrian = std::mem::size_of_val("Adrian Fahrenheit Țepeș");

    println!("A String is Sized and always {size_of_string} bytes.");
    println!("An i8 is Sized and always {size_of_i8} bytes.");
    println!("An f64 is always Sized and {size_of_f64} bytes.");
    println!("But a &str is not Sized: '자우림' is {size_of_jaurim} bytes.");
    println!("And 'Adrian Fahrenheit Țepeș' is {size_of_adrian} bytes - not Sized.");

    let str_var: &str = "hello";
    let str_var2 = str_var.to_string(); // to String
    println!("{}", str_var2);

    let name = "Billybrobby"; //&str
    let country = "USA"; //&str
    let home = "Korea"; //&str
    let together = format!("I am {name} from {country} but I live in {home}"); //String
    println!("{}", together);

    let my_string: String = "Try to make this a String".into(); // same as FROM::
    println!("{}", my_string);

}

/*
C:/Users/z003uz6j/.cargo/bin/cargo.exe run --color=always --package untitled --bin untitled --profile dev
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.15s
     Running `target\debug\untitled.exe`
Hello, volkan kumbasar! 😂
A String is Sized and always 24 bytes.
An i8 is Sized and always 1 bytes.
An f64 is always Sized and 8 bytes.
But a &str is not Sized: '자우림' is 9 bytes.
And 'Adrian Fahrenheit Țepeș' is 25 bytes - not Sized.
hello
I am Billybrobby from USA but I live in Korea
Try to make this a String

Process finished with exit code 0

 */
