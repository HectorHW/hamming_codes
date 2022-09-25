use crate::bits::BitVector;

pub const CHECK_POSITIONS: [usize; 4] = [0, 1, 3, 7];
pub const CHECKSUM_POSITION: usize = 12;

pub type InfoVector = BitVector<8>;
pub type CodeVector = BitVector<13>;

fn compute_checks_inplace(buf: &mut [bool]) {
    buf[0] = buf[2] ^ buf[4] ^ buf[6] ^ buf[8] ^ buf[10];
    buf[1] = buf[2] ^ buf[5] ^ buf[6] ^ buf[9] ^ buf[10];
    buf[3] = buf[4] ^ buf[5] ^ buf[6] ^ buf[11];
    buf[7] = buf[8] ^ buf[9] ^ buf[10] ^ buf[11];
    buf[12] = compute_checksum(&buf[0..buf.len() - 1]);
}

pub fn encode_hamming(vec: &InfoVector) -> CodeVector {
    let mut buf = vec![false; 13];
    let info_digits = [3, 5, 6, 7, 9, 10, 11, 12];
    for (digit, index) in vec.iter().cloned().zip(info_digits.into_iter()) {
        buf[index - 1] = digit;
    }

    compute_checks_inplace(&mut buf);

    buf.try_into().unwrap()
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DecodeResult {
    Ok(InfoVector),
    ErrorFixed(InfoVector, usize),
    MultipleErrorsDetected,
}

fn extract_info_vector(vec: &CodeVector) -> InfoVector {
    vec.iter()
        .cloned()
        .enumerate()
        .filter_map(|(idx, item)| {
            if CHECK_POSITIONS.contains(&idx) || idx == CHECKSUM_POSITION {
                None
            } else {
                Some(item)
            }
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

fn compute_checksum(data: &[bool]) -> bool {
    data.iter().cloned().fold(false, |a, b| a ^ b)
}

pub fn decode_hamming(vec: &CodeVector) -> DecodeResult {
    let mut original = vec.as_ref().to_owned();
    let mut recomputed_copy = vec.as_ref().to_owned();
    compute_checks_inplace(&mut recomputed_copy);

    let errors = vec
        .iter()
        .cloned()
        .zip(recomputed_copy.iter().cloned())
        .enumerate()
        .filter_map(|(idx, (l, r))| if l != r { Some(idx) } else { None })
        .collect::<Vec<_>>();

    if errors.is_empty() {
        return DecodeResult::Ok(extract_info_vector(vec));
    }

    let errors_without_checksum = errors
        .into_iter()
        .filter(|&idx| idx != CHECKSUM_POSITION)
        .collect::<Vec<_>>();

    if errors_without_checksum.is_empty() {
        return DecodeResult::ErrorFixed(
            extract_info_vector(&original.try_into().unwrap()),
            CHECKSUM_POSITION,
        );
    }

    let flip_idx: usize =
        errors_without_checksum.iter().sum::<usize>() + errors_without_checksum.len() - 1;

    if flip_idx >= CHECKSUM_POSITION {
        return DecodeResult::MultipleErrorsDetected;
    }

    original.as_mut_slice()[flip_idx] = !original[flip_idx];

    let checksum = compute_checksum(&original);

    if checksum {
        DecodeResult::MultipleErrorsDetected
    } else {
        DecodeResult::ErrorFixed(extract_info_vector(&original.try_into().unwrap()), flip_idx)
    }
}

#[allow(clippy::inconsistent_digit_grouping)]
#[cfg(test)]
mod test {

    use crate::bits::convert_ascii;

    use super::{decode_hamming, encode_hamming, DecodeResult};
    #[test]
    fn encode_w() {
        let vector = vec![0, 1, 1, 1, 0, 1, 1, 1].try_into().unwrap();
        let result = encode_hamming(&vector);
        assert_eq!(
            result,
            vec![1, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0]
                .try_into()
                .unwrap()
        )
    }

    #[test]
    fn decode_with_one_error() {
        let vector = vec![1, 0, 0, 0, 0_, 1, 1, 1, 0, 1, 1, 1, 0]
            .try_into()
            .unwrap();
        let decoded = decode_hamming(&vector);

        assert_eq!(
            decoded,
            DecodeResult::ErrorFixed(vec![0, 1, 1, 1, 0, 1, 1, 1].try_into().unwrap(), 4)
        )
    }

    #[test]
    fn decode_with_two_errors() {
        let vector = vec![1, 0, 0, 1_, 0_, 1, 1, 1, 0, 1, 1, 1, 0]
            .try_into()
            .unwrap();
        let decoded = decode_hamming(&vector);

        assert_eq!(decoded, DecodeResult::MultipleErrorsDetected)
    }

    #[test]
    fn decode_with_no_errors() {
        let vector = vec![1, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0]
            .try_into()
            .unwrap();
        let decoded = decode_hamming(&vector);

        assert_eq!(
            decoded,
            DecodeResult::Ok(vec![0, 1, 1, 1, 0, 1, 1, 1].try_into().unwrap())
        )
    }

    use rstest::rstest;

    #[rstest]
    #[case('a', 0)]
    #[case('b', 5)]
    #[case('w', 6)]
    #[case('q', 8)]
    #[case('z', 9)]
    #[case('w', 11)]
    #[case('w', 12)]
    fn sigle_error_test(#[case] input: char, #[case] offset: usize) {
        let code = convert_ascii(input).unwrap();

        let mut encoded = encode_hamming(&code);

        encoded[offset] = !encoded[offset];
        assert_eq!(
            decode_hamming(&encoded),
            DecodeResult::ErrorFixed(code, offset)
        )
    }
}
