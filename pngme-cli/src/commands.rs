use std::path::PathBuf;
use std::str::FromStr;
use clap::Parser;
use pngme_core;
use pngme_core::chunk::Chunk;
use pngme_core::chunk_type::ChunkType;
use pngme_core::png::Png;
use pngme_core::pngme_error::PngMeError;

#[derive(Parser, Debug)]
pub enum Args {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Parser, Debug)]
pub struct EncodeArgs {
    file_path: PathBuf,
    chunk_type: String,
    message: String,
    output_file: Option<PathBuf>,
}

impl EncodeArgs {
    pub fn encode(self) -> Result<(), PngMeError> {
        let mut png = Png::read_png_from_file(&self.file_path)?;

        let new_chunk_type = ChunkType::from_str(&self.chunk_type)?;
        let data:Vec<u8> = self.message.bytes().collect();

        let new_chunk = Chunk::new(new_chunk_type, data);

        png.append_chunk(new_chunk);

        match self.output_file {
            None => png.write_into_file(&self.file_path).unwrap(),
            Some(output_path) => png.write_into_file(&output_path).unwrap()
        }

        Ok(())
    }
}

#[derive(Parser, Debug)]
pub struct DecodeArgs {
    file_path: PathBuf,
    chunk_type: String,
}

impl DecodeArgs {
    pub fn decode(self) -> Result<(), PngMeError> {
        let png = Png::read_png_from_file(&self.file_path)?;
        match png.chunk_by_type(&self.chunk_type) {
            None => println!("No such chunk found."),
            Some(chunk) => println!("{}", chunk),
        };

        Ok(())
    }
}

#[derive(Parser, Debug)]
pub struct RemoveArgs {
    file_path: PathBuf,
    chunk_type: String,
    output_file: Option<PathBuf>,
}

impl RemoveArgs {
    pub fn remove(self) -> Result<(), PngMeError> {
        let mut png = Png::read_png_from_file(&self.file_path)?;
        let removed_chunk = png.remove_chunk(&self.chunk_type)?;
        println!("Removed {}", removed_chunk);

        match self.output_file {
            None => png.write_into_file(&self.file_path).unwrap(),
            Some(output_path) => png.write_into_file(&output_path).unwrap()
        }

        Ok(())
    }
}

#[derive(Parser, Debug)]
pub struct PrintArgs {
    file_path: PathBuf,
}

impl PrintArgs {
    pub fn print(self) -> Result<(), PngMeError> {
        let png = Png::read_png_from_file(&self.file_path)?;
        println!("{}", png);

        Ok(())
    }
}
