use std::fs;
use std::fs::File;
use std::io::{Seek, SeekFrom, Write};
use std::path::{Path, MAIN_SEPARATOR};

pub fn get_size_to_write(size_present: bool, size_str: &str, file_size: u64) -> u64 {
    if size_present {
        parse_size(size_str)
    } else {
        file_size
    }
}

pub fn remove_file(verbose: bool, path: &Path) {
    let canonical = path
        .canonicalize()
        .expect("error getting the file directory");

    let dir = canonical.parent().unwrap();

    let filename = path.file_name().unwrap_or_else(|| {
        panic!(
            "error getting the file name to remove from {:?}",
            path.to_str()
        )
    });
    let mut new_filename = (0..filename.len()).map(|_| "0").collect::<String>();
    let mut old_filepath = String::from(
        canonical
            .clone()
            .as_os_str()
            .to_str()
            .expect("error getting the canonical path form file"),
    );
    let mut new_filepath: String = String::new();
    while !new_filename.is_empty() {
        new_filepath.clear();
        new_filepath.push_str(
            &*(dir.as_os_str().to_str().unwrap().to_owned()
                + MAIN_SEPARATOR.to_string().as_str()
                + new_filename.as_str()),
        );
        fs::rename(
            Path::new(old_filepath.clone().as_str()),
            Path::new(&new_filepath),
        )
        .expect("error renaming the file");
        if verbose {
            println!("{} renamed to {}", old_filepath, new_filepath);
        }
        old_filepath = new_filepath.clone();
        new_filename.pop();
    }
    fs::remove_file(Path::new(&new_filepath))
        .unwrap_or_else(|err| panic!("error deleting the file {}: {:?}", new_filepath, err.kind()));
    if verbose {
        println!("{} removed", new_filepath);
    }
}

pub fn write_buffer(file: &mut File, buf: &mut Vec<u8>) {
    file.seek(SeekFrom::Start(0))
        .map_err(|err| println!("{:?}", err))
        .ok();
    file.write_all(buf.as_slice()).unwrap_or_else(|err| {
        panic!("! {:?}", err);
    });
}

fn parse_size(size_str: &str) -> u64 {
    let last_char: &str = size_str[size_str.len() - 1..size_str.len()].as_ref();
    let mut size = size_str[0..size_str.len() - 1]
        .parse::<u64>()
        .unwrap_or_else(|err| {
            panic!("! {:?}", err);
        });
    let mut multiplier: u64 = 1;
    if last_char.parse::<u64>().is_err() {
        if "k".eq_ignore_ascii_case(last_char) {
            multiplier = 1024;
        } else if "m".eq_ignore_ascii_case(last_char) {
            multiplier = 1024 * 1024;
        } else if "m".eq_ignore_ascii_case(last_char) {
            multiplier = 1024 * 1024 * 1024;
        } else {
            panic!("Cannot parse the size value {:?}", size_str);
        }
    } else {
        size = size * 10 + last_char.parse::<u64>().expect("error parsing size")
    }
    size * multiplier
}
