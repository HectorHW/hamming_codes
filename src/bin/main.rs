use std::borrow::Cow;

use colored::*;
use encoding_rs::WINDOWS_1251;
use hamming::{
    bits::{break_one_bit, break_two_bits},
    encoding::{
        decode_hamming, encode_hamming, CodeVector, InfoVector, CHECKSUM_POSITION, CHECK_POSITIONS,
    },
};

use hamming::encoding::DecodeResult;
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

fn print_with_highlight(vec: &[bool], positions: &[usize]) {
    for (i, digit) in vec.iter().cloned().enumerate() {
        if positions.contains(&i) {
            print!("{}", (digit as u8).to_string().red());
        } else {
            print!("{}", (digit as u8).to_string().white());
        }
    }
    println!()
}

fn into_cp1251_bits(letter: char) -> InfoVector {
    let string_buf = String::from(letter);
    let buf = WINDOWS_1251.encode(&string_buf).0.to_vec();
    debug_assert_eq!(buf.len(), 1);
    InfoVector::from(buf[0])
}

fn from_cp1251(buf: &[u8]) -> Cow<'_, str> {
    WINDOWS_1251.decode(buf).0
}

fn main() {
    let alphabet = ['л', 'м', 'н', 'о', 'п'];

    println!("коды символов:");

    let char_codes = alphabet
        .iter()
        .cloned()
        .map(into_cp1251_bits)
        .collect::<Vec<_>>();

    for (code, &letter) in char_codes.iter().cloned().zip(alphabet.iter()) {
        println!("{letter} => {code}");
    }

    let encoded = char_codes.iter().map(encode_hamming).collect::<Vec<_>>();

    println!("закодированная версия:");

    for (item, original) in encoded.iter().zip(char_codes.iter()) {
        print!("{original} => ");
        display_encoded_vector(item);
    }

    println!("декодирование:");
    for item in &encoded {
        if let DecodeResult::Ok(decoded) = decode_hamming(item) {
            let buf = [u8::from(&decoded)];
            println!("{} => {} ({})", item, decoded, from_cp1251(&buf));
        } else {
            unreachable!()
        }
    }

    println!("вносим по одной ошибке:");

    for mut item in encoded.clone() {
        break_one_bit(&mut item);
        match decode_hamming(&item) {
            hamming::encoding::DecodeResult::ErrorFixed(msg, err_idx) => {
                print_with_highlight(&item, &[err_idx]);
                println!("{}", format!("ошибка в позиции {err_idx}").yellow());
                let char_buf = [u8::from(&msg)];
                println!("сообщение: {} ({})", msg, from_cp1251(&char_buf));
            }
            _ => unreachable!(),
        }
    }

    println!("после внесения двух ошибок:");

    for mut item in encoded {
        let errors = break_two_bits(&mut item);
        print_with_highlight(&item, &errors);
        match decode_hamming(&item) {
            DecodeResult::MultipleErrorsDetected => {
                println!("{}", "произошло как минимум 2 ошибки.".red())
            }
            _ => unreachable!(),
        }
    }
}
