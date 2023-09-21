use std::{str::FromStr, fmt::Display, error::Error, io::Read};
use std::str;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ChunkType {
    bytes: [u8; 4],
}

#[derive(Debug, PartialEq, Eq)]
enum ChunkLayout {
    AncillaryBit = 0,
    PrivateBit = 1,
    ReservedBit = 2,
    SafeToCopyBit = 3,
}

impl ChunkType { 
    const BITMAP: u8 = 32;

    pub fn bytes(&self) -> [u8; 4] {
        self.bytes.clone()
    }
    pub fn is_critical(&self) -> bool {
        self.bytes[ChunkLayout::AncillaryBit as usize] & Self::BITMAP != Self::BITMAP
    }
    pub fn is_public(&self) -> bool {
        self.bytes[ChunkLayout::PrivateBit as usize] & Self::BITMAP != Self::BITMAP
    }
    pub fn is_safe_to_copy(&self) -> bool {
        self.bytes[ChunkLayout::SafeToCopyBit as usize] & Self::BITMAP == Self::BITMAP
    }
    pub fn is_valid(&self) -> bool {
        self.are_bytes_valid() && self.is_reserved_bit_valid()
    }
    pub fn is_reserved_bit_valid(&self) -> bool {
        self.bytes[ChunkLayout::ReservedBit as usize] & Self::BITMAP != Self::BITMAP
    }

    fn are_bytes_valid(&self) -> bool {
        self.bytes
            .iter()
            .all(|&x| (x >= 65 && x <= 90) || (x >= 97 && x <= 122))
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = ChunkTypeError;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        let chunk_type = Self{ bytes: value };
        if !chunk_type.are_bytes_valid() {
            return Err(ChunkTypeError::InvalicCharater);
        }
        Ok(chunk_type)
    }
}

impl FromStr for ChunkType {
    type Err = ChunkTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes: [u8; 4] = match s.as_bytes().try_into() {
            Ok(bytes) => bytes,
            Err(_e) => return Err(ChunkTypeError::ByteLengthError(s.as_bytes().len())),
        };
        
        Ok(Self::try_from(bytes)?)
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", str::from_utf8(&self.bytes).unwrap())
    }
}

#[derive(Debug, PartialEq)]
pub enum ChunkTypeError {
    ByteLengthError(usize),
    InvalicCharater,
}

impl Error for ChunkTypeError {}

impl Display for ChunkTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ByteLengthError(len) => write!(f, "chunk type expected length 4, provided {}", len),
            Self::InvalicCharater => write!(f, "incorrect charater in chunk type"),
        }
    }
}

#[cfg(test)]
mod chunk_type_tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_illegal_bytes() {
        let actual = ChunkType::try_from([0, 0, 0,0 ]);
        assert!(actual.is_err());
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

    
}

#[cfg(test)]
mod chunk_type_traits_tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_from_too_long_string() {
        let s = "RUSTT";
        let chunk = ChunkType::from_str(s).err().unwrap();
        assert_eq!(chunk, ChunkTypeError::ByteLengthError(s.len()));
    }

    #[test]
    pub fn test_chunk_from_too_short_string() {
        let s = "RUS";
        let chunk = ChunkType::from_str(s).err().unwrap();
        assert_eq!(chunk, ChunkTypeError::ByteLengthError(s.len()));
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
