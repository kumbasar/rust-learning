fn main() {

    let color1 = "red";
    let color2 = "blue";
    let color3 = "green";

    println!("{color1}, {color2} and {color3}");
    println!("{}, {} and {}", color1, color2, color3);

    let my_number = {
        let second_number = 8;
        second_number + 9
    };

    println!("My number is {}", my_number);

    let my_number2 = {
        let second_number = 9;
        second_number + 8; // warning:  the arithmetic operation produces a value
    };

    println!("My number is {:?}", my_number2);

}

/*
C:/Users/z003uz6j/.cargo/bin/cargo.exe run --color=always --package untitled --bin untitled --profile dev
warning: unused arithmetic operation that must be used
  --> src\main.rs:19:9
   |
19 |         second_number + 8; // warning:  the arithmetic operation produces a value
   |         ^^^^^^^^^^^^^^^^^ the arithmetic operation produces a value
   |
   = note: `#[warn(unused_must_use)]` (part of `#[warn(unused)]`) on by default
help: use `let _ = ...` to ignore the resulting value
   |
19 |         let _ = second_number + 8; // warning:  the arithmetic operation produces a value
   |         +++++++

warning: `untitled` (bin "untitled") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running `target\debug\untitled.exe`
red, blue and green
red, blue and green
My number is 17
My number is ()

Process finished with exit code 0

 */
