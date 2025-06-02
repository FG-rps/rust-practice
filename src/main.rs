use std::{io, ops::Index};

fn main() {
    let a = [1, 2, 3, 4, 5, 76];

    println!("Please Enter the index of array you want to access");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read input");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the elemnt at {index} is {element}");
}
