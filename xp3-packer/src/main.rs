/*
 * Created on Wed Dec 16 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use std::{env, fs::File, io::{self, BufWriter, Read, Seek, Write}, path::PathBuf};

use io::BufReader;
use xp3::{header::XP3HeaderVersion, index::file::{IndexInfoFlag, IndexSegmentFlag}, index_set::XP3IndexCompression, writer::XP3Writer};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("usage: {} <input_dir> <out_xp3>", args[0]);
        return;
    }

    let out = File::create(&args[2]).expect(&format!("Cannot open {} for read", &args[1]));

    let mut writer = XP3Writer::start(
        BufWriter::new(out),
        XP3HeaderVersion::Current {
            minor_version: 1,
            index_size_offset: 0
        },
        XP3IndexCompression::Compressed
    ).expect("Cannot write out_xp3");

    add_all_file(&mut writer, &PathBuf::new().join(&args[1]), &PathBuf::new().join(&args[1])).expect("Cannot add files");
    writer.finish().expect("Cannot finish file");

}

fn add_all_file<T: Write + Seek>(writer: &mut XP3Writer<T>, root: &PathBuf, dir_path: &PathBuf) -> io::Result<()> {
    let dir = std::fs::read_dir(dir_path)?;

    for entry in dir {
        let entry = entry?;

        let path = entry.path();
        let path_str = path.to_string_lossy();

        let relative_path = path.strip_prefix(root).unwrap().to_string_lossy().to_string();

        if path.is_dir() {
            let res = add_all_file(writer, root, &path);
            if res.is_err() {
                println!("Skipping unreadable directory: {}. Err: {:?}", path_str, res.err());
                continue;
            }
        } else {
            let file = File::open(&path);
            println!("Packing {} ", relative_path);

            if file.is_err() {
                println!("Skipping unreadable file: {} Err: {:?}", path_str, file.err());
                continue;
            }
            let file = file.unwrap();

            let time = path.metadata().unwrap().modified().unwrap().elapsed().unwrap().as_millis() as u64;
    
            let mut buffer = Vec::new();
            let mut reader = BufReader::new(file);

            reader.read_to_end(&mut buffer)?;

            let mut entry = writer.enter_file(IndexInfoFlag::NotProtected, relative_path.clone().replace("\\", &"/"), Some(time));
            entry.write_segment(
                IndexSegmentFlag::UnCompressed,
                &mut buffer
            )?;
            entry.finish();
        }
    }

    Ok(())
}