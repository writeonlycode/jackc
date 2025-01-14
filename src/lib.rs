use anyhow::Result;
use clap::Parser;
use compiler::Compiler;
use std::{fs::File, io::BufReader};
use tokenizer::Tokenizer;
use walkdir::WalkDir;

mod compiler;
mod tokenizer;

#[derive(Parser, Debug)]
pub struct Config {
    #[arg(help = "Input file or directory.")]
    file_path: String,
}

pub fn run(config: Config) -> Result<()> {
    for entry in WalkDir::new(config.file_path) {
        let entry = entry?;

        if let Some(extension) = entry.path().extension() {
            if extension == "jack" {
                let in_file_path = entry.path().to_string_lossy().to_string();
                let in_file = File::open(in_file_path)?;
                let mut buf_reader = BufReader::new(in_file);

                let mut tokenizer = Tokenizer::new(&mut buf_reader).peekable();

                let out_file_path = entry.path().with_extension("xml");
                let mut out_file = File::create(out_file_path)?;

                let mut compiler = Compiler::new(&mut tokenizer, &mut out_file);
                compiler.compile()?;
            }
        }
    }

    return Ok(());
}
