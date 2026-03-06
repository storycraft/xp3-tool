use std::{
    fs::{self, File},
    io::{BufReader, BufWriter},
    path::PathBuf,
};

use anyhow::Context;
use clap::Parser;
use common::{SyncIo, copy, oneshot_async};
use xp3::read::XP3Archive;

#[derive(Parser)]
struct Args {
    /// Input XP3 archive path
    input_xp3: PathBuf,
    /// Output path for archive files
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
    let mut archive = oneshot_async(XP3Archive::open(SyncIo(BufReader::new(input_xp3))))
        .context("invalid input file")?;

    for i in 0..archive.entries().len() {
        let entry = &archive.entries()[i];
        println!("Extracting: {}", entry.name);

        if entry.name.len() > 260 {
            println!("Skipping {} . File name is too long.", entry.name);
            continue;
        }

        let mut path = args.out_dir.clone();
        for comps in entry.name.split('\\') {
            if comps.is_empty() {
                continue;
            }
            path.push(comps);
        }
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }

        let mut stream =
            SyncIo(BufWriter::new(File::create(&path).with_context(|| {
                format!("failed to create output file: {}", path.display())
            })?));

        let mut file = oneshot_async(archive.by_index(i))
            .unwrap()
            .context("failed to open file")?;
        oneshot_async(copy(&mut file, &mut stream)).context("failed to unpack file")?;
    }

    Ok(())
}
