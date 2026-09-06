fn main() {
    let my_variable: i32;
    {
        let calculate_result = {
            32
        };
        my_variable = calculate_result;

        print!("{}",my_variable);

    }
}

/*
32
 */
