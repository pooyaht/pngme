use anyhow::{anyhow, Error, Result};
use std::{
    fmt::Display,
    str::{self, FromStr},
};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ChunkType([u8; 4]);

impl ChunkType {
    pub(super) fn bytes(&self) -> [u8; 4] {
        self.0
    }
    pub(super) fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }
    pub(super) fn is_critical(&self) -> bool {
        !self.is_nbit_set(*self.0.first().unwrap(), 5)
    }
    pub(super) fn is_public(&self) -> bool {
        !self.is_nbit_set(*self.0.get(1).unwrap(), 5)
    }
    pub(super) fn is_reserved_bit_valid(&self) -> bool {
        !self.is_nbit_set(*self.0.get(2).unwrap(), 5)
    }
    pub(super) fn is_safe_to_copy(&self) -> bool {
        self.is_nbit_set(*self.0.get(3).unwrap(), 5)
    }
    fn is_nbit_set(&self, byte: u8, bit: u8) -> bool {
        (byte >> bit) & 1 == 1
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.0.to_vec()).unwrap())
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(value: [u8; 4]) -> Result<Self, Error> {
        match value.iter().find(|c| !c.is_ascii_alphabetic()) {
            None => Ok(Self(value)),
            Some(char) => Err(anyhow!("{char} is not ascii_alphabetic")),
        }
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.as_bytes().iter().find(|c| !c.is_ascii_alphabetic()) {
            None => Ok(Self(s.as_bytes().to_vec().try_into().unwrap())),
            Some(char) => Err(anyhow!("{char} is not ascii_alphabetic")),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;
    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());
        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
