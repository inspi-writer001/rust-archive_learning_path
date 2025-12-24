fn main() {
    let number_to_far = farenheight_to_celcius(12.9);
    println!("{}", number_to_far);

    println!("{}", celcius_to_farenheight(number_to_far));
}

pub fn farenheight_to_celcius(value: f32) -> f32 {
    // formula 0C = 0F - 32 / 1.8
    // F = C * 1.8 + 32
    let formula = (value * 1.8) + 32.0;
    formula
}

pub fn celcius_to_farenheight(value: f32) -> f32 {
    // formula 0C = 0F - 32 / 1.8
    // F = C * 1.8 + 32
    let formula = (value - 32.0) / 1.8;
    formula
}
