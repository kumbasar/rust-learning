fn main() {  //fn: function
    println!("Hello, world!");

    let my_float = 5.0;
    {
        let my_other_float = 8.5;
        println!("my_other_float: {}", my_other_float);
        println!("my_float: {}", my_float);
    }
    println!("my_float: {}", my_float);
    // println!("my_other_float: {}", my_other_float); //not found in this scope

    println!("Hello, world number {}!", 8);

    println!("Hello, world number {} {}!", 8, 9);

    println!("Hello, world number {}!", give_me_number());

    println!("Number: {}", give_me_other_number());

    let some_number = 10;
    let some_other_number = 2;

    multiply(some_number, some_other_number);
    multiply(8,9);

    let result = multiply2(8, 9);
    println!("The result is {}", result);

}

fn give_me_number() -> i32 {
    8  // 8; => expected `i32`, found `()`
}

fn give_me_other_number() -> i32 {
    return 5; // or simply 5
}

fn multiply(number_one: i32, number_two: i32) {

    let result = number_one * number_two;
    println!("{} times {} is {}", number_one, number_two, result);
}

fn multiply2(number_one: i32, number_two: i32) -> i32 {
    let result = number_one * number_two;
    result
}

/*

Hello, world!
my_other_float: 8.5
my_float: 5
my_float: 5
Hello, world number 8!
Hello, world number 8 9!
Hello, world number 8!
Number: 5
10 times 2 is 20
8 times 9 is 72
The result is 72

Process finished with exit code 0

 */
