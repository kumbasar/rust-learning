fn main() {

    let _my_float = 5.0; // default f64

    let my_float: f64 = 5.0;
    let my_other_float: f32 = 8.5;

    let _third_float = my_float + my_other_float as f64; // or expected `f64`, found `f32`

    let my_float_1: f32 = 5.0;  
    let my_other_float_1 = 8.5;

    let _third_float_1 = my_float_1 + my_other_float_1; // rust is smart

}
