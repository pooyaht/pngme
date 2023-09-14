use crate::chunk_type::ChunkType;
use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "PNGme",
    version = "1.0",
    author = "pooya",
    about = "Allows encoding and decoding messages into PNG files"
)]
pub struct PNGme {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Args)]
pub struct EncodeArgs {
    /// PNG file to read from
    pub file_path: PathBuf,
    /// Chunk type
    pub chunk_type: ChunkType,
    /// Message to encode
    pub hidden_message: String,
    /// File path to encode into
    pub output_path: Option<PathBuf>,
}

#[derive(Args)]
pub struct DecodeArgs {
    /// PNG file to read from
    pub file_path: PathBuf,
    /// Chunk type
    pub chunk_type: ChunkType,
}

#[derive(Args)]
pub struct RemoveArgs {
    /// PNG file to read from
    pub file_path: PathBuf,
    /// Chunk type
    pub chunk_type: ChunkType,
}

#[derive(Args)]
pub struct PrintArgs {
    /// PNG file to read from
    pub file_path: PathBuf,
}
