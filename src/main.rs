use clap::{App, Arg};
use rand::Rng;
use shred::files;
use shred::threadpool::ThreadPool;
use std::fs::OpenOptions;
use std::path::Path;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::sync::Arc;

mod utils;

fn main() {
    let matches = App::new("shred")
        .version("1.0.0")
        .arg(
            Arg::with_name("times")
                .short("n")
                .long("iterations")
                .takes_value(true)
                .help("Overwrite N times.")
                .default_value("3"),
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
            Arg::with_name("recursive")
                .short("r")
                .long("recursive")
                .takes_value(false)
                .help("Recursively deletes the files in directories"),
        )
        .arg(
            Arg::with_name("size")
                .short("s")
                .long("size")
                .takes_value(true)
                .help("Shred this many bytes (suffixes like K, M, G accepted)."),
        )
        .arg(
            Arg::with_name("threads")
                .short("t")
                .long("threads")
                .takes_value(true)
                .default_value("1")
                .help("Number of threads to execute in parallel"),
        )
        .arg(
            Arg::with_name("FILES")
                .help("Sets the files to to shred")
                .required(true)
                .multiple(true),
        )
        .get_matches();
    let times = matches
        .value_of("times")
        .map(|s| s.parse::<u32>())
        .unwrap()
        .unwrap();
    let remove = matches.is_present("remove");
    let write_zeroes = matches.is_present("zero");
    let verbose = matches.is_present("verbose");
    let recursive = matches.is_present("recursive");
    let size_str = Arc::new(String::from(
        matches
            .value_of("size")
            .unwrap_or("-1"),
    ));
    let threads = matches
        .value_of("threads")
        .map(|s| s.parse::<usize>())
        .unwrap()
        .unwrap();
    if let Some(file_paths) = matches.values_of_lossy("FILES") {
        let thread_pool: ThreadPool = ThreadPool::new(threads);
        let (explorer_sender, explorer_receiver) = mpsc::channel();
        for file_path in file_paths {
            let sender = explorer_sender.clone();
            thread_pool.execute(move || {
                explore(file_path.as_str(), recursive, sender);
            });
        }
        drop(explorer_sender);
        for file in explorer_receiver {
            let size_str = Arc::clone(&size_str);
            thread_pool.execute(move || {
                shred(
                    file.as_str(),
                    times,
                    remove,
                    write_zeroes,
                    verbose,
                    size_str.as_str(),
                );
            });
        }
    }
}

fn explore(file_path: &str, recursive: bool, sender: Sender<String>) {
    let path = Path::new(file_path);
    let file_metadata = path.metadata().unwrap_or_else(|err| {
        panic!("{}! {:?}", file_path, err.kind());
    });
    let path_str = path.to_str().unwrap();
    if file_metadata.is_dir() && recursive {
        for entry in path.read_dir().unwrap() {
            explore(
                entry.unwrap().path().to_str().unwrap(),
                recursive,
                sender.clone(),
            );
        }
    } else if file_metadata.is_file() {
        sender.send(String::from(path_str)).unwrap()
    }
}

/// rewrites and optionally deletes the file pointed by `file_path`.
///
/// # Arguments
///
/// * `file_path`: the path of the file to rewrite.
/// * `times`: the number of times to rewrite the file.
/// * `remove`: the flag for deleting the file after the rewrites.
/// * `write_zeroes`: flag for writing one more time after `times` withe zeroes.
/// * `verbose`: flag for printing verbose messages on console.
/// * `recursive`: flag for doing the process recursively (only for directories).
/// * `size_str`: the size to write in str or "-1" for writing the entire file.
///
/// returns: ()
///
/// # Examples
///
/// ```
///
/// ```
fn shred(
    file_path: &str,
    times: u32,
    remove: bool,
    write_zeroes: bool,
    verbose: bool,
    size_str: &str,
) {
    let size_present = !"-1".eq_ignore_ascii_case(size_str);
    let path = Path::new(file_path);
    let file_metadata = path.metadata().unwrap_or_else(|err| {
        panic!("{}! {:?}", file_path, err.kind());
    });
    if file_metadata.is_dir() {
        panic!("cannot shred directories without the recursive parameter. See help");
    }

    if file_metadata.is_file() {
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
        if file_metadata.permissions().readonly() {
            panic!("cannot write to {:?}", file_path);
        };
        let size_to_write = files::get_size_to_write(size_present, size_str, file_metadata.len());
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
    } else {
        panic!("cannot find the type of the file {:?}", path.to_str())
    }
    if remove {
        files::remove_file(verbose, path)
    }
}
