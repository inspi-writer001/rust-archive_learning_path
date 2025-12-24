fn append_vector(current_vec: &mut Vec<u32>, to_append: u32) {
    current_vec.push(to_append);
}

fn divide(a: f32, b: f32) -> Result<f32, String> {
    if b == 0.0 {
        return Err("Division by Zero".to_string());
    }
    return Ok(a / b);
}

fn print_optional_value(value_a: f32, value_b: f32) -> Result<f32, String> {
    let quotient = divide(value_a, value_b)?;
    println!("{:?}", quotient);
    return Ok(quotient);
}

fn main() {
    let mut vec_2: Vec<u32> = Vec::new();
    //  let mut vec_2 = vec![3, 5];
    append_vector(&mut vec_2, 24);
    append_vector(&mut vec_2, 49);
    append_vector(&mut vec_2, 278);
    append_vector(&mut vec_2, 245);
    println!("{:?}", vec_2);

    let indexed = vec_2.get(6);
    println!("{:?}", indexed);
    //  vec_2[6] = 23;

    let value_from_function = print_optional_value(2.0, 6.9);
    let value_from_function2 = print_optional_value(2.0, 0.0);

    println!("main: {:?}", value_from_function.unwrap());
    println!("main: {:?}", value_from_function2.unwrap_err());
}
