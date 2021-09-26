use rand::prelude::ThreadRng;
use rand::RngCore;

pub fn fill_buffer(buffer: &mut Vec<u8>, rng: &mut ThreadRng, randomize: bool, value: u8) {
    if randomize {
        rng.fill_bytes(buffer.as_mut_slice());
    } else {
        buffer.iter_mut().map(|x| *x = value).count();
    }
}

pub fn print_write_value(
    total_steps: u32,
    actual_step: u32,
    write_zeroes: bool,
    file_path: &str,
    randomize: bool,
    value: u8,
) {
    let total_steps_computed = match write_zeroes {
        true => total_steps,
        false => total_steps + 1,
    };
    let actual_value = match randomize {
        true => String::from("random"),
        false => format!("{:x?}", value),
    };
    println!(
        "{file:?}: step{time}/{times} ({value}).",
        file = file_path,
        time = actual_step + 1,
        times = total_steps_computed,
        value = actual_value
    );
}
