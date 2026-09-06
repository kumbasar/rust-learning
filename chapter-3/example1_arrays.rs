fn main() {
    let _array1 = ["One", "Two"];
    let _array2 = ["One", "Two", "Three"];

    let my_array = ["a"; 5];
    println!("{:?}", my_array);

    println!("{:?}", b"Hello, world!");

    let my_numbers = [0, 10, -20];
    println!("{}", my_numbers[1]);

    let array_of_ten = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let two_of_five = &array_of_ten[2..5];
    let start_at_one = &array_of_ten[1..];
    let end_at_five = &array_of_ten[..5];
    let everything = &array_of_ten[..];

    println!("{two_of_five:?}, {start_at_one:?}, {end_at_five:?}, {everything:?}");

}

/*
["a", "a", "a", "a", "a"]
[72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33]
10
[2, 3, 4], [1, 2, 3, 4, 5, 6, 7, 8, 9], [0, 1, 2, 3, 4], [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]

 */
