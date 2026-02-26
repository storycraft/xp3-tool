use std::{
    fs::{self, File},
    io::{BufReader, BufWriter},
    path::PathBuf,
};

use anyhow::Context;
use clap::Parser;
use xp3::reader::XP3Reader;

#[derive(Parser)]
struct Args {
    input_xp3: PathBuf,
    out_dir: PathBuf,
}

fn main() {
    if let Err(err) = run(Args::parse()) {
        eprintln!("Error: {err:?}");
    }
}

fn run(args: Args) -> anyhow::Result<()> {
    let input_xp3 = File::open(&args.input_xp3)
        .with_context(|| format!("cannot open {} for read", args.input_xp3.display()))?;
    let archive =
        XP3Reader::open_archive(BufReader::new(input_xp3)).expect("input file is invalid");

    for (name, _) in archive.entries() {
        println!("Extracting: {}", name);

        if name.len() > 128 {
            println!("Skipping {} . File name is too long.", name);
            continue;
        }

        let path = args.out_dir.join(name);
        if let Some(parent_dir) = path.parent() {
            fs::create_dir_all(parent_dir).context("failed to create parent dir for files")?;
        }

        let mut stream = BufWriter::new(
            File::create(&path)
                .with_context(|| format!("failed to create output file: {}", path.display()))?,
        );

        archive
            .unpack(name, &mut stream)
            .expect("failed to unpack file");
    }

    Ok(())
}
