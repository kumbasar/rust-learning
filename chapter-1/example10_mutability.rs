fn main() {

    let _my_number = 8;

    //_my_number = 10 // cannot assign twice to immutable variable

    let mut my_number2 = 8; //Warning: this value is reassigned later and never used
    my_number2 = 1;
    println!("My number is {}", my_number2);

    let mut _my_variable = 8;
    //_my_variable = "Hello, World!"; //expected integer, found `&str`

}

/*
C:/Users/z003uz6j/.cargo/bin/cargo.exe run --color=always --package untitled --bin untitled --profile dev
warning: value assigned to `my_number2` is never read
 --> src\main.rs:7:26
  |
7 |     let mut my_number2 = 8;
  |                          ^ this value is reassigned later and never used
8 |     my_number2 = 1;
  |     -------------- `my_number2` is overwritten here before the previous value is read
  |
  = note: `#[warn(unused_assignments)]` (part of `#[warn(unused)]`) on by default

warning: `untitled` (bin "untitled") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.35s
     Running `target\debug\untitled.exe`
My number is 1

Process finished with exit code 0

 */
