#[macro_use]
mod utils;

use crate::utils::read_lines;
use anyhow::{ensure, Context, Result};
use clap::Parser;
use rand::{thread_rng, Rng};
use std::{fs, path::Path};
use utils::file_exists;

#[derive(Parser)]
#[command(
    about = "Clones a text file, removing words at random from each line",
    author = "SirGolem",
    version
)]
struct Args {
    /// Path to the source file to process
    file: String,
    /// Path to location of output file
    out: String,

    #[arg(
        short,
        long,
        default_value_t = 1,
        help = "The number of words to remove per line (negative values exclude a certain number of words, removing the rest)"
    )]
    words: isize,
    #[arg(
        short,
        long,
        default_value_t = String::from("_"),
        help = "String to replace each letter with"
    )]
    replacement: String,

    #[arg(short, long, default_value_t = false, help = "Enable verbose logging")]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    match process_file(&args).with_context(|| "error processing file") {
        Ok(_) => (),
        Err(error) => {
            if args.verbose {
                printerror!("{:?}", error);
            } else {
                match error.source() {
                    Some(source) => printerror!("{}: {}", error, source),
                    None => printerror!("{}", error),
                }
            }
        }
    }
}

fn process_file(args: &Args) -> Result<()> {
    let input_path = Path::new(&args.file);
    ensure!(
        file_exists(input_path).with_context(|| "Failed to check existence of input file")?,
        "Input file does not exist"
    );

    let input_lines = read_lines(input_path).with_context(|| "Failed to read lines from file")?;
    let mut output_lines = String::new();
    let mut rng = thread_rng();

    for line in input_lines.map_while(Result::ok) {
        let mut words: Vec<String> = line.split(' ').map(|string| string.to_string()).collect();
        let mut remaining_words = words.clone();
        remaining_words.retain(|string| !string.trim().is_empty());
        let clamped_removals = args
            .words
            .clamp(-(words.len() as isize), words.len() as isize);
        let removals: usize = if clamped_removals < 0 {
            ((words.len() as isize) + clamped_removals) as usize
        } else {
            clamped_removals as usize
        };

        for _ in 0..removals {
            let word_to_remove = rng.gen_range(0..remaining_words.len());
            let index_to_remove_option = words
                .iter()
                .position(|item| *item == remaining_words[word_to_remove]);
            match index_to_remove_option {
                Some(index) => {
                    words[index] = args.replacement.repeat(words[index].len());
                    remaining_words.remove(word_to_remove);
                }
                None => {
                    if args.verbose {
                        printwarning!("failed to find word index");
                    }
                }
            }
        }

        if !output_lines.is_empty() {
            output_lines += "\n";
        }
        output_lines += &words.join(" ");
    }

    let output_path = Path::new(&args.out);
    if let Some(parent_path) = output_path.parent() {
        if !parent_path.is_dir() {
            fs::create_dir_all(parent_path)
                .with_context(|| "Failed to create parent directories")?;
        }
    }
    fs::write(output_path, output_lines).with_context(|| "Failed to write output file")?;

    Ok(())
}
