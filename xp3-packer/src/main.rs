use std::{
    fs::File,
    io::{self, BufWriter, Read, Seek, Write},
    path::PathBuf,
};

use anyhow::Context;
use clap::Parser;
use io::BufReader;
use xp3::{
    header::XP3HeaderVersion,
    index::file::{IndexInfoFlag, IndexSegmentFlag},
    index_set::XP3IndexCompression,
    writer::XP3Writer,
};

#[derive(Parser)]
struct Args {
    input_dir: PathBuf,
    out_xp3: PathBuf,
}

fn main() {
    if let Err(err) = run(Args::parse()) {
        eprintln!("Error: {err:?}");
    }
}

fn run(args: Args) -> anyhow::Result<()> {
    let out = File::create(&args.out_xp3)
        .with_context(|| format!("failed to open {} for read", args.out_xp3.display()))?;

    let mut writer = XP3Writer::start(
        BufWriter::new(out),
        XP3HeaderVersion::Current {
            minor_version: 1,
            index_size_offset: 0,
        },
        XP3IndexCompression::Compressed,
    )
    .expect("failed to write out_xp3");

    add_all_file(&mut writer, &args.input_dir, &args.input_dir)
        .context("failed to add input files")?;

    writer.finish().expect("failed to finish file");

    Ok(())
}

fn add_all_file<T: Write + Seek>(
    writer: &mut XP3Writer<T>,
    root: &PathBuf,
    dir_path: &PathBuf,
) -> anyhow::Result<()> {
    let dir = std::fs::read_dir(dir_path).context("failed to read directory")?;

    for entry in dir {
        let entry = entry?;

        let path = entry.path();
        let path_str = path.to_string_lossy();

        let relative_path = path.strip_prefix(root)?;

        if path.is_dir() {
            let res = add_all_file(writer, root, &path);
            if res.is_err() {
                println!(
                    "Skipping unreadable directory: {}. Err: {:?}",
                    path_str,
                    res.err()
                );
                continue;
            }
        } else {
            println!("Packing {} ", relative_path.display());
            let file = match File::open(&path) {
                Ok(file) => file,
                Err(err) => {
                    println!("Skipping unreadable file: {} Err: {:?}", path_str, err);
                    continue;
                }
            };
            let time = path.metadata()?.modified()?.elapsed()?.as_millis() as u64;

            let mut buffer = Vec::new();
            let mut reader = BufReader::new(file);

            reader
                .read_to_end(&mut buffer)
                .with_context(|| format!("failed to read content of {}", path.display()))?;

            let mut entry = writer.enter_file(
                IndexInfoFlag::NotProtected,
                relative_path.display().to_string(),
                Some(time),
            );
            entry
                .write_segment(IndexSegmentFlag::UnCompressed, &buffer)
                .context("failed writing xp3 segments")?;
            entry.finish();
        }
    }

    Ok(())
}
