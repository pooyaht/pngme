use clap::Parser;
use std::fs;

mod chunk;
mod chunk_type;
mod cli;
mod png;
fn main() -> Result<(), anyhow::Error> {
    let cli = cli::PNGme::parse();
    match cli.command {
        cli::Commands::Encode(args) => {
            let mut png = png::Png::from_file(args.file_path.clone())?;
            png.append_chunk(chunk::Chunk::new(
                args.chunk_type,
                args.hidden_message.into(),
            ));
            let out_file = args.output_path.unwrap_or(args.file_path);
            let _ = fs::write(out_file, png.as_bytes());
        }
        cli::Commands::Decode(args) => {
            let png = png::Png::from_file(args.file_path)?;
            match png.chunk_by_type(args.chunk_type.to_string().as_str()) {
                Some(chunk) => {
                    println!("Hidden Message: {}", chunk.data_as_string()?)
                }
                None => println!("Chunk type {} not found", args.chunk_type),
            }
        }
        cli::Commands::Remove(args) => {
            let mut png = png::Png::from_file(args.file_path.clone())?;
            let removed_chunck = png.remove_chunk(args.chunk_type.to_string().as_str())?;
            println!(
                "chuck with type: {:?} is removed",
                removed_chunck.chunk_type().to_string()
            );
            let _ = fs::write(args.file_path, png.as_bytes());
        }
        cli::Commands::Print(args) => {
            let png = png::Png::from_file(args.file_path)?;
            png.chunks().iter().for_each(|chunk| println!("{}", chunk))
        }
    }
    Ok(())
}
