use core::f64;
use std::{
    io::{self, Read},
    result,
};

fn main() {
    let mut temprature_in_farenheit = String::new();

    io::stdin()
        .read_line(&mut temprature_in_farenheit)
        .expect("Failed to read user input");

    let temprature_in_farenheit: f64 = temprature_in_farenheit
        .trim()
        .parse::<f64>()
        .expect("Failed to parse as 64 bit float");

    println!("{}", convert_to_celcius(temprature_in_farenheit));
}

fn convert_to_celcius(farenheit: f64) -> f64 {
    let celcius: f64 = (farenheit / 1.8) - 32.0;
    celcius
}
