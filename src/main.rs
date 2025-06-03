use std::{io, process::Termination};

fn main() {
    //Ask the user for term_number to calciulate
    let mut term_number = String::new();
    io::stdin()
        .read_line(&mut term_number)
        .expect("Failed to read input");

    let term_number: i128 = term_number
        .trim()
        .parse::<i128>()
        .expect("Not valid number");

    //Calculate and print the value of nth_term of
    println!(
        "The {} th term of faboncci sequence is {}",
        term_number,
        nth_term(term_number)
    );
}

fn nth_term(term_mumber: i128) -> i128 {
    //Sign of the term_number
    let sign: i128 = term_mumber / term_mumber.abs();

    let mut first_term: i128 = 0;
    let mut second_term: i128 = 1 * sign;
    let mut third_term: i128 = 0;

    let mut counter: u64 = 0;
    let term_counter: u64 = term_mumber.abs() as u64; // For comparing against counter

    let nth_term: i128 = loop {
        while counter < term_counter {
            third_term = first_term + second_term;
            first_term = second_term;
            second_term = third_term;
            counter += 1;
        }
        if counter == term_counter {
            break third_term;
        }
    };
    nth_term
}
