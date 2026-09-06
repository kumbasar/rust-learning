fn main() {
    let name1 = String::from("Jack");
    let name2 = String::from("John");
    let mut my_vec = Vec::new();
    my_vec.push(name1);
    my_vec.push(name2);

    println!("{:?}", my_vec);

    let mut _my_vec2: Vec<String> = Vec::new();

    let vec_of_ten = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let three_to_five = &vec_of_ten[2..5];
    let start_at_two = &vec_of_ten[1..];
    let end_at_five = &vec_of_ten[..5];
    let everything = &vec_of_ten[..];

    println!("Three to five: {:?},
start at two: {:?}
end at five: {:?}
everything: {:?}", three_to_five, start_at_two, end_at_five, everything);

    let mut num_vec = Vec::with_capacity(8);
    println!("{}", num_vec.capacity());
    num_vec.push('a');
    println!("{}", num_vec.capacity());
    num_vec.push('a');
    num_vec.push('a');
    num_vec.push('a');
    println!("{}", num_vec.capacity());
    num_vec.push('a');
    println!("{}", num_vec.capacity());

     let my_vec_2: Vec<u8> = [1, 2, 3].into();
}

/*
warning: unused variable: `my_vec_2`
  --> src\main.rs:34:14
   |
34 |          let my_vec_2: Vec<u8> = [1, 2, 3].into();
   |              ^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_my_vec_2`
   |
   = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: `untitled` (bin "untitled") generated 1 warning (run `cargo fix --bin "untitled" -p untitled` to apply 1 suggestion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target\debug\untitled.exe`
["Jack", "John"]
Three to five: [3, 4, 5],
start at two: [2, 3, 4, 5, 6, 7, 8, 9, 10]
end at five: [1, 2, 3, 4, 5]
everything: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
8
8
8
8

 */
