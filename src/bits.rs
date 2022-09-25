use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

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

impl From<u8> for BitVector<8> {
    fn from(mut item: u8) -> Self {
        let mut buf = vec![];
        for _ in 0..8 {
            buf.push(item & 128 != 0);
            item <<= 1;
        }
        BitVector(buf)
    }
}

impl From<&BitVector<8>> for u8 {
    fn from(val: &BitVector<8>) -> Self {
        let mut result = 0u8;
        let mut power = 128;
        for &bit in &val[..] {
            if bit {
                result += power;
            }
            power >>= 1;
        }
        result
    }
}

impl<const N: usize> Display for BitVector<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for &item in &self[..] {
            write!(f, "{}", item as u8)?;
        }
        Ok(())
    }
}

#[cfg(test)]
pub mod test {
    use super::BitVector;

    pub fn convert_ascii(value: char) -> Result<BitVector<8>, ()> {
        if !value.is_ascii() {
            return Err(());
        }

        let byte = value as u8;

        Ok(BitVector::from(byte))
    }

    #[test]
    fn convert_w() {
        let result = convert_ascii('w').unwrap();
        assert_eq!(
            result.as_ref(),
            &vec![false, true, true, true, false, true, true, true]
        )
    }
}
