use std::fs;
use std::fs::File;
use std::io::{Seek, SeekFrom, Write};
use std::path::{Path, MAIN_SEPARATOR};

/// Returns the file described in `size_str` or in the `file_size` depending on the `size_present`
/// flag.
///
/// # Arguments
///
/// * `size_present`: the flag indicating whether parse the size in the `file_str` arg or
/// returning the `file_size` arg.
/// * `size_str`:  the size to parse with or without a size suffix (k, m, g).
/// * `file_size`: the file size in u64.
///
/// returns: the file size parsed
///
/// # Examples
///
/// ```
/// ```
pub fn get_size_to_write(size_present: bool, size_str: &str, file_size: u64) -> u64 {
    if size_present {
        parse_size(size_str)
    } else {
        file_size
    }
}

/// Deletes the files pointed by `path` argument by renaming it several times and then deleting it.
///
/// # Arguments
///
/// * `verbose`: flag for printing in console the steps.
/// * `path`:  the path of the file to delete.
///
/// returns: ()
///
/// # Examples
///
/// ```
/// ```
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

/// writes the content of `buf`param in the `file` starting at the beginning of the file.
///
/// # Arguments
///
/// * `file`: `the file to write.
/// * `buf`:  `the data to write`
///
/// returns: ()
///
/// # Examples
///
/// ```
/// ```
pub fn write_buffer(file: &mut File, buf: &mut Vec<u8>) {
    file.seek(SeekFrom::Start(0))
        .map_err(|err| println!("{:?}", err))
        .ok();
    file.write_all(buf.as_slice()).unwrap_or_else(|err| {
        panic!("! {:?}", err);
    });
}

/// Parses the size writen in `size_str` and returns it as a u64.
///
/// # Arguments
///
/// * `size_str`: the size with the following regex "^\d+\[k|m|g|K|M|G\]*$"<br/>
/// for example:
/// - 524
/// - 457k
/// - 1247M
/// - 9246789g
///
/// returns: u64 The parsed size of the string.
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
        } else if "g".eq_ignore_ascii_case(last_char) {
            multiplier = 1024 * 1024 * 1024;
        } else {
            panic!("Cannot parse the size value {:?}", size_str);
        }
    } else {
        size = size * 10 + last_char.parse::<u64>().expect("error parsing size")
    }
    size * multiplier
}

#[cfg(test)]
mod tests {
    use crate::files::{get_size_to_write, parse_size, remove_file, write_buffer};
    use std::borrow::BorrowMut;
    use std::io::{Read, Seek, SeekFrom};
    use std::panic;
    use tempfile::NamedTempFile;

    #[test]
    fn test_parse_size() {
        assert_eq!(126 as u64, parse_size("126"));
        assert_eq!(1234 * 1024 as u64, parse_size("1234K"));
        assert_eq!(1234 * 1024 as u64, parse_size("1234k"));
        assert_eq!(846 * 1024 * 1024 as u64, parse_size("846M"));
        assert_eq!(846 * 1024 * 1024 as u64, parse_size("846m"));
        assert_eq!(54638 * 1024 * 1024 * 1024 as u64, parse_size("54638G"));
        assert_eq!(54638 * 1024 * 1024 * 1024 as u64, parse_size("54638g"));
        assert_eq!(12, get_size_to_write(false, "684", 12 as u64));
        assert_eq!(684, get_size_to_write(true, "684", 12 as u64));
    }

    #[test]
    fn test_write_files() {
        let mut temp_file = NamedTempFile::new().unwrap_or_else(|err| {
            panic!("!cannot create tempFile {:?}", err.kind());
        });
        let file = temp_file.as_file_mut();
        let size = 512 as usize;
        let mut write_buf = vec![5 as u8; size];
        write_buffer(file, &mut write_buf);
        file.seek(SeekFrom::Start(0))
            .unwrap_or_else(|err| panic!("{:?}", err));
        let mut read_buff = vec![0 as u8; size];
        let bytes_read = file.read(read_buff.borrow_mut()).unwrap();
        assert_eq!(size, bytes_read);
        assert_eq!(write_buf, read_buff);
        temp_file.into_temp_path().close().unwrap_or_else(|err| {
            format!("cannot delete temporary file cause: {:?}", err);
        });
    }

    #[test]
    fn test_delete_files() {
        let temp_file = NamedTempFile::new().unwrap_or_else(|err| {
            panic!("!cannot create tempFile {:?}", err.kind());
        });
        let mut temp_path = temp_file.into_temp_path();
        let path = temp_path.borrow_mut();
        remove_file(false, path);
        let delete_result = temp_path.close();
        assert!(delete_result.is_err());
    }
}
