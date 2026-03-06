use std::{
    fs::File,
    io::{self, BufWriter},
    path::PathBuf,
};

use anyhow::Context;
use clap::Parser;
use common::{AsyncSeek, AsyncWrite, SyncIo, copy, oneshot_async};
use io::BufReader;
use xp3::{header::XP3Version, write::XP3Writer};

#[derive(Parser)]
struct Args {
    /// Directory for archive files
    input_dir: PathBuf,
    /// Output XP3 path
    out_xp3: PathBuf,
    /// Archive compression level from 0 to 9
    #[arg(short, long)]
    compression: Option<u8>,
}

fn main() {
    if let Err(err) = run(Args::parse()) {
        eprintln!("Error: {err:?}");
    }
}

fn run(args: Args) -> anyhow::Result<()> {
    let out = File::create(&args.out_xp3)
        .with_context(|| format!("failed to open {} for read", args.out_xp3.display()))?;

    let mut writer = oneshot_async(XP3Writer::new(
        XP3Version::Current { minor: 1 },
        SyncIo(BufWriter::new(out)),
    ))
    .context("failed to write out_xp3")?;

    add_all_file(&mut writer, &args.input_dir, &args.input_dir, args.compression)
        .context("failed to add input files")?;

    oneshot_async(writer.finish(args.compression)).context("failed to finish file")?;
    Ok(())
}

fn add_all_file<T: AsyncWrite + AsyncSeek + Unpin>(
    writer: &mut XP3Writer<T>,
    root: &PathBuf,
    dir_path: &PathBuf,
    compression: Option<u8>,
) -> anyhow::Result<()> {
    let dir = std::fs::read_dir(dir_path).context("failed to read directory")?;

    for entry in dir {
        let entry = entry?;

        let path = entry.path();
        let path_str = path.to_string_lossy();

        let relative_path = path.strip_prefix(root)?;

        if path.is_dir() {
            let res = add_all_file(writer, root, &path, compression);
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

            let mut entry = oneshot_async(writer.file(
                relative_path.display().to_string(),
                false,
                compression,
            ))?;
            entry.timestamp(Some(time));
            oneshot_async(copy(&mut SyncIo(BufReader::new(file)), &mut entry))?;
            oneshot_async(entry.finish())?;
        }
    }

    Ok(())
}
