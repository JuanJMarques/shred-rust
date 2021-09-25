use std::fs;
use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom, Write};
use std::path::{Path, MAIN_SEPARATOR};

use clap::{App, Arg};
use rand::{Rng, RngCore};

fn main() {
    let matches = App::new("shred")
        .version("1.0.0")
        .arg(Arg::with_name("times")
            .short("n")
            .long("iterations")
            .takes_value(true)
            .help("Overwrite N times instead of the default (3)."))
        .arg(Arg::with_name("remove")
            .short("u")
            .long("remove")
            .takes_value(false)
            .help("Truncate and remove file after overwriting."))
        .arg(Arg::with_name("zero")
            .short("z")
            .long("zero")
            .takes_value(false)
            .help("Add a final overwrite with zeros to hide shredding."))
        .arg(Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .takes_value(false)
            .help("Show verbose information about shredding progress."))
        .arg(Arg::with_name("size")
            .short("s")
            .long("size")
            .takes_value(true)
            .help("Shred this many bytes (suffixes like K, M, G accepted)."))
        .arg(Arg::from_usage(" <FILE>              'Sets the file to use'"))
        .get_matches();
    let times = matches.value_of("times").unwrap_or("3").parse::<i32>()
        .unwrap_or_else(|err| {
            panic!("! {:?}", err);
        }) as u32;
    let remove = matches.is_present("remove");
    let write_zeroes = matches.is_present("zero");
    let verbose = matches.is_present("verbose");
    let size_present = matches.is_present("size");
    let file_path_arg = matches.value_of("FILE");
    let file_path = file_path_arg.unwrap_or_else(|| {
        panic!("Cannot parse file! {:?}", file_path_arg)
    });
    let path = Path::new(file_path);
    let mut file = OpenOptions::new()
        .write(true)
        .open(path).unwrap_or_else(|err| {
        panic!("{:?} -> {:?}", path.canonicalize().unwrap().to_str().unwrap(), err);
    });
    let file_metadata = file.metadata().unwrap_or_else(|err| {
        panic!("! {:?}", err.kind());
    });
    if !file_metadata.is_file() || file_metadata.permissions().readonly() {
        panic!("cannot write to {:?}", file_path);
    }
    let size_to_write: u64;
    if size_present {
        size_to_write = parse_size(matches.value_of("size").unwrap());
    } else {
        size_to_write = file_metadata.len();
    }
    let mut buf = vec![0; size_to_write as usize];
    let mut rng = rand::thread_rng();
    for i in 0..times {
        let randomize = (i + 1) % 3 == 0;
        let value: u8 = rng.gen();
        if verbose {
            let mut times_to_write = times;
            if write_zeroes {
                times_to_write += 1;
            }
            if randomize {
                println!("{file:?}: step{time}/{times} (random).",
                         file = file_path, time = i + 1,
                         times = times_to_write);
            } else {
                println!("{file:?}: step{time}/{times} ({value}).",
                         file = file_path, time = i + 1,
                         times = times_to_write, value = value);
            }
        }
        if randomize {
            rng.fill_bytes(buf.as_mut_slice());
        } else {
            buf.iter_mut().map(|x| *x = value).count();
        }
        file.seek(SeekFrom::Start(0)).map_err(|err| println!("{:?}", err)).ok();
        file.write(buf.as_slice()).unwrap_or_else(|err| {
            panic!("! {:?}", err);
        });
    }
    if write_zeroes {
        let value: u8 = 0;
        if verbose {
            println!("{file:?}: step{time}/{times} ({value}).",
                     file = file_path,
                     time = times + 1,
                     times = times + 1,
                     value = value);
        }
        buf.iter_mut().map(|x| *x = value).count();
        file.seek(SeekFrom::Start(0)).map_err(|err| println!("{:?}", err)).ok();
        file.write(buf.as_slice()).unwrap_or_else(|err| {
            panic!("! {:?}", err.kind());
        });
    }

    if remove {
        let canonical = path.canonicalize()
            .expect("error getting the file directory");

        let dir = canonical.parent().unwrap();

        let filename = path.file_name()
            .expect(format!("error getting the file name to remove from {:?}",
                            path.to_str())
                .as_str());
        let mut new_filename = (0..filename.len()).map(|_| "0").collect::<String>();
        let mut old_filepath = String::from(canonical.clone().as_os_str().to_str()
            .expect("error getting the canonical path form file"));
        let mut new_filepath: String = String::new();
        while new_filename.len() > 0 {
            new_filepath.clear();
            new_filepath.push_str(&*(dir.as_os_str().to_str().unwrap().to_owned()
                + MAIN_SEPARATOR.to_string().as_str()
                + new_filename.as_str()));
            fs::rename(Path::new(old_filepath.clone().as_str()), Path::new(&new_filepath))
                .expect("error renaming the file");
            if verbose {
                println!("{} renamed to {}", old_filepath, new_filepath);
            }
            old_filepath = new_filepath.clone();
            new_filename.pop();
        }
        fs::remove_file(Path::new(&new_filepath))
            .expect(format!("error deleting the file {}", new_filepath).as_str());
        if verbose {
            println!("{} removed", new_filepath);
        }
    }
}

fn parse_size(size_str: &str) -> u64 {
    let last_char: &str = size_str[size_str.len() - 1..size_str.len()].as_ref();
    let mut size = size_str[0..size_str.len() - 1].parse::<u64>()
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

