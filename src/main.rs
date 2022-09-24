use bits::convert_ascii;
use colored::*;
use encoding::{decode_hamming, encode_hamming, CodeVector, CHECKSUM_POSITION, CHECK_POSITIONS};
use rand::{thread_rng, Rng};
mod bits;
mod encoding;

fn display_encoded_vector(vec: &CodeVector) {
    for (position, digit) in vec.iter().cloned().enumerate() {
        if CHECK_POSITIONS.contains(&position) {
            print!("{}", (digit as u8).to_string().purple());
        } else if position == CHECKSUM_POSITION {
            print!("{}", (digit as u8).to_string().green());
        } else {
            print!("{}", (digit as u8).to_string().white());
        }
    }
    println!()
}

fn print_vector(vec: &[bool]) {
    vec.iter()
        .for_each(|&digit| print!("{}", (digit as u8).to_string().white()));
    println!()
}

fn print_with_highlight(vec: &[bool], position: usize) {
    for (i, digit) in vec.iter().cloned().enumerate() {
        if i == position {
            print!("{}", (digit as u8).to_string().red());
        } else {
            print!("{}", (digit as u8).to_string().white());
        }
    }
    println!()
}

fn main() {
    let bits = convert_ascii('w').unwrap();
    let mut coded = encode_hamming(&bits);
    display_encoded_vector(&coded);

    match thread_rng().gen_range(1..=3) {
        1 => {
            println!("pay no attention to me")
        }
        2 => {
            coded[5] = !coded[5];
            println!("something bad happened!")
        }
        3 => {
            coded[5] = !coded[5];
            coded[7] = !coded[7];
            println!("something reeeally bad happened!")
        }
        _ => unreachable!(),
    }
    println!("now message is:");
    print_vector(&coded);

    match decode_hamming(&coded) {
        encoding::DecodeResult::Ok(message) => print_vector(&message),
        encoding::DecodeResult::ErrorFixed(message, idx) => print_with_highlight(&message, idx),
        encoding::DecodeResult::MultipleErrorsDetected => println!("multiple errors occured"),
    }
}
