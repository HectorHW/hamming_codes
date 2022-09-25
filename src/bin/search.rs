use hamming::{
    bits::break_n_bits,
    encoding::{decode_hamming, encode_hamming, DecodeResult, InfoVector},
};

fn main() {
    const NUM_ATTEMPTS: usize = 50_000;
    'outer: for candidate_word in 0u8..=255u8 {
        let bits: InfoVector = candidate_word.into();
        let reference_vector = encode_hamming(&bits);
        for _ in 0..NUM_ATTEMPTS {
            let mut target = reference_vector.clone();
            break_n_bits(&mut target, 3);
            //println!("attempting {} and {}", reference_vector, target);
            match decode_hamming(&target) {
                DecodeResult::Ok(m) => {
                    println!("found!\n{}\nand\n{}", reference_vector, target);
                    println!("original was {}, decoded after attack: {}", bits, m);
                    break 'outer;
                }
                DecodeResult::ErrorFixed(m, _) => {
                    println!(
                        "found with correction!\n{}\nand\n{}",
                        reference_vector, target
                    );
                    println!("original was {}, decoded after attack: {}", bits, m);
                    break 'outer;
                }
                DecodeResult::MultipleErrorsDetected => {}
            }
        }
    }
}
