use std::env;
use std::fs::{self, File};
use std::io::{BufReader, Read, Write};
use std::path::Path;

use coverage_formats::go::GoReport;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let input_dir = args
        .next()
        .expect("usage: merge_folder <input_dir> <output_file>");
    let output_file = args
        .next()
        .expect("usage: merge_folder <input_dir> <output_file>");

    let input_dir = Path::new(&input_dir);
    if !input_dir.is_dir() {
        return Err(format!("input path is not a directory: {}", input_dir.display()).into());
    }

    let mut merged: Option<GoReport> = None;

    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let mut file = File::open(&path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        if !contents.is_empty() {
            let report = GoReport::from_buf_read(&mut BufReader::new(&mut contents.as_bytes()))?;
            merged = match merged {
                None => Some(report),
                Some(prev) => Some(prev.try_merge(report)?),
            };
        }
    }

    let merged = match merged {
        Some(r) => r,
        None => return Err("no coverage files found".into()),
    };

    // Serialize merged report back to text (implement format_write)
    let mut out = File::create(&output_file)?;
    out.write_all(format!("{merged}").as_bytes())?;

    println!("Merged coverage written to {output_file}");
    Ok(())
}
