use std::fs::OpenOptions;
use std::path::Path;

use clap::{App, Arg};

use rand::Rng;

use shred::files;

mod utils;

fn main() {
    let matches = App::new("shred")
        .version("1.0.0")
        .arg(
            Arg::with_name("times")
                .short("n")
                .long("iterations")
                .takes_value(true)
                .help("Overwrite N times instead of the default (3)."),
        )
        .arg(
            Arg::with_name("remove")
                .short("u")
                .long("remove")
                .takes_value(false)
                .help("Truncate and remove file after overwriting."),
        )
        .arg(
            Arg::with_name("zero")
                .short("z")
                .long("zero")
                .takes_value(false)
                .help("Add a final overwrite with zeros to hide shredding."),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .takes_value(false)
                .help("Show verbose information about shredding progress."),
        )
        .arg(
            Arg::with_name("size")
                .short("s")
                .long("size")
                .takes_value(true)
                .help("Shred this many bytes (suffixes like K, M, G accepted)."),
        )
        .arg(Arg::from_usage(
            " <FILE>              'Sets the file to use'",
        ))
        .get_matches();
    let times = matches
        .value_of("times")
        .unwrap_or("3")
        .parse::<i32>()
        .unwrap_or_else(|err| {
            panic!("! {:?}", err);
        }) as u32;
    let remove = matches.is_present("remove");
    let write_zeroes = matches.is_present("zero");
    let verbose = matches.is_present("verbose");
    let size_present = matches.is_present("size");
    let file_path_arg = matches.value_of("FILE");
    let file_path =
        file_path_arg.unwrap_or_else(|| panic!("Cannot parse file! {:?}", file_path_arg));
    let path = Path::new(file_path);
    let mut file = OpenOptions::new()
        .write(true)
        .open(path)
        .unwrap_or_else(|err| {
            panic!(
                "{:?} -> {:?}",
                path.canonicalize().unwrap().to_str().unwrap(),
                err
            );
        });
    let file_metadata = file.metadata().unwrap_or_else(|err| {
        panic!("! {:?}", err.kind());
    });
    if !file_metadata.is_file() || file_metadata.permissions().readonly() {
        panic!("cannot write to {:?}", file_path);
    }
    let size_to_write = files::get_size_to_write(
        size_present,
        matches.value_of("size").unwrap_or("0"),
        file_metadata.len(),
    );
    let mut buf = vec![0; size_to_write as usize];
    let mut rng = rand::thread_rng();
    for i in 0..times {
        let randomize = (i + 1) % 3 == 0;
        let value: u8 = rng.gen();
        if verbose {
            utils::print_write_value(times, i, write_zeroes, file_path, randomize, value)
        }
        utils::fill_buffer(&mut buf, &mut rng, randomize, value);
        files::write_buffer(&mut file, &mut buf);
    }
    if write_zeroes {
        let value: u8 = 0;
        if verbose {
            utils::print_write_value(times, times, false, file_path, false, value);
        }
        utils::fill_buffer(&mut buf, &mut rng, false, value);
        files::write_buffer(&mut file, &mut buf);
    }
    if remove {
        files::remove_file(verbose, path)
    }
}
