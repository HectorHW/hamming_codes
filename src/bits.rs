use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BitVector<const SIZE: usize>(Vec<bool>);

impl<const N: usize> Deref for BitVector<N> {
    type Target = [bool];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl<const N: usize> DerefMut for BitVector<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut_slice()
    }
}

impl<const N: usize> TryFrom<Vec<bool>> for BitVector<N> {
    type Error = ();

    fn try_from(value: Vec<bool>) -> Result<Self, Self::Error> {
        if value.len() != N {
            return Err(());
        }
        Ok(BitVector(value))
    }
}

impl<const N: usize> TryFrom<Vec<u8>> for BitVector<N> {
    type Error = ();

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.len() != N {
            return Err(());
        }
        value
            .into_iter()
            .map(|v| v != 0)
            .collect::<Vec<bool>>()
            .try_into()
    }
}

pub fn convert_ascii(value: char) -> Result<BitVector<8>, ()> {
    if !value.is_ascii() {
        return Err(());
    }

    let mut byte = value as u8;
    let mut buf = vec![];
    for _ in 0..8 {
        buf.push(byte & 128 != 0);
        byte <<= 1;
    }
    Ok(BitVector(buf))
}

#[cfg(test)]
mod test {
    use super::convert_ascii;

    #[test]
    fn convert_w() {
        let result = convert_ascii('w').unwrap();
        assert_eq!(
            result.as_ref(),
            &vec![false, true, true, true, false, true, true, true]
        )
    }
}
