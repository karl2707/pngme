use std::error::Error;
use std::fmt;
use std::fmt::Formatter;


#[derive(Debug, PartialEq)]
pub enum PngMeError {
    ChunkTypeByteLengthError(usize),
    InvalidCharacterChunkType,
    CrcDoNotMatch(u32, u32),
    IncorrectFileHeader,
    ChunkTypeNotPresent
}

impl Error for PngMeError {}

impl fmt::Display for PngMeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PngMeError::ChunkTypeByteLengthError(length) => {
                write!(f, "provided type was {} bytes but required is 4", length)
            }
            PngMeError::InvalidCharacterChunkType => {
                write!(f, "provided header included invalid character")
            }
            PngMeError::CrcDoNotMatch(given, calculated) => {
                write!(f, "given crc {} does not match claculated {}", given, calculated)
            }
            PngMeError::IncorrectFileHeader => {
                write!(f, "provided file header was incorrect")
            }
            PngMeError::ChunkTypeNotPresent => {
                write!(f, "provided chunk type is not present in file")
            }
        }
    }
}