/*
 * Created on Thu Dec 17 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use std::{env, fs::{self, File}, io::{BufReader, BufWriter}, path::Path};

use xp3::reader::XP3Reader;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("usage: {} <input_xp3> <out_dir>", args[0]);
        return;
    }

    let input_xp3 = File::open(&args[1]).expect(&format!("Cannot open {} for read", &args[1]));
    let out_dir = args[2].clone();
    let archive = XP3Reader::open_archive(BufReader::new(input_xp3)).expect("Input file is invalid");

    for (name, _) in archive.entries() {
        println!("Extracting: {}", name);

        if name.len() > 128 {
            println!("Skipping {} . File name is too long.", name);
            continue;
        }
        
        let path_str = format!("{}/{}", out_dir, name);
        let path = Path::new(&path_str);
        fs::create_dir_all(path.parent().unwrap()).unwrap();

        archive.unpack(&name.into(), &mut BufWriter::new(File::create(path).unwrap())).unwrap();
    }


}
